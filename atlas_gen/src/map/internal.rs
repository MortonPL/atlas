use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, utils::HashMap};

use crate::map::MapDataLayer;

#[derive(Default, Resource)]
pub struct MapLogicData {
    layers: HashMap<MapDataLayer, Vec<u8>>,
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
        bevy::render::render_resource::TextureFormat::Rgba8Unorm,
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

/// Convert graphical representation of a map layer to a logical representation of the map layer.
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
        MapDataLayer::Humidity => extract_monochrome(data),
        MapDataLayer::HumidityInfluence => extract_monochrome(data),
        MapDataLayer::Climate => climate_from_png(data),
        MapDataLayer::Fertility => todo!(), // TODO
        MapDataLayer::Resource => todo!(),  // TODO
        MapDataLayer::Richness => todo!(),  // TODO
        MapDataLayer::RealTopography => extract_monochrome(data),
        MapDataLayer::TopographyFilter => extract_monochrome(data),
    }
}

/// Convert logical representation of a map layer to a graphical representation of the map layer.
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
        MapDataLayer::Humidity => expand_monochrome(data),
        MapDataLayer::HumidityInfluence => expand_monochrome(data),
        MapDataLayer::Climate => climate_to_png(data),
        MapDataLayer::Fertility => todo!(), // TODO
        MapDataLayer::Resource => todo!(),  // TODO
        MapDataLayer::Richness => todo!(),  // TODO
        MapDataLayer::RealTopography => expand_monochrome(data),
        MapDataLayer::TopographyFilter => expand_monochrome(data),
    }
}

/// Expand climate layer data to climate's assigned color.
pub fn climate_to_hsv(data: u8) -> (f32, f32, f32) {
    (0.3, 0.9, 0.9) // TODO
}

/// Channels of an RGBA image.
enum RgbaChannel {
    Red,
    Green,
    Blue,
    #[allow(dead_code)]
    Alpha,
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

fn climate_from_png(data: Vec<u8>) -> Vec<u8> {
    let fun = |x: &[u8]| match (x[0], x[1], x[2]) {
        _ => 0, // TODO
    };
    data.chunks_exact(4).map(fun).collect()
}

/// Extract one channel from an RGBA image.
fn extract_rgba_channel(data: Vec<u8>, channel: RgbaChannel) -> Vec<u8> {
    let offset = match channel {
        RgbaChannel::Red => 0,
        RgbaChannel::Green => 1,
        RgbaChannel::Blue => 2,
        RgbaChannel::Alpha => 3,
    };
    data.into_iter().skip(offset).step_by(4).collect()
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

fn climate_to_png(data: &[u8]) -> Vec<u8> {
    let fun = |x: &u8| [0, 0, 0, 255]; // TODO
    data.iter().flat_map(fun).collect()
}

/// Expand one channel to an RGBA image.
fn expand_rgba_channel(data: &[u8], channel: RgbaChannel) -> Vec<u8> {
    let fun = match channel {
        RgbaChannel::Red => |x: &u8| [*x, 0, 0, 255],
        RgbaChannel::Green => |x: &u8| [0, *x, 0, 255],
        RgbaChannel::Blue => |x: &u8| [0, 0, *x, 255],
        RgbaChannel::Alpha => |x: &u8| [0, 0, 0, *x],
    };
    data.iter().flat_map(fun).collect()
}

/// Expand one channel to an RGBA image.
fn expand_monochrome(data: &[u8]) -> Vec<u8> {
    data.iter().flat_map(|x: &u8| [*x, *x, *x, 255]).collect()
}
