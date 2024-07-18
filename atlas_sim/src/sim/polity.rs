use std::collections::BTreeSet;

use atlas_lib::{
    bevy::{
        ecs as bevy_ecs,
        prelude::*,
        render::{
            render_asset::RenderAssetUsages,
            render_resource::{Extent3d, TextureDimension, TextureFormat},
        },
        utils::{hashbrown::HashMap, petgraph::matrix_graph::Zero},
    },
    bevy_egui,
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::{sim::AtlasSimConfig, AtlasConfig},
    domain::{
        graphics::{color_to_u8, MapLogicData},
        map::{is_sea, MapDataLayer},
    },
    ui::{
        sidebar::{MakeUi, SidebarColor, SidebarControl, SidebarEnumDropdown, SidebarSlider},
        UiEditableEnum,
    },
};
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::sim::{check_tick, SimMapData};

use super::check_tick_annual;

/// Polity simulation.
pub struct PolityPlugin;

impl Plugin for PolityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_visuals).add_systems(
            FixedUpdate,
            (update_mapgrab, update_pops, update_jobs, update_resources)
                .chain()
                .run_if(check_tick),
        );
    }
}

/// Ownership status of a polity.
#[derive(Default, Clone, Copy)]
pub enum Ownership {
    /// This polity is independent and has no master.
    #[default]
    Independent,
    /// This polity has a master but keeps local autonomy.
    Autonomous(Entity),
    /// This polity has a master and no local ruler.
    Integrated(Entity),
    /// This polity is occupied by an external force.
    Occupied(Entity),
}

impl UiEditableEnum for Ownership {
    const LEN: usize = 4;

    fn self_as_index(&self) -> usize {
        match self {
            Ownership::Independent => 0,
            Ownership::Autonomous(_) => 1,
            Ownership::Integrated(_) => 2,
            Ownership::Occupied(_) => 3,
        }
    }

    fn index_as_self(&self, _idx: usize) -> Self {
        unreachable!()
    }

    fn index_to_str(idx: usize) -> &'static str {
        match idx {
            0 => "Independent",
            1 => "Autonomous",
            2 => "Integrated",
            3 => "Occupied",
            _ => unreachable!(),
        }
    }
}

/// A political entity that owns land and population.
#[derive(Component, Clone)]
pub struct Polity {
    /// Map tile indices that this polity owns.
    pub tiles: Vec<u32>,
    /// Map tile indices outside of the polity border.
    pub border_tiles: BTreeSet<u32>,
    /// Centroid of owned land, in map coords.
    pub centroid: Vec2,
    /// XYWH bounding box in map coordinates.
    pub xywh: [u32; 4],
    /// Visuals need to be updated due to color or shape changes.
    pub need_visual_update: bool,
    /// Ownership status.
    pub ownership: Ownership,
    /// Map color.
    pub color: Color,
    /// The desire to claim border tiles.
    pub land_claim_points: f32,
    /// Map of # of tiles owned in resource chunks.
    pub resource_chunks: HashMap<u32, u16>,
    /// Map of available deposits.
    pub deposits: HashMap<u32, f32>,
    /// Amount of supply points produced.
    pub supply: f32,
    /// Amount of industry points produced.
    pub industry: f32,
    /// Amount of wealth points produced.
    pub wealth: f32,
    /// Total polity population.
    pub population: f32,
    /// Job pop group.
    pub jobs: HashMap<u32, f32>,
}

impl Polity {
    pub fn into_ui(&self, config: &AtlasSimConfig) -> PolityUi {
        PolityUi {
            ownership: self.ownership,
            color: color_to_u8(&self.color),
            land_claim_points: self.land_claim_points,
            deposits: self
                .deposits
                .iter()
                .map(|(k, v)| (config.deposits.types[*k as usize].name.clone(), *v))
                .collect(),
            supply: self.supply,
            industry: self.industry,
            wealth: self.wealth,
            population: self.population,
            jobs: self
                .jobs
                .iter()
                .map(|(k, v)| (config.jobs.types[*k as usize].name.clone(), *v))
                .collect(),
        }
    }
}

#[derive(Component)]
pub struct PolityUi {
    /// Ownership status.
    pub ownership: Ownership,
    /// Polity map color.
    pub color: [u8; 3],
    /// The desire to claim border tiles.
    pub land_claim_points: f32,
    /// Map of available deposits.
    pub deposits: Vec<(String, f32)>,
    /// Amount of supply points produced.
    pub supply: f32,
    /// Amount of industry points produced.
    pub industry: f32,
    /// Amount of wealth points produced.
    pub wealth: f32,
    /// Total polity population.
    pub population: f32,
    /// List of pop job groups.
    pub jobs: Vec<(String, f32)>,
}

impl MakeUi for PolityUi {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarEnumDropdown::new(ui, "Ownership", &mut self.ownership).show(None);
        SidebarColor::new(ui, "Color", &mut self.color).show(None);
        SidebarSlider::new(ui, "Land Claim Points", &mut self.land_claim_points).show(None);
        ui.heading("Resources");
        ui.end_row();
        SidebarSlider::new(ui, "Supply", &mut self.supply).show(None);
        SidebarSlider::new(ui, "Industry", &mut self.industry).show(None);
        SidebarSlider::new(ui, "Wealth", &mut self.wealth).show(None);
        ui.heading("Population & Jobs");
        ui.end_row();
        SidebarSlider::new(ui, "Population", &mut self.population).show(None);
        for (k, v) in &mut self.jobs {
            SidebarSlider::new(ui, k.clone(), v).show(None);
        }
        ui.heading("Deposits");
        ui.end_row();
        for (k, v) in &mut self.deposits {
            SidebarSlider::new(ui, k.clone(), v).show(None);
        }
    }
}

impl Default for Polity {
    fn default() -> Self {
        Self {
            tiles: vec![],
            border_tiles: Default::default(),
            centroid: Vec2::ZERO,
            xywh: [0, 0, 1, 1],
            ownership: Ownership::Independent,
            color: Default::default(),
            need_visual_update: true,
            land_claim_points: 0.0,
            resource_chunks: Default::default(),
            deposits: Default::default(),
            population: 0.0,
            jobs: Default::default(),
            supply: 0.0,
            industry: 0.0,
            wealth: 0.0,
        }
    }
}

/// Update system
///
/// Claim map tiles.
fn update_mapgrab(
    config: Res<AtlasSimConfig>,
    mut query: Query<(Entity, &mut Polity)>,
    logics: Res<MapLogicData>,
    mut extras: ResMut<SimMapData>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let climate = logics.get_layer(MapDataLayer::Climate);
    let conts = logics.get_layer(MapDataLayer::Continents);
    for (entity, mut polity) in query.iter_mut() {
        // Only claim land when in the mood.
        if polity.land_claim_points <= config.rules.land_claim_cost {
            continue;
        }
        // Check border tiles for free land.
        let weights: Vec<f32> = polity
            .border_tiles
            .iter()
            .map(|i| {
                let i = *i as usize;
                match extras.tile_owner[i] {
                    Some(_) => 0.0,
                    None => {
                        if is_sea(conts[i]) {
                            0.0
                        } else {
                            config.get_biome(climate[i]).habitability
                        }
                    }
                }
            })
            .collect();
        // Don't bother if all land is taken or very bad.
        if weights.is_empty() || weights.iter().fold(0.0f32, |acc, x| acc.max(*x)) <= 0.1 {
            continue;
        }
        // Choose one of the tiles.
        let table = WalkerTableBuilder::new(&weights).build();
        let i = table.next_rng(rng.as_mut());
        // Add to polity.
        let i = *polity.border_tiles.iter().nth(i).unwrap();
        polity.claim_tile(i, Some(entity.clone()), &mut extras, &config);
    }
}

/// Update system
///
/// Grow/shrink population based on supply.
fn update_pops(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>) {
    for mut polity in query.iter_mut() {
        // How much % of the population is supplied and can survive.
        // Any surplus should boost growth beyond the base rate.
        let consumption = polity.get_supply_consumption(&config);
        let coverage = if consumption.is_zero() {
            1.0
        } else {
            (polity.supply / consumption).min(2.0)
        };
        let base_coverage = coverage.min(1.0);
        // Grow the population.
        polity.population =
            (polity.population * (1.0 * base_coverage + config.rules.pop_growth * coverage)).max(1.0);
    }
}

/// Update system
///
/// Update resources (supply/industry/wealth).
fn update_resources(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>) {
    for mut polity in query.iter_mut() {
        polity.update_resources(&config);
    }
}

/// Update system
///
/// Assign jobs.
fn update_jobs(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>) {
    for mut polity in query.iter_mut() {
        polity.update_jobs(&config);
    }
}

/// Update system
///
/// Update polity visuals.
fn update_visuals(
    config: Res<AtlasSimConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&mut Polity, &mut Transform, &mut Handle<StandardMaterial>), With<Polity>>,
) {
    for (mut polity, mut tran, mut mat) in query.iter_mut() {
        // Don't update if not needed.
        if !polity.need_visual_update {
            continue;
        }
        let (w, _) = config.get_world_size();
        let (x, y, width, height) = (polity.xywh[0], polity.xywh[1], polity.xywh[2], polity.xywh[3]);
        // Make new texture data.
        let (off, diff) = (w * y + x, w - width);
        let mut data = vec![0; width as usize * height as usize * 4];
        for i in &polity.tiles {
            let i = i - off;
            let i = ((i - diff * (i / w)) * 4) as usize;
            data[i] = 255;
            data[i + 1] = 255;
            data[i + 2] = 255;
            data[i + 3] = 255;
        }
        // Get world space origin and scale.
        let p = config.centroid_to_world_centered(polity.centroid.into());
        let s = (width as f32 / 100.0, height as f32 / 100.0);
        tran.translation = Vec3::new(p.0, p.1, 0.0);
        tran.scale = Vec3::new(s.0, s.1, s.1);
        // Update the material (with tint) and texture (with shape).
        *mat = materials.add(StandardMaterial {
            base_color: polity.color,
            base_color_texture: Some(images.add(Image::new(
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                data,
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::all(),
            ))),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });

        polity.need_visual_update = false;
    }
}

impl Polity {
    pub fn claim_tile(
        &mut self,
        tile: u32,
        entity: Option<Entity>,
        extras: &mut SimMapData,
        config: &AtlasSimConfig,
    ) {
        self.border_tiles.remove(&tile);
        extras.tile_owner[tile as usize] = entity;
        self.tiles.push(tile);
        // Update xywh.
        self.tiles.sort();
        let (width, _) = config.get_world_size();
        let (first, last) = (self.tiles[0], self.tiles[self.tiles.len() - 1]);
        let mut min = width;
        let mut max = 0;
        for t in &self.tiles {
            let v = t % width;
            min = std::cmp::min(min, v);
            max = std::cmp::max(max, v);
        }
        let (x, y, w) = (min, first / width, max - min + 1);
        let h = last / width + 1 - y;
        self.xywh = [x, y, w, h];
        // Recalculate centroid.
        self.centroid = Vec2::new(x as f32 + w as f32 / 2.0, y as f32 + h as f32 / 2.0);
        // Update polity borders.
        if let Some(entity) = entity {
            self.border_tiles.extend(
                &mut config
                    .get_border_tiles(tile)
                    .iter()
                    .filter(|x| !extras.tile_owner[**x as usize].is_some_and(|y| y.eq(&entity))),
            );
        } else {
            self.border_tiles
                .extend(&mut config.get_border_tiles(tile).iter());
        };
        // Update resource chunk coverage.
        let j = config.index_to_chunk(tile, config.deposits.chunk_size as u32);
        let coverage = if let Some(chunk) = self.resource_chunks.get_mut(&j) {
            *chunk += 1;
            *chunk
        } else {
            self.resource_chunks.insert(j, 1);
            1
        };
        // Update available deposits.
        let chunk = &config.deposits.chunks[j as usize];
        let coverage = coverage as f32 / chunk.tile_count as f32;
        for (resource, amount) in &chunk.deposits {
            let amount = amount * coverage;
            if let Some(x) = self.deposits.get_mut(resource) {
                *x += amount;
            } else {
                self.deposits.insert(*resource, amount);
            }
        }
        // Mark to redraw.
        self.need_visual_update = true;
    }

    pub fn update_resources(&mut self, config: &AtlasSimConfig) {
        // Calculate potential resources.
        let mut supply_max = 0.0;
        let mut industry_max = 0.0;
        let mut wealth_max = 0.0;
        let mut supply_amount = 0.0;
        let mut industry_amount = 0.0;
        let mut wealth_amount = 0.0;
        for (id, amount) in self.deposits.iter() {
            let deposit = &config.deposits.types[*id as usize];
            supply_max += amount * deposit.supply; // TODO * get_supply_modifier_for_deposit()
            supply_amount += amount * deposit.supply;
            industry_max += amount * deposit.industry;
            industry_amount += amount * deposit.industry;
            wealth_max += amount * deposit.wealth;
            wealth_amount += amount * deposit.wealth;
        }
        let supply_bonus = if supply_max.is_zero() {
            0.0
        } else {
            supply_max / supply_amount
        };
        let industry_bonus = if industry_max.is_zero() {
            0.0
        } else {
            industry_max / industry_amount
        };
        let wealth_bonus = if wealth_max.is_zero() {
            0.0
        } else {
            wealth_max / wealth_amount
        };
        // Calculate work output.
        self.supply = (*self.jobs.get(&0).unwrap_or(&0.0) * config.jobs.types[0].efficiency * supply_bonus)
            .min(supply_max); // * TODO get_supply_modifier()
        self.industry =
            (*self.jobs.get(&1).unwrap_or(&0.0) * config.jobs.types[1].efficiency * industry_bonus)
                .min(industry_max);
        self.wealth = (*self.jobs.get(&2).unwrap_or(&0.0) * config.jobs.types[2].efficiency * wealth_bonus)
            .min(wealth_max);
        // TODO Advanced jobs
    }

    pub fn update_jobs(&mut self, config: &AtlasSimConfig) {
        // Reset jobs.
        self.jobs.clear();
        let manpower = self.population; // TODO get_population_manpower_ratio()
                                        // Calculate potential supply.
        let mut supply_max = 0.0;
        let mut supply_amount = 0.0;
        for (id, amount) in self.deposits.iter() {
            let deposit = &config.deposits.types[*id as usize];
            supply_max += amount * deposit.supply; // TODO * get_supply_modifier_for_deposit()
            supply_amount += amount * deposit.supply;
        }
        // Early exit if no supplies to be made.
        if supply_max.is_zero() {
            self.jobs.insert(0, manpower);
            return;
        }
        let supply_bonus = supply_max / supply_amount;
        // Consumption target should always be met.
        let consumption = self.get_supply_consumption(&config);
        let minimum_supply_manpower = consumption / config.jobs.types[0].efficiency / supply_bonus; // TODO / get_supply_modifier()
        let minimum_supply_manpower = minimum_supply_manpower.min(manpower);
        let spare_manpower = manpower - minimum_supply_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            self.jobs.insert(0, minimum_supply_manpower);
            return;
        }
        // Assign spare manpower to other sectors.
        let manpower_split = [0.1, 0.45, 0.45]; // TODO: Should be decided by govt / culture.
        let supply_manpower = minimum_supply_manpower + spare_manpower * manpower_split[0];
        let industry_manpower = spare_manpower * manpower_split[1];
        let wealth_manpower = spare_manpower * manpower_split[2];
        self.jobs.insert(0, supply_manpower);
        self.jobs.insert(1, industry_manpower);
        self.jobs.insert(2, wealth_manpower);
        // TODO Assign manpower to secondary sectors.
    }

    fn get_supply_consumption(&self, config: &AtlasSimConfig) -> f32 {
        if config.rules.supply_per_pop.is_zero() {
            0.0
        } else {
            self.population * config.rules.supply_per_pop
        }
    }
}
