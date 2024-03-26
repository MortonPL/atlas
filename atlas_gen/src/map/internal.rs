use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, utils::HashMap};

use crate::map::ViewedMapLayer;

#[derive(Default, Resource)]
pub struct MapLogicData {
    pub layers: HashMap<ViewedMapLayer, Vec<u8>>,
}

#[derive(Default, Resource)]
pub struct MapGraphicsData {
    pub current: ViewedMapLayer,
    pub layers: HashMap<ViewedMapLayer, MapGraphicsLayer>,
    pub empty_material: Handle<StandardMaterial>,
}

impl MapGraphicsData {
    /// Access mutably one of the graphical layers.
    pub fn get_layer_mut(&mut self, layer: ViewedMapLayer) -> &mut MapGraphicsLayer {
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

/// Channels of an RGBA image.
enum RgbaChannel {
    Red,
    Green,
    Blue,
    #[allow(dead_code)]
    Alpha
}

/// Convert graphical representation of a map layer to a logical representation of the map layer.
/// The underlying conversion may differ based on layer variant.
///
/// This function is the inverse of [`magic_convert_data_to_png`].
pub fn magic_convert_png_to_data(data: Vec<u8>, layer: ViewedMapLayer) -> Vec<u8> {
    match layer {
        ViewedMapLayer::Pretty => data,
        ViewedMapLayer::Continents => continents_from_png(data),
        ViewedMapLayer::Topography => extract_rgba_channel(data, RgbaChannel::Green),
        ViewedMapLayer::Temperature => extract_rgba_channel(data, RgbaChannel::Red),
        ViewedMapLayer::Humidity => extract_rgba_channel(data, RgbaChannel::Blue),
        ViewedMapLayer::Climate => todo!(), // TODO
        ViewedMapLayer::Fertility => extract_rgba_channel(data, RgbaChannel::Green),
        ViewedMapLayer::Resource => todo!(), // TODO
        ViewedMapLayer::Richness => extract_rgba_channel(data, RgbaChannel::Blue),
    }
}

/// Convert an RGBA image to continents/ocean data.
/// Data: Value <= 127 is continent, value > 127 is ocean.
/// Image: Every ocean only has blue channel, every continent only green.
/// When both channels are set, continents take priority.
fn continents_from_png(data: Vec<u8>) -> Vec<u8> {
    let fun = |x: &[u8]| {
        if x[1] > 0 {
            x[1] / 2
        } else if x[2] > 0 {
            x[2] / 2
        } else {
            0
        }
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

/// Convert logical representation of a map layer to a graphical representation of the map layer.
/// The underlying conversion may differ based on layer variant.
///
/// This function is the inverse of [`magic_convert_png_to_data`].
pub fn magic_convert_data_to_png(data_layers: &MapLogicData, layer: ViewedMapLayer) -> Vec<u8> {
    let data = data_layers
        .layers
        .get(&layer)
        .expect("MapLogicData should map all layers");
    match layer {
        ViewedMapLayer::Pretty => data.to_vec(),
        ViewedMapLayer::Continents => continents_to_png(data),
        ViewedMapLayer::Topography => expand_rgba_channel(data, RgbaChannel::Green),
        ViewedMapLayer::Temperature => expand_rgba_channel(data, RgbaChannel::Red),
        ViewedMapLayer::Humidity => expand_rgba_channel(data, RgbaChannel::Blue),
        ViewedMapLayer::Climate => todo!(), // TODO
        ViewedMapLayer::Fertility => expand_rgba_channel(data, RgbaChannel::Green),
        ViewedMapLayer::Resource => todo!(), // TODO
        ViewedMapLayer::Richness => expand_rgba_channel(data, RgbaChannel::Blue),
    }
}

/// Expand one channel to an RGBA image.
fn expand_rgba_channel(data: &[u8], channel: RgbaChannel) -> Vec<u8> {
    let fun = match channel {
        RgbaChannel::Red => |x: &u8| [*x, 0, 0, 0],
        RgbaChannel::Green => |x: &u8| [0, *x, 0, 0],
        RgbaChannel::Blue => |x: &u8| [0, 0, *x, 0],
        RgbaChannel::Alpha => |x: &u8| [0, 0, 0, *x],
    };
    data.iter().map(fun).flatten().collect()
}

/// Convert continents/ocean data to an RGBA image.
/// Data: Value <= 127 is continent, value > 127 is ocean.
/// Image: Every ocean only has blue channel, every continent only green.
fn continents_to_png(data: &[u8]) -> Vec<u8> {
    let fun = |x: &u8| {
        if *x <= 127 {
            [0, x * 2, 0, 0]
        } else {
            [0, 0, (x - 128) * 2, 0]
        }
    };
    data.iter().map(fun).flatten().collect()
}
