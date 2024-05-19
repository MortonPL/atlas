use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    config::{load_image_grey, AtlasGenConfig, BiomeConfig},
    map::MapDataLayer,
};

#[derive(Default, Resource)]
pub struct MapLogicData {
    layers: HashMap<MapDataLayer, Vec<u8>>,
    climatemap: Option<Vec<u8>>,
}

impl MapLogicData {
    pub fn get_layer(&self, layer: MapDataLayer) -> &[u8] {
        self.layers
            .get(&layer)
            .expect("MapLogicData should map all layers")
    }

    pub fn get_layer_mut(&mut self, layer: MapDataLayer) -> &mut [u8] {
        self.layers
            .get_mut(&layer)
            .expect("MapLogicData should map all layers")
    }

    pub fn pop_layer(&mut self, layer: MapDataLayer) -> Vec<u8> {
        self.layers
            .remove(&layer)
            .expect("MapLogicData should map all layers")
    }

    pub fn put_layer(&mut self, layer: MapDataLayer, data: Vec<u8>) {
        self.layers.insert(layer, data);
    }

    pub fn resize_all_layers(&mut self, size: usize) {
        for (layer, data) in self.layers.iter_mut() {
            let bpp = match layer {
                MapDataLayer::Preview => 4,
                _ => 1,
            };
            data.resize(size * bpp, 0);
        }
    }

    pub fn get_climatemap(&self) -> Option<&[u8]> {
        self.climatemap.as_deref()
    }

    pub fn load_climatemap(&mut self) {
        self.climatemap = load_image_grey("climatemap.png", 255, 255).ok();
    }
}

#[derive(Default, Resource)]
pub struct MapGraphicsData {
    pub current: MapDataLayer,
    pub layers: HashMap<MapDataLayer, MapGraphicsLayer>,
    pub empty_material: Handle<StandardMaterial>,
}

impl MapGraphicsData {
    /// Access mutably one of the graphical layers.
    pub fn get_layer_mut(&mut self, layer: MapDataLayer) -> &mut MapGraphicsLayer {
        self.layers
            .get_mut(&layer)
            .expect("MapGraphicsData should map all layers")
    }
}

#[derive(Default)]
pub struct MapGraphicsLayer {
    pub material: Handle<StandardMaterial>,
    pub invalid: bool,
}

impl MapGraphicsLayer {
    pub fn new(material: Handle<StandardMaterial>) -> Self {
        Self {
            material,
            invalid: true,
        }
    }
}

// World map model tag.
#[derive(Component)]
pub struct WorldMapMesh;

// World globe model tag.
#[derive(Component)]
pub struct WorldGlobeMesh;

// Currently visible world model.
#[derive(Component)]
pub struct CurrentWorldModel;

/// Spawn a default sphere for the globe world model.
pub fn spawn_default_globe(commands: &mut Commands, meshes: &mut Assets<Mesh>, graphics: &MapGraphicsData) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: graphics.empty_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        WorldGlobeMesh,
    ));
}

/// SPawn a default plane for the flat world model.
pub fn spawn_default_plane(commands: &mut Commands, meshes: &mut Assets<Mesh>, graphics: &MapGraphicsData) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::default().into()),
            material: graphics.empty_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                FRAC_PI_2,
                FRAC_PI_2,
                0.0,
            )),
            ..Default::default()
        },
        WorldMapMesh,
        CurrentWorldModel,
    ));
}

/// Create a new texture.
pub fn make_image(width: u32, height: u32, data: Vec<u8>) -> Image {
    Image::new(
        bevy::render::render_resource::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
    )
}

/// Access a material.
pub fn get_material<'a>(
    materials: &'a Assets<StandardMaterial>,
    handle: &Handle<StandardMaterial>,
) -> &'a StandardMaterial {
    materials.get(handle).expect("Material handle should be valid")
}

/// Access a material.
pub fn get_material_mut<'a>(
    materials: &'a mut Assets<StandardMaterial>,
    handle: &Handle<StandardMaterial>,
) -> &'a mut StandardMaterial {
    materials
        .get_mut(handle)
        .expect("Material handle should be valid")
}

/// Import layer data from a PNG file.
/// The underlying conversion may differ based on layer variant.
///
/// This function is the inverse of [`data_to_png`].
pub fn png_to_data(data: Vec<u8>, layer: MapDataLayer) -> Vec<u8> {
    match layer {
        MapDataLayer::Preview => data,
        MapDataLayer::Continents => continents_from_png(data),
        MapDataLayer::ContinentsInfluence => extract_monochrome(data),
        MapDataLayer::Topography => extract_monochrome(data),
        MapDataLayer::TopographyInfluence => extract_monochrome(data),
        MapDataLayer::Temperature => extract_monochrome(data),
        MapDataLayer::TemperatureInfluence => extract_monochrome(data),
        MapDataLayer::Precipitation => extract_monochrome(data),
        MapDataLayer::PrecipitationInfluence => extract_monochrome(data),
        MapDataLayer::Climate => extract_monochrome(data),
        MapDataLayer::Fertility => todo!(), // TODO
        MapDataLayer::Resource => todo!(),  // TODO
        MapDataLayer::Richness => todo!(),  // TODO
        MapDataLayer::RealTopography => extract_monochrome(data),
        MapDataLayer::TopographyFilter => extract_monochrome(data),
    }
}

/// Export layer data to a PNG file.
/// The underlying conversion may differ based on layer variant.
///
/// This function is the inverse of [`png_to_data`].
pub fn data_to_png(data_layers: &MapLogicData, layer: MapDataLayer) -> Vec<u8> {
    let data = data_layers
        .layers
        .get(&layer)
        .expect("MapLogicData should map all layers");
    match layer {
        MapDataLayer::Preview => data.to_vec(),
        MapDataLayer::Continents => continents_to_png(data),
        MapDataLayer::ContinentsInfluence => expand_monochrome(data),
        MapDataLayer::Topography => expand_monochrome(data),
        MapDataLayer::TopographyInfluence => expand_monochrome(data),
        MapDataLayer::Temperature => expand_monochrome(data),
        MapDataLayer::TemperatureInfluence => expand_monochrome(data),
        MapDataLayer::Precipitation => expand_monochrome(data),
        MapDataLayer::PrecipitationInfluence => expand_monochrome(data),
        MapDataLayer::Climate => expand_monochrome(data),
        MapDataLayer::Fertility => todo!(), // TODO
        MapDataLayer::Resource => todo!(),  // TODO
        MapDataLayer::Richness => todo!(),  // TODO
        MapDataLayer::RealTopography => expand_monochrome(data),
        MapDataLayer::TopographyFilter => expand_monochrome(data),
    }
}

/// Convert logical layer data to a texture.
/// For most cases, this functions the same as [`data_to_png`].
pub fn data_to_view(data_layers: &MapLogicData, layer: MapDataLayer, config: &AtlasGenConfig) -> Vec<u8> {
    let data = data_layers
        .layers
        .get(&layer)
        .expect("MapLogicData should map all layers");
    match layer {
        MapDataLayer::Climate => climate_to_view(data, config),
        _ => data_to_png(data_layers, layer),
    }
}

/// Convert an RGBA image to continents/ocean data.
/// Data: Value <= 127 is ocean, value > 127 is continent.
/// Image: Every ocean only has blue channel, every continent only green.
/// When both channels are set, continents take priority.
fn continents_from_png(data: Vec<u8>) -> Vec<u8> {
    let fun = |x: &[u8]| {
        if x[1] > 0 {
            x[1] / 2 + 128
        } else if x[2] > 0 {
            x[2] / 2
        } else {
            255
        }
    };
    data.chunks_exact(4).map(fun).collect()
}

/// Extract one channel from an RGBA image.
fn extract_monochrome(data: Vec<u8>) -> Vec<u8> {
    data.into_iter().step_by(4).collect()
}

/// Convert continents/ocean data to an RGBA image.
/// Data: Value <= 127 is ocean, value > 127 is continent.
/// Image: Every ocean only has blue channel, every continent only green.
fn continents_to_png(data: &[u8]) -> Vec<u8> {
    let fun = |x: &u8| {
        if *x <= 127 {
            [0, 0, 1 + x * 2, 255]
        } else {
            [0, (x - 128) * 2 + 1, 0, 255]
        }
    };
    data.iter().flat_map(fun).collect()
}

/// Expand one channel to an RGBA image.
fn expand_monochrome(data: &[u8]) -> Vec<u8> {
    data.iter().flat_map(|x: &u8| [*x, *x, *x, 255]).collect()
}

pub fn fetch_climate(i: usize, config: &AtlasGenConfig) -> &BiomeConfig {
    if i > config.climate.biomes.len() {
        &config.climate.default_biome
    } else {
        &config.climate.biomes[i]
    }
}

fn climate_to_view(data: &[u8], config: &AtlasGenConfig) -> Vec<u8> {
    let fun = |x: &u8| {
        let climate = fetch_climate(*x as usize, config);
        [climate.color[0], climate.color[1], climate.color[2], 255]
    };
    data.iter().flat_map(fun).collect()
}
