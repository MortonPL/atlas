use std::{collections::BTreeSet, f32::consts::FRAC_PI_2};

use atlas_lib::{
    bevy::{
        ecs as bevy_ecs,
        prelude::*,
        render::{
            mesh::PlaneMeshBuilder,
            render_asset::RenderAssetUsages,
            render_resource::{Extent3d, TextureDimension, TextureFormat},
        },
        utils::{
            hashbrown::{HashMap, HashSet},
            petgraph::matrix_graph::Zero,
        },
    },
    bevy_egui::{self},
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::{sim::AtlasSimConfig, AtlasConfig},
    domain::{
        graphics::{color_to_u8, MapLogicData},
        map::{is_sea, MapDataLayer, MapDataOverlay},
    },
    rand::Rng,
    ui::{sidebar::*, UiEditableEnum},
    MakeUi,
};
use bevy_mod_picking::{prelude::*, PickableBundle};
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::{
    sim::{check_tick, SimControl, SimMapData},
    ui::{MapOverlay, UpdateSelectionEvent},
};

use super::ui::{CityUi, PolityUi};

/// Polity simulation.
pub struct PolityPlugin;

impl Plugin for PolityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_visuals).add_systems(
            FixedUpdate,
            (
                update_mapgrab,
                update_pops,
                update_jobs,
                update_resources,
                update_construction,
                update_culture,
                update_tech,
                update_splits,
            )
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
    /// Produced resources.
    pub resources: [f32; LEN_RES],
    /// Researched technology.
    pub tech: [f32; LEN_TECH],
    /// Tech points accumulated this year.
    pub tech_acc: f32,
    /// Upkept traditions.
    pub traditions: [f32; LEN_TRAD],
    /// Tradition points accumulated this year.
    pub tradition_acc: f32,
    /// Govt policies.
    pub policies: [f32; LEN_POL],
    /// Total polity population.
    pub population: f32,
    /// Population jobs.
    pub jobs: JobStruct,
    /// Owned cities.
    pub cities: Vec<Entity>,
    /// Accumulated heritage.
    pub heritage: [f32; LEN_TRAD],
    /// Created great works.
    pub great_works: Vec<GreatWork>,
    /// Accumulated polity currency.
    pub treasure_acc: f32,
    /// Construction points accumulated this year.
    pub const_acc: f32,
    /// Advanced resource capacities.
    pub capacities: [f32; 6],
    /// Population split.
    pub manpower_split: [f32; 3],
    /// Production split.
    pub indu_split: [f32; 2],
    /// Wealth split.
    pub wealth_split: [f32; 3],
    /// Technology split.
    pub tech_split: [f32; LEN_TECH],
    /// Tradition split.
    pub trad_split: [f32; LEN_TRAD],
    /// Structure split.
    pub struct_split: [f32; LEN_STR],
}

impl Default for Polity {
    fn default() -> Self {
        Self {
            tiles: Default::default(),
            border_tiles: Default::default(),
            centroid: Vec2::ZERO,
            xywh: [0, 0, 1, 1],
            ownership: Ownership::Independent,
            color: Default::default(),
            need_visual_update: true,
            land_claim_points: 0.0,
            resource_chunks: Default::default(),
            deposits: Default::default(),
            resources: Default::default(),
            tech: Default::default(),
            tech_acc: 0.0,
            traditions: Default::default(),
            tradition_acc: 0.0,
            policies: Default::default(),
            population: 0.0,
            heritage: Default::default(),
            great_works: Default::default(),
            cities: Default::default(),
            jobs: Default::default(),
            capacities: Default::default(),
            treasure_acc: 0.0,
            const_acc: 0.0,
            manpower_split: Default::default(),
            indu_split: Default::default(),
            wealth_split: Default::default(),
            tech_split: Default::default(),
            trad_split: Default::default(),
            struct_split: Default::default(),
        }
    }
}

impl Polity {
    pub fn into_ui(&self, config: &AtlasSimConfig) -> PolityUi {
        PolityUi {
            ownership: self.ownership,
            color: color_to_u8(&self.color),
            land_claim_points: self.land_claim_points,
            cities: self.cities.len() as u32,
            deposits: self
                .deposits
                .iter()
                .map(|(k, v)| (config.deposits.types[*k as usize].name.clone(), *v))
                .collect(),
            resources: self.resources.clone(),
            tech: self.tech.clone(),
            tech_acc: self.tech_acc,
            traditions: self.traditions.clone(),
            tradition_acc: self.tradition_acc,
            policies: self.policies.clone(),
            population: self.population,
            heritage: self.heritage.clone(),
            great_works: self.great_works.clone(),
            jobs: self.jobs.clone(),
            treasure_acc: self.treasure_acc,
            const_acc: self.const_acc,
        }
    }
}

#[derive(Clone, Default)]
pub struct GreatWork {
    /// Tradition associated with this great work.
    pub tradition: u8,
    /// Time of creation.
    pub time: u32,
}

#[derive(Clone, Default, MakeUi)]
pub struct JobStruct {
    #[name("Non-Working")]
    #[control(SidebarSlider)]
    pub non_working: f32,
    #[name("Agriculture Workers")]
    #[control(SidebarSlider)]
    pub supply: f32,
    #[name("Industry Workers")]
    #[control(SidebarSlider)]
    pub industry: f32,
    #[name("Servants & Specialists")]
    #[control(SidebarSlider)]
    pub wealth: f32,
}

/// A city belonging to a polity.
#[derive(Component, Clone)]
pub struct City {
    /// Visuals need to be updated due to color or shape changes.
    pub need_visual_update: bool,
    /// Position on the map.
    pub position: u32,
    /// Owner polity.
    pub owner: Entity,
    /// Urbanization level.
    pub level: f32,
    /// Level of special structures.
    pub structures: [f32; LEN_STR],
}

impl City {
    pub fn into_ui(&self, _config: &AtlasSimConfig) -> CityUi {
        CityUi {
            level: self.level,
            structures: self.structures.clone(),
        }
    }
}

pub const LEN_RES: usize = 8;
/// Supply
const RES_SUPPLY: usize = 0;
/// Industry Consumption
const RES_INDU_POPS: usize = 1;
/// Civilian Industry
const RES_CIVILIAN: usize = 2;
/// Military Industry
const RES_MILITARY: usize = 3;
/// Wealth Consumption
const RES_WEALTH_POPS: usize = 4;
/// Research
const RES_RESEARCH: usize = 5;
/// Culture
const RES_CULTURE: usize = 6;
/// Treasure
const RES_TREASURE: usize = 7;

pub const RES_LABELS: [&str; LEN_RES] = [
    "Supply",
    "Industry Consumption",
    "Civilian Industry",
    "Military Industry",
    "Wealth Consumption",
    "Research",
    "Culture",
    "Treasure",
];

pub const LEN_TECH: usize = 13;
/// Arable & grazing land bonus
const TECH_AGRICULTURE: usize = 0;
/// Fishing bonus, sea movement bonus
const TECH_ASTRONOMY: usize = 1;
/// Forest & wild game bonus
const TECH_FORESTRY: usize = 2;
/// Rock and Ore deposits bonus
const TECH_GEOLOGY: usize = 3;
/// Civil engineering bonus
const TECH_ENGINEERING: usize = 4;
/// Military engineering bonus
const TECH_METALLURGY: usize = 5;
/// Culture bonus
const TECH_PHILOSOPHY: usize = 6;
/// Science bonus
const TECH_MATHEMATICS: usize = 7;
/// Pop growth bonus
const TECH_MEDICINE: usize = 8;
/// Treasure bonus
const TECH_FINANCES: usize = 9;
/// Governance bonus
const TECH_LAW: usize = 10;
/// Diplomacy bonus
const TECH_LINGUISTICS: usize = 11;
/// Military bonus
const TECH_PHYSICS: usize = 12;

pub const TECH_LABELS: [&str; LEN_TECH] = [
    "Agriculture",
    "Astronomy",
    "Forestry",
    "Geology",
    "Engineering",
    "Metallurgy",
    "Philosophy",
    "Mathematics",
    "Medicine",
    "Finances",
    "Law",
    "Linguistics",
    "Physics",
];

pub const LEN_POL: usize = 7;
/// Growth policy: Isolationist (improve land) vs Expansionist (claim land)
const POL_EXPANSIONIST: usize = 0;
/// Diplomacy policy: Cooperative (deals) vs Competitive (threats)
const POL_COMPETITIVE: usize = 1;
/// Consumption policy: Strict (low consumption) vs Liberal (high consumption)
const POL_LIBERAL: usize = 2;
/// Work Split policy: Industrial (industry) vs Mercantile (wealth)
const POL_MERCANTILE: usize = 3;
/// Industry policy: Pacifist (civilian ind) vs Militarist (military ind)
const POL_MILITARIST: usize = 4;
/// Wealth policy: Traditional (culture) vs Progressive (science)
const POL_PROGRESSIVE: usize = 5;
/// Treasure policy: Spending (low treasure) vs Greedy (high treasure)
const POL_GREEDY: usize = 6;

pub const POL_LABELS: [&str; LEN_POL] = [
    "Expansionist",
    "Competitive",
    "Liberal",
    "Mercantile",
    "Militarist",
    "Progressive",
    "Spending",
];

pub const LEN_TRAD: usize = 8;
/// Supply bonus / Great Economist
const TRAD_AGRARIAN: usize = 0;
/// Production bonus / Great Economist
const TRAD_INDUSTRIOUS: usize = 1;
/// Wealth bonus / Great Economist
const TRAD_MERCANTILE: usize = 2;
/// Science bonus / Great Scientist
const TRAD_PROGRESSIVE: usize = 3;
/// Culture bonus / Great Artist
const TRAD_TRADITIONAL: usize = 4;
/// Governance bonus / Great Governor
const TRAD_LEGALIST: usize = 5;
/// Diplomacy bonus / Great Diplomat
const TRAD_COOPERATIVE: usize = 6;
/// Military bonus / Great General
const TRAD_MILITANT: usize = 7;

pub const TRAD_LABELS: [&str; LEN_TRAD] = [
    "Agrarian",
    "Industrious",
    "Mercantile",
    "Progressive",
    "Traditional",
    "Legalist",
    "Cooperative",
    "Militant",
];

pub const LEN_STR: usize = 7;
/// Hospital / Pop growth bonus?
const STR_HOSPITAL: usize = 0;
/// Manufacture / Trade Goods cap
const STR_MANUFACTURE: usize = 1;
/// Forge / Military Equiment cap
const STR_FORGE: usize = 2;
/// University / Research cap
const STR_UNIVERSITY: usize = 3;
/// Amphitheater / Culture cap
const STR_AMPHITHEATER: usize = 4;
/// Courthouse / Governance cap
const STR_COURTHOUSE: usize = 5;
/// Fortress / Military cap
const STR_FORTRESS: usize = 6;

pub const STR_LABELS: [&str; LEN_STR] = [
    "Hospital",
    "Manufacture",
    "Forge",
    "University",
    "Amphitheater",
    "Courthouse",
    "Fortress",
];

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
        if polity.land_claim_points <= config.rules.misc.land_claim_cost {
            continue;
        }
        polity.land_claim_points -= config.rules.misc.land_claim_cost;
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
    query.iter_mut().for_each(|mut x| x.update_pops(&config));
}

/// Update system
///
/// Assign jobs.
fn update_jobs(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>) {
    query.iter_mut().for_each(|mut x| x.update_jobs(&config));
}

/// Update system
///
/// Update resources (supply/industry/wealth).
fn update_resources(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>) {
    query.iter_mut().for_each(|mut x| x.update_resources(&config));
}

/// Update system
///
/// Update construction.
fn update_construction(
    config: Res<AtlasSimConfig>,
    mut query_p: Query<(Entity, &mut Polity)>,
    mut query_c: Query<&mut City>,
    sim: Res<SimControl>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if sim.is_new_year() {
        for (owner, mut polity) in query_p.iter_mut() {
            // Update construction.
            let city_request = polity.update_construction(&config, &mut query_c, &mut rng);
            // Spawn new cities.
            if let Some(position) = city_request {
                let entity = commands.spawn_empty().id();
                let city = City {
                    need_visual_update: true,
                    position,
                    owner,
                    level: 1.0,
                    structures: Default::default(),
                };
                let position = config.index_to_world(position);
                init_city(city, entity, position, &mut meshes, &mut materials, &mut commands);
                polity.cities.push(entity);
            }
        }
    }
}

/// Update system
///
/// Update culture.
fn update_culture(
    config: Res<AtlasSimConfig>,
    mut query: Query<&mut Polity>,
    sim: Res<SimControl>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if sim.is_new_year() {
        query
            .iter_mut()
            .for_each(|mut x| x.update_culture(&config, &sim, &mut rng));
    }
}

/// Update system
///
/// Update tech.
fn update_tech(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>, sim: Res<SimControl>) {
    if sim.is_new_year() {
        query.iter_mut().for_each(|mut x| x.update_tech(&config));
    }
}

/// Update system
///
/// Update resource splits.
fn update_splits(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>, sim: Res<SimControl>) {
    if sim.is_new_year() {
        query.iter_mut().for_each(|mut x| x.update_splits(&config));
    }
}

/// Update system
///
/// Update polity & city visuals.
fn update_visuals(
    config: Res<AtlasSimConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut query_p: Query<(&mut Polity, &mut Transform, &mut Handle<StandardMaterial>), With<Polity>>,
    mut query_c: Query<(&mut City, &mut Handle<StandardMaterial>), (With<City>, Without<Polity>)>,
    asset_server: Res<AssetServer>,
) {
    for (mut polity, mut tran, mut mat) in query_p.iter_mut() {
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
    for (mut city, mut mat) in query_c.iter_mut() {
        // Don't update if not needed.
        if !city.need_visual_update {
            continue;
        }
        let (polity, _, _) = query_p.get(city.owner).unwrap();
        *mat = materials.add(StandardMaterial {
            base_color: polity.color,
            base_color_texture: Some(asset_server.load("city.png")),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });
        city.need_visual_update = false;
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
                    .get_border_tiles_4(tile)
                    .iter()
                    .filter(|x| !extras.tile_owner[**x as usize].is_some_and(|y| y.eq(&entity))),
            );
        } else {
            self.border_tiles
                .extend(&mut config.get_border_tiles_4(tile).iter());
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

    pub fn update_pops(&mut self, config: &AtlasSimConfig) {
        // Calculate supply consumption and growth bonus.
        let consumption = self.get_supply_consumption(&config);
        let coverage = if consumption.is_zero() {
            1.0
        } else {
            (self.resources[RES_SUPPLY] / consumption).min(2.0)
        };
        let base_coverage = coverage.min(1.0);
        let hospital_coverage = if self.capacities[STR_HOSPITAL].is_zero() {
            0.0
        } else {
            (self.capacities[STR_HOSPITAL] * config.rules.economy.pop_hospital_factor) / self.population
        };
        let hospital_coverage = 1.0 - (1.0 - hospital_coverage) * config.rules.economy.pop_hospital_penalty;
        // Grow the population.
        self.population = (self.population
            * (1.0 * base_coverage
                + config.rules.economy.pop_growth
                    * coverage
                    * self.get_tech_multiplier(config, TECH_MEDICINE))
            * hospital_coverage)
            .max(1.0);
    }

    pub fn update_jobs(&mut self, config: &AtlasSimConfig) {
        let manpower = self.population; // TODO get_population_manpower_ratio()
                                        // Calculate potential supply.
        let mut supply_max = 0.0;
        let mut supply_amount = 0.0;
        let mut indu_max = 0.0;
        let mut indu_amount = 0.0;
        let mut wealth_max = 0.0;
        let mut wealth_amount = 0.0;
        for (id, amount) in self.deposits.iter() {
            let deposit = &config.deposits.types[*id as usize];
            let bonus = match *id {
                0..=2 => self.get_tech_multiplier(config, TECH_AGRICULTURE),
                3..=5 => self.get_tech_multiplier(config, TECH_FORESTRY),
                6 => self.get_tech_multiplier(config, TECH_ASTRONOMY),
                7..=10 => self.get_tech_multiplier(config, TECH_GEOLOGY),
                _ => 1.0,
            };
            supply_max += amount * deposit.supply * bonus;
            supply_amount += amount * deposit.supply;
            indu_max += amount * deposit.industry * bonus;
            indu_amount += amount * deposit.industry;
            wealth_max += amount * deposit.wealth * bonus;
            wealth_amount += amount * deposit.wealth;
        }
        // Early exit if no supplies to be made.
        if supply_max.is_zero() {
            self.jobs = JobStruct {
                non_working: self.population - manpower,
                supply: manpower,
                ..Default::default()
            };
            return;
        }
        let supply_bonus = supply_max / supply_amount;
        // Consumption target should always be met.
        let consumption = self.get_supply_consumption(&config);
        let minimum_supply_manpower = consumption
            / config.rules.economy.resources[RES_SUPPLY].efficiency
            / supply_bonus
            / self.get_tradition_multiplier(config, TRAD_AGRARIAN).max(1.0);
        let minimum_supply_manpower = minimum_supply_manpower.min(manpower);
        let spare_manpower = manpower - minimum_supply_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            self.jobs = JobStruct {
                non_working: self.population - manpower,
                supply: minimum_supply_manpower,
                ..Default::default()
            };
            return;
        }
        let indu_bonus = indu_max / indu_amount;
        // Consumption target should always be met.
        let consumption = self.get_industry_consumption(&config);
        let minimum_indu_manpower = consumption
            / config.rules.economy.resources[RES_INDU_POPS].efficiency
            / indu_bonus
            / self.get_tradition_multiplier(config, TRAD_INDUSTRIOUS).max(1.0);
        let minimum_indu_manpower = minimum_indu_manpower.min(spare_manpower);
        let spare_manpower = spare_manpower - minimum_indu_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            self.jobs = JobStruct {
                non_working: self.population - manpower,
                supply: minimum_supply_manpower,
                industry: minimum_indu_manpower,
                ..Default::default()
            };
            return;
        }
        let wealth_bonus = wealth_max / wealth_amount;
        // Consumption target should always be met.
        let consumption = self.get_wealth_consumption(&config);
        let minimum_wealth_manpower = consumption
            / config.rules.economy.resources[RES_WEALTH_POPS].efficiency
            / wealth_bonus
            / self.get_tradition_multiplier(config, TRAD_MERCANTILE).max(1.0);
        let minimum_wealth_manpower = minimum_wealth_manpower.min(spare_manpower);
        let spare_manpower = spare_manpower - minimum_wealth_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            self.jobs = JobStruct {
                non_working: self.population - manpower,
                supply: minimum_supply_manpower,
                industry: minimum_indu_manpower,
                wealth: minimum_wealth_manpower,
            };
            return;
        }
        // Assign spare manpower.
        self.jobs = JobStruct {
            non_working: self.population - manpower,
            supply: minimum_supply_manpower * self.manpower_split[0],
            industry: minimum_indu_manpower + spare_manpower * self.manpower_split[1],
            wealth: minimum_wealth_manpower + spare_manpower * self.manpower_split[2],
        };
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
            let bonus = match *id {
                0..=2 => self.get_tech_multiplier(config, TECH_AGRICULTURE),
                3..=5 => self.get_tech_multiplier(config, TECH_FORESTRY),
                6 => self.get_tech_multiplier(config, TECH_ASTRONOMY),
                7..=10 => self.get_tech_multiplier(config, TECH_GEOLOGY),
                _ => 1.0,
            };
            supply_max += amount * deposit.supply * bonus;
            supply_amount += amount * deposit.supply;
            industry_max += amount * deposit.industry * bonus;
            industry_amount += amount * deposit.industry;
            wealth_max += amount * deposit.wealth * bonus;
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
        let supply = self.get_resource_yield(
            (self.jobs.supply * supply_bonus, supply_max, -1.0),
            (RES_SUPPLY, 1001, TRAD_AGRARIAN),
            config,
        );
        // Split primary resources into secondary resources (industry).
        let indu_pop = self.get_industry_consumption(&config);
        let industry =
            self.jobs.industry * self.get_tradition_multiplier(config, TRAD_INDUSTRIOUS) * industry_bonus;
        let industry = (industry - indu_pop).max(0.0);
        let mut civ_indu = 0.0;
        let mut mil_indu = 0.0;
        if industry > 0.0 {
            civ_indu = self.get_resource_yield(
                (industry * self.indu_split[0], industry_max, self.capacities[1]),
                (RES_CIVILIAN, TECH_ENGINEERING, 1001),
                config,
            );
            mil_indu = self.get_resource_yield(
                (industry * self.indu_split[1], industry_max, self.capacities[2]),
                (RES_MILITARY, TECH_METALLURGY, 1001),
                config,
            );
        };
        // Split primary resources into secondary resources (wealth).
        let wealth_pop = self.get_wealth_consumption(&config);
        let wealth = self.jobs.wealth * self.get_tradition_multiplier(config, TRAD_MERCANTILE) * wealth_bonus;
        let wealth = (wealth - wealth_pop).max(0.0);
        let mut research = 0.0;
        let mut culture = 0.0;
        let mut treasure = 0.0;
        if wealth > 0.0 {
            research = self.get_resource_yield(
                (wealth * self.wealth_split[0], wealth_max, self.capacities[3]),
                (RES_RESEARCH, TECH_MATHEMATICS, TRAD_PROGRESSIVE),
                config,
            );
            culture = self.get_resource_yield(
                (wealth * self.wealth_split[1], wealth_max, self.capacities[4]),
                (RES_CULTURE, TECH_PHILOSOPHY, TRAD_TRADITIONAL),
                config,
            );
            treasure = self.get_resource_yield(
                (wealth * self.wealth_split[2], wealth_max, self.capacities[5]),
                (RES_TREASURE, TECH_FINANCES, 10001),
                config,
            );
        }
        // Set new resources.
        self.resources = [
            supply, indu_pop, civ_indu, mil_indu, wealth_pop, research, culture, treasure,
        ];
        self.const_acc += civ_indu;
        self.tech_acc += research;
        self.tradition_acc += culture;
        self.treasure_acc += treasure;
    }

    pub fn update_construction(
        &mut self,
        config: &AtlasSimConfig,
        query: &mut Query<&mut City>,
        rng: &mut GlobalEntropy<WyRand>,
    ) -> Option<u32> {
        let mut city_count = 0.0;
        // Clear resource capacities.
        self.capacities = Default::default();
        let mut occupied_tiles = HashSet::<u32>::default();
        // Find cities that aren't maxxed out.
        for city in self.cities.iter() {
            let city = query.get(*city).unwrap();
            if city.level < config.rules.city.max_level {
                city_count += 1.0;
            }
            occupied_tiles.extend(config.get_border_tiles_9(city.position));
        }
        let value = (self.const_acc / city_count) * config.rules.city.base_speed;
        // Expand city.
        for city in self.cities.iter() {
            let mut city = query.get_mut(*city).unwrap();
            let multiplier = self.get_city_multiplier(config, city.level);
            let sum = city.structures.iter().fold(0.0, |acc, x| acc + x);
            let diff = city.level - sum;
            let mut value_str = value.min(diff);
            if diff >= 0.0 {
                let increment = value_str / multiplier;
                // Build special structures.
                for i in 0..city.structures.len() {
                    city.structures[i] +=
                        increment * self.struct_split[i] * config.rules.city.structures[i].cost;
                }
            } else {
                // City level dropped: deal structural damage.
                value_str = value;
                for i in 0..city.structures.len() {
                    city.structures[i] = city.level * self.struct_split[i];
                }
            }
            // Increase city level.
            if city.level < config.rules.city.max_level {
                let value = (value - value_str).max(0.0) / multiplier;
                city.level =
                    (city.level + value * config.rules.city.upgrade_speed).min(config.rules.city.max_level);
            }
            // Recalculate resource capacities.
            for i in 0..self.capacities.len() {
                self.capacities[i] += city.structures[i]
                    * config.rules.city.structures[i].strength
                    * config.rules.city.base_capacity;
            }
        }
        // Clear accumulated construction.
        self.const_acc = 0.0;
        // Request building new cities.
        // TODO
        if true {
            // Choose position
            let good_tiles: Vec<u32> = self
                .tiles
                .iter()
                .filter_map(|i| {
                    if occupied_tiles.contains(i) {
                        None
                    } else {
                        Some(*i)
                    }
                })
                .collect();
            if good_tiles.is_empty() {
                None
            } else {
                let i = rng.gen_range(0..good_tiles.len());
                Some(good_tiles[i])
            }
        } else {
            None
        }
    }

    pub fn update_culture(
        &mut self,
        config: &AtlasSimConfig,
        sim: &SimControl,
        rng: &mut GlobalEntropy<WyRand>,
    ) {
        let culture = self.tradition_acc * config.rules.culture.base_speed;
        for (i, val) in self.traditions.iter_mut().enumerate() {
            let increment = self.trad_split[i] * culture / config.rules.culture.traditions[i].cost
                - Self::get_tradition_decay(config, *val);
            let overflow = *val + increment - config.rules.culture.max_level;
            if overflow > 0.0 {
                *val = config.rules.culture.max_level;
                self.heritage[i] += overflow * config.rules.culture.heritage_ratio;
            } else {
                *val = (*val + increment).max(0.0);
            }
        }
        self.tradition_acc = 0.0;
        if config.rules.culture.great_event_heritage <= 0.0 {
            return;
        }
        for (i, val) in self.heritage.iter_mut().enumerate() {
            let chance = (*val / config.rules.culture.great_event_heritage)
                .min(config.rules.culture.great_event_chance_max);
            let great_event = rng.gen_bool(chance as f64);
            if !great_event {
                continue;
            }
            let great_person = rng.gen_bool(config.rules.culture.great_person_chance as f64);
            if great_person {
                // TODO add great person
            } else {
                // add great work
                self.great_works.push(GreatWork {
                    tradition: i as u8,
                    time: sim.time,
                })
            }
            *val = 0.0;
        }
    }

    pub fn update_tech(&mut self, config: &AtlasSimConfig) {
        let tech = self.tech_acc * config.rules.tech.base_speed;
        for (i, val) in self.tech.iter_mut().enumerate() {
            let increment = self.tech_split[i] * tech / config.rules.tech.techs[i].cost
                - Self::get_tech_decay(config, *val);
            *val = (*val + increment).clamp(0.0, config.rules.tech.max_level);
        }
        self.tech_acc = 0.0;
    }

    pub fn update_splits(&mut self, config: &AtlasSimConfig) {
        // Update manpower split.
        self.manpower_split = [
            0.0,
            1.0 - self.policies[POL_MERCANTILE],
            self.policies[POL_MERCANTILE],
        ];
        let sum: f32 = self.manpower_split.iter().sum();
        self.manpower_split = self.manpower_split.map(|x| x / sum);
        // Update industry split.
        self.indu_split = [1.0 - self.policies[POL_MILITARIST], self.policies[POL_MILITARIST]];
        let sum: f32 = self.indu_split.iter().sum();
        self.indu_split = self.indu_split.map(|x| x / sum);
        // Update wealth split.
        self.wealth_split = [
            self.policies[POL_PROGRESSIVE],
            1.0 - self.policies[POL_PROGRESSIVE],
            self.policies[POL_GREEDY],
        ];
        let sum: f32 = self.wealth_split.iter().sum();
        self.wealth_split = self.wealth_split.map(|x| x / sum);
        // Update technology split.
        self.tech_split = [
            1.0 - self.policies[POL_EXPANSIONIST],
            1.0 - self.policies[POL_EXPANSIONIST],
            1.0 - self.policies[POL_EXPANSIONIST],
            1.0 - self.policies[POL_EXPANSIONIST],
            0.5, // TODO
            1.0 - self.policies[POL_MILITARIST],
            self.policies[POL_MILITARIST],
            1.0 - self.policies[POL_PROGRESSIVE],
            self.policies[POL_PROGRESSIVE],
            self.policies[POL_GREEDY],
            1.0 - self.policies[POL_GREEDY],
            1.0 - self.policies[POL_COMPETITIVE],
            self.policies[POL_COMPETITIVE],
        ];
        let sum: f32 = self.tech_split.iter().sum();
        self.tech_split = self.tech_split.map(|x| x / sum);
        // Update tradition split.
        for (x, (tradition, split)) in self.trad_split.iter_mut().zip(
            self.traditions
                .iter()
                .zip(config.rules.misc.default_tradition_split),
        ) {
            *x = (1.0 + tradition) * split;
        }
        let sum: f32 = self.trad_split.iter().sum();
        self.trad_split = self.trad_split.map(|x| x / sum);
        // Update structue split.
        self.struct_split = [
            self.policies[POL_PROGRESSIVE],
            (1.0 - self.policies[POL_MILITARIST]),
            self.policies[POL_MILITARIST],
            self.policies[POL_PROGRESSIVE],
            (1.0 - self.policies[POL_PROGRESSIVE]),
            (1.0 - self.policies[POL_PROGRESSIVE]),
            self.policies[POL_COMPETITIVE],
        ];
        let sum: f32 = self.struct_split.iter().sum();
        self.struct_split = self.struct_split.map(|x| x / sum);
    }

    fn get_supply_consumption(&self, config: &AtlasSimConfig) -> f32 {
        if config.rules.economy.base_supply_need.is_zero() {
            0.0
        } else {
            self.population * config.rules.economy.base_supply_need
        }
    }

    fn get_industry_consumption(&self, config: &AtlasSimConfig) -> f32 {
        if config.rules.economy.base_industry_need.is_zero() {
            0.0
        } else {
            self.population * config.rules.economy.base_industry_need
        }
    }

    fn get_wealth_consumption(&self, config: &AtlasSimConfig) -> f32 {
        if config.rules.economy.base_wealth_need.is_zero() {
            0.0
        } else {
            self.population * config.rules.economy.base_wealth_need
        }
    }

    fn get_resource_yield(
        &self,
        input_max_cap: (f32, f32, f32),
        res_tech_trad: (usize, usize, usize),
        config: &AtlasSimConfig,
    ) -> f32 {
        let (res, tech, trad) = res_tech_trad;
        let mut out = (input_max_cap.0 * config.rules.economy.resources[res].efficiency).min(input_max_cap.1);
        if tech < 1000 {
            out *= self.get_tech_multiplier(config, tech);
        }
        if trad < 1000 {
            out *= self.get_tradition_multiplier(config, trad);
        }
        if input_max_cap.2 >= 0.0 {
            let diff = out - input_max_cap.2;
            if diff > 0.0 {
                out = out + diff * config.rules.economy.resources[res].over_cap_efficiency;
            }
        }
        out
    }

    fn get_city_multiplier(&self, config: &AtlasSimConfig, city: f32) -> f32 {
        1.0 + config.rules.city.level_cost * city.floor()
    }

    fn get_tech_multiplier(&self, config: &AtlasSimConfig, i: usize) -> f32 {
        let strength = config.rules.tech.techs[i].strength * self.tech[i].floor();
        1.0 + config.rules.tech.level_bonus * strength
    }

    fn get_tradition_multiplier(&self, config: &AtlasSimConfig, i: usize) -> f32 {
        let strength = config.rules.culture.traditions[i].strength * self.traditions[i].floor();
        1.0 + config.rules.culture.level_bonus * strength
    }

    fn get_tech_decay(config: &AtlasSimConfig, value: f32) -> f32 {
        config.rules.tech.base_decay * (1.0 + config.rules.tech.level_decay * value.floor())
    }

    fn get_tradition_decay(config: &AtlasSimConfig, value: f32) -> f32 {
        config.rules.culture.base_decay * (1.0 + config.rules.culture.level_decay * value.floor())
    }
}

pub fn init_city(
    city: City,
    entity: Entity,
    position: (f32, f32),
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    commands: &mut Commands,
) {
    commands.get_entity(entity).unwrap().insert((
        city,
        PbrBundle {
            mesh: meshes.add(PlaneMeshBuilder::new(Direction3d::Y, Vec2::ONE / 50.0)),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_xyz(position.0, position.1, 0.02).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                FRAC_PI_2,
                0.0,
                0.0,
            )),
            visibility: Visibility::Visible,
            ..Default::default()
        },
        PickableBundle::default(),
        On::<Pointer<Click>>::send_event::<UpdateSelectionEvent>(),
        MapOverlay::new(MapDataOverlay::Cities),
    ));
}
