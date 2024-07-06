use std::collections::BTreeSet;

use atlas_lib::{
    bevy::{
        ecs as bevy_ecs,
        prelude::*,
        render::{
            render_asset::RenderAssetUsages,
            render_resource::{Extent3d, TextureDimension, TextureFormat},
        },
    },
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::AtlasConfig,
    domain::{graphics::MapLogicData, map::{is_sea, MapDataLayer}},
};
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::config::AtlasSimConfig;

use crate::sim::check_tick;

use super::SimMapData;

/// Plugin responsible for the actual simulation.
pub struct PolityPlugin;

impl Plugin for PolityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_visuals)
            .add_systems(FixedUpdate, (update_mapgrab).run_if(check_tick));
    }
}

/// Ownership status of a polity.
#[derive(Default)]
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

/// A political entity that owns land and population.
#[derive(Component)]
pub struct Polity {
    /// Map tile indices that this polity owns.
    pub tiles: Vec<u32>,
    /// Map tile indices outside of the polity border.
    pub border_tiles: BTreeSet<u32>,
    /// Centroid of owned land, in map coords.
    pub centroid: Vec2,
    /// XYWH bounding box in map coordinates.
    pub xywh: [u32; 4],
    /// Ownership status.
    pub ownership: Ownership,
    /// Polity map color.
    pub color: Color,
    /// Visuals need to be updated due to color or shape changes.
    pub need_visual_update: bool,
    /// The desire to claim border tiles.
    pub expansion_desire: f32,
}

impl Default for Polity {
    fn default() -> Self {
        Self {
            tiles: vec![0],
            border_tiles: Default::default(),
            centroid: Vec2::ZERO,
            xywh: [0, 0, 1, 1],
            ownership: Ownership::Independent,
            color: Default::default(),
            need_visual_update: true,
            expansion_desire: 0.0,
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
        if polity.expansion_desire <= config.rules.land_claim_cost {
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
        polity.border_tiles.remove(&i);
        extras.tile_owner[i as usize] = Some(entity);
        polity.tiles.push(i);
        // Update xywh.
        polity.tiles.sort();
        let (width, _) = config.get_world_size();
        let (first, last) = (polity.tiles[0], polity.tiles[polity.tiles.len() - 1]);
        let mut min = width;
        let mut max = 0;
        for t in &polity.tiles {
            let v = t % width;
            min = std::cmp::min(min, v);
            max = std::cmp::max(max, v);
        }
        let (x, y, w) = (min, first / width, max - min + 1);
        let h = last / width + 1 - y;
        polity.xywh = [x, y, w, h];
        // Recalculate centroid.
        polity.centroid = Vec2::new(x as f32 + w as f32 / 2.0, y as f32 + h as f32 / 2.0);
        // Update polity borders.
        polity.border_tiles.extend(
            &mut config
                .get_border_tiles(i)
                .iter()
                .filter(|x| !extras.tile_owner[**x as usize].is_some_and(|y| y.eq(&entity))),
        );
        // Mark to redraw.
        polity.need_visual_update = true;
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
