use std::{collections::BTreeSet, f32::consts::FRAC_PI_2};

use atlas_lib::{
    bevy::{
        ecs as bevy_ecs,
        prelude::*,
        render::{mesh::PlaneMeshBuilder, render_resource::Extent3d},
        utils::{HashMap, HashSet},
    },
    config::{sim::AtlasSimConfig, AtlasConfig},
    domain::map::{is_sea, MapDataOverlay},
};

use crate::ui::MapOverlay;

use super::{
    polity::{Polity, LEN_STR},
    ui::RegionUi,
    SimMapData,
};

#[derive(Component)]
pub struct City;

#[derive(Component, Clone)]
pub struct Region {
    /// Centroid of owned land, in map coords.
    pub centroid: Vec2,
    /// XYWH bounding box in map coordinates.
    pub xywh: [u32; 4],
    /// Extra color light to apply (for slight variation).
    pub color_l: f32,
    /// Visuals need to be updated due to color or shape changes.
    pub need_visual_update: bool,
    /// Map tile indices that this region owns.
    pub tiles: Vec<u32>,
    /// Map tile indices outside of the polity border.
    pub border_tiles: BTreeSet<u32>,
    /// Is there free land bordering this region?
    pub can_expand: bool,
    /// Is the region big enough to split?
    pub can_split_size: bool,
    /// Map of # of tiles owned in resource chunks.
    pub resource_chunks: HashMap<u32, u16>,
    /// Map of available deposits.
    pub deposits: HashMap<u32, f32>,
    /// Region population.
    pub population: f32,
    /// Owner polity.
    pub polity: Entity,
    /// City marker.
    pub city: Entity,
    /// Land claim points.
    pub land_claim_fund: f32,
    /// New city points.
    pub new_city_fund: f32,
    /// Tile index of this region's city.
    pub city_position: u32,
    /// New city can be built here.
    pub split_tiles: BTreeSet<u32>,
    /// Development level.
    pub development: f32,
    /// Level of special structures.
    pub structures: [f32; LEN_STR],
    /// Stability level.
    pub stability: f32,
    /// Healthcare level.
    pub healthcare: f32,
    /// Security force power.
    pub security: f32,
    /// Public health power.
    pub health: f32,
    /// Sum of all structure levels.
    pub struct_levels: f32,
}

impl Region {
    pub fn new(polity: Entity, city: Entity, city_position: u32) -> Self {
        Self {
            centroid: Vec2::ZERO,
            xywh: [0, 0, 1, 1],
            need_visual_update: true,
            tiles: Default::default(),
            border_tiles: Default::default(),
            resource_chunks: Default::default(),
            deposits: Default::default(),
            population: 0.0,
            land_claim_fund: 0.0,
            new_city_fund: 0.0,
            city_position,
            can_expand: false,
            can_split_size: false,
            split_tiles: Default::default(),
            polity,
            city,
            color_l: 0.0,
            development: 1.0,
            structures: Default::default(),
            stability: 1.0,
            healthcare: 1.0,
            security: 0.0,
            health: 0.0,
            struct_levels: 0.0,
        }
    }

    pub fn claim_tile(
        &mut self,
        this: Entity,
        tile: u32,
        weight: f32,
        extras: &mut SimMapData,
        config: &AtlasSimConfig,
    ) {
        self.land_claim_fund -= config.rules.region.land_claim_cost * (2.0 - weight);
        self.border_tiles.remove(&tile);
        extras.tile_region[tile as usize] = Some(this);
        extras.tile_polity[tile as usize] = Some(self.polity);
        self.tiles.push(tile);
        // Recalculate xywh & centroid.
        self.update_xywh(config);
        // Update region borders.
        self.border_tiles.extend(
            &mut config
                .get_border_tiles_4(tile)
                .iter()
                .filter(|x| !extras.tile_region[**x as usize].is_some_and(|y| y.eq(&this))),
        );
        // Update resource chunk coverage.
        let (chunk, coverage) = self.update_chunk_coverage(config, tile);
        // Update available deposits.
        self.update_deposits_from_chunk(config, chunk, coverage);
        // Lower development.
        self.development = self.development - self.development / self.tiles.len() as f32;
        // Check if the region is big and spacious enough to split.
        self.can_split_size = self.tiles.len() as u32 > config.rules.region.min_split_size;
        if !extras.city_borders.contains(&tile) {
            self.split_tiles.insert(tile);
        }
        // Mark to redraw.
        self.need_visual_update = true;
    }

    pub fn into_ui(&self, config: &AtlasSimConfig) -> RegionUi {
        RegionUi {
            population: self.population,
            deposits: self
                .deposits
                .iter()
                .map(|(k, v)| (config.deposits.types[*k as usize].name.clone(), *v))
                .collect(),
            tiles: self.tiles.len() as u32,
            land_claim: self.land_claim_fund,
            city_fund: self.new_city_fund,
            development: self.development,
            structures: self.structures.clone(),
            stability: self.stability,
            healthcare: self.healthcare,
            security: self.security,
            health: self.health,
        }
    }

    pub fn reset_tiles(
        &mut self,
        this: Entity,
        tiles: Vec<u32>,
        config: &AtlasSimConfig,
        extras: &SimMapData,
        conts: &[u8],
        climate: &[u8],
    ) {
        // Clear old data.
        self.border_tiles.clear();
        self.deposits.clear();
        self.resource_chunks.clear();
        for tile in tiles.iter() {
            let tile = *tile;
            // Update resource chunk coverage.
            self.update_chunk_coverage(config, tile);
            // Update region borders.
            self.border_tiles.extend(
                config
                    .get_border_tiles_4(tile)
                    .iter()
                    .filter(|x| !extras.tile_region[**x as usize].is_some_and(|y| y.eq(&this))),
            );
        }
        // Update available deposits.
        let resource_chunks = std::mem::take(&mut self.resource_chunks);
        for (chunk, coverage) in resource_chunks.iter() {
            self.update_deposits_from_chunk(config, *chunk, *coverage);
        }
        self.resource_chunks = resource_chunks;
        // Assign tiles, check if the region can expand or split.
        self.tiles = tiles;
        self.update_expansion(&config, &extras, conts, climate);
        self.can_split_size = self.tiles.len() as u32 > config.rules.region.min_split_size;
        self.split_tiles = self
            .tiles
            .iter()
            .filter(|x| !extras.city_borders.contains(x))
            .map(|x| *x)
            .collect();
        // Recalculate xywh & centroid.
        self.update_xywh(config);
        self.need_visual_update = true;
    }

    #[inline(always)]
    pub fn can_split(&self) -> bool {
        self.can_split_size && !self.split_tiles.is_empty()
    }

    pub fn update_visuals(
        &mut self,
        config: &AtlasSimConfig,
        polity: &Polity,
        tran: &mut Transform,
        mat: &mut Handle<StandardMaterial>,
        city_mat: &mut Handle<StandardMaterial>,
        images: &mut Assets<Image>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        let (w, _) = config.get_world_size();
        let (x, y, width, height) = (self.xywh[0], self.xywh[1], self.xywh[2], self.xywh[3]);
        // Make new texture data.
        let (off, diff) = (w * y + x, w - width);
        let mut data = vec![0; width as usize * height as usize * 4];
        for i in &self.tiles {
            let i = i - off;
            let i = ((i - diff * (i / w)) * 4) as usize;
            data[i] = 255;
            data[i + 1] = 255;
            data[i + 2] = 255;
            data[i + 3] = 255;
        }
        // Update world space origin and scale.
        let p = config.centroid_to_world_centered(self.centroid.into());
        let s = (width as f32 / 100.0, height as f32 / 100.0);
        tran.translation = Vec3::new(p.0, p.1, 0.0);
        tran.scale = Vec3::new(s.0, s.1, s.1);
        // Update the material (with tint) and texture (with shape).
        let mat = materials.get_mut(mat.clone()).unwrap();
        let img = mat.base_color_texture.as_ref().map(|x| x.clone()).unwrap();
        let img = images.get_mut(img).unwrap();
        img.texture_descriptor.size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        img.data = data;
        let color = polity.color.with_l(polity.color.l() + self.color_l);
        mat.base_color = color;
        // Update city material wit tint as well.
        let mat = materials.get_mut(city_mat.clone()).unwrap();
        mat.base_color = color;
        // Mark as done.
        self.need_visual_update = false;
    }

    pub fn update_expansion(
        &mut self,
        config: &AtlasSimConfig,
        extras: &SimMapData,
        conts: &[u8],
        climate: &[u8],
    ) -> Vec<f32> {
        let mut weights = vec![];
        for tile in self.border_tiles.iter() {
            let tile = *tile as usize;
            let weight = match extras.tile_region[tile] {
                Some(_) => 0.0,
                None => {
                    if is_sea(conts[tile]) {
                        0.0
                    } else {
                        config.get_biome(climate[tile]).habitability
                    }
                }
            };
            weights.push(weight);
        }
        self.can_expand = !weights.is_empty() && weights.iter().fold(0.0f32, |acc, x| acc.max(*x)) > 0.1;
        weights
    }

    pub fn get_border_regions(&self, extras: &SimMapData) -> HashMap<Entity, HashSet<Entity>> {
        let mut map: HashMap<Entity, HashSet<Entity>> = Default::default();
        for tile in &self.border_tiles {
            let tile = *tile as usize;
            let polity = match &extras.tile_polity[tile] {
                Some(polity) => *polity,
                None => continue,
            };
            if polity == self.polity {
                continue;
            }
            let region = extras.tile_region[tile].unwrap();
            if let Some(set) = map.get_mut(&polity) {
                set.insert(region);
            } else {
                map.insert(polity, [region].into());
            }
        }
        map
    }

    pub fn update_can_split(&mut self, extras: &SimMapData) {
        self.split_tiles.retain(|x| !extras.city_borders.contains(x));
    }

    fn update_xywh(&mut self, config: &AtlasSimConfig) {
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
        self.centroid = Vec2::new(x as f32 + w as f32 / 2.0, y as f32 + h as f32 / 2.0);
    }

    fn update_chunk_coverage(&mut self, config: &AtlasSimConfig, tile: u32) -> (u32, u16) {
        let chunk = config.index_to_chunk(tile, config.deposits.chunk_size as u32);
        let coverage = if let Some(chunk) = self.resource_chunks.get_mut(&chunk) {
            *chunk += 1;
            *chunk
        } else {
            self.resource_chunks.insert(chunk, 1);
            1
        };
        (chunk, coverage)
    }

    fn update_deposits_from_chunk(&mut self, config: &AtlasSimConfig, chunk: u32, coverage: u16) {
        let chunk = &config.deposits.chunks[chunk as usize];
        let coverage = coverage as f32 / chunk.tile_count as f32;
        for (resource, amount) in &chunk.deposits {
            let amount = amount * coverage;
            if let Some(x) = self.deposits.get_mut(resource) {
                *x += amount;
            } else {
                self.deposits.insert(*resource, amount);
            }
        }
    }
}

pub fn spawn_region_with_city(
    region_entity: Entity,
    city_entity: Entity,
    region: Region,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    images: &mut Assets<Image>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    config: &AtlasSimConfig,
) {
    let city_pos = config.index_to_world_centered(region.city_position);
    commands.get_entity(city_entity).unwrap().insert((
        City,
        PbrBundle {
            mesh: meshes.add(PlaneMeshBuilder::new(Direction3d::Y, Vec2::ONE / 50.0)),
            material: materials.add(StandardMaterial {
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                base_color_texture: Some(asset_server.load("city.png")),
                ..Default::default()
            }),
            transform: Transform::from_xyz(city_pos.0, city_pos.1, 0.0002)
                .with_rotation(Quat::from_euler(EulerRot::XYZ, FRAC_PI_2, 0.0, 0.0))
                .with_scale(Vec3::new(0.8, 0.8, 0.8)),
            visibility: Visibility::Visible,
            ..Default::default()
        },
        MapOverlay::new(MapDataOverlay::Cities),
    ));
    let region_pos = config.centroid_to_world_centered(region.centroid.into());
    commands.get_entity(region_entity).unwrap().insert((
        region,
        PbrBundle {
            mesh: meshes.add(PlaneMeshBuilder::new(Direction3d::Y, Vec2::ONE).build()),
            material: materials.add(StandardMaterial {
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                base_color_texture: Some(images.add(Image::default())),
                ..Default::default()
            }),
            transform: Transform::from_xyz(region_pos.0, region_pos.1, 0.0001)
                .with_rotation(Quat::from_euler(EulerRot::XYZ, FRAC_PI_2, 0.0, 0.0))
                .with_scale(Vec3::new(0.01, 0.01, 0.01)),
            visibility: Visibility::Visible,
            ..Default::default()
        },
        MapOverlay::new(MapDataOverlay::Polities),
    ));
}
