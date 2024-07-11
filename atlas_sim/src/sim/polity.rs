use std::collections::BTreeSet;

use atlas_lib::{
    bevy::{
        ecs as bevy_ecs,
        prelude::*,
        render::{
            render_asset::RenderAssetUsages,
            render_resource::{Extent3d, TextureDimension, TextureFormat},
        },
        utils::hashbrown::HashMap,
    },
    bevy_egui,
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::AtlasConfig,
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

use crate::{
    config::AtlasSimConfig,
    sim::{check_tick, SimMapData},
};

/// Polity simulation.
pub struct PolityPlugin;

impl Plugin for PolityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_visuals)
            .add_systems(FixedUpdate, (update_mapgrab, update_resources).run_if(check_tick));
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
    /// Map of available resources.
    pub resources: HashMap<u32, f32>,
}

impl Polity {
    pub fn rcrs(&mut self) -> (&HashMap<u32, u16>, &mut HashMap<u32, f32>) {
        (&self.resource_chunks, &mut self.resources)
    }

    pub fn into_ui(&self, config: &AtlasSimConfig) -> PolityUi {
        PolityUi {
            ownership: self.ownership,
            color: color_to_u8(&self.color),
            land_claim_points: self.land_claim_points,
            resources: self
                .resources
                .iter()
                .map(|(k, v)| (config.resources.types[*k as usize].name.clone(), *v))
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
    /// Map of available resources.
    pub resources: Vec<(String, f32)>,
}

impl MakeUi for PolityUi {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarEnumDropdown::new(ui, "Ownership", &mut self.ownership).show(None);
        SidebarColor::new(ui, "Color", &mut self.color).show(None);
        SidebarSlider::new(ui, "Land Claim Points", &mut self.land_claim_points).show(None);
        for (k, v) in &mut self.resources {
            SidebarSlider::new(ui, format!("Resource type '{}'", k), v).show(None);
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
            resources: Default::default(),
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

fn update_resources(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>) {
    for mut polity in query.iter_mut() {
        // Set all resources to 0.
        for (_, amount) in polity.resources.iter_mut() {
            *amount = 0.0;
        }
        // Add resource chunk deposits.
        let (resource_chunks, resources) = polity.rcrs();
        for (chunk, count) in resource_chunks {
            let chunk = &config.resources.chunks[*chunk as usize];
            let percent_owned = *count as f32 / chunk.tile_count as f32;
            for (resource, amount) in &chunk.resources {
                let amount = amount * percent_owned;
                if let Some(x) = resources.get_mut(resource) {
                    *x += amount;
                } else {
                    resources.insert(*resource, amount);
                }
            }
        }
        // Process jobs.
        // TODO
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
        let j = config.index_to_chunk(tile, config.resources.chunk_size as u32);
        if let Some(chunk) = self.resource_chunks.get_mut(&j) {
            *chunk += 1;
        } else {
            self.resource_chunks.insert(j, 1);
        }
        // Mark to redraw.
        self.need_visual_update = true;
    }
}
