use std::f32::consts::FRAC_PI_2;

use crate::{
    bevy::{
        prelude::*,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
        utils::HashMap,
    },
    config::load_image_grey,
    domain::map::{MapDataLayer, MAP_DATA_LAYERS},
};

pub const PREVIEW_NAME: &str = "preview.png";
pub const CLIMATEMAP_NAME: &str = "climatemap.png";
pub const CLIMATEMAP_SIZE: usize = 256;

// World map model tag.
#[derive(Component)]
pub struct WorldMapMesh;

// World globe model tag.
#[derive(Component)]
pub struct WorldGlobeMesh;

// Currently visible world model.
#[derive(Component)]
pub struct CurrentWorldModel;

#[derive(Resource)]
pub struct MapLogicData {
    layers: HashMap<MapDataLayer, Vec<u8>>,
    climatemap: Vec<u8>,
}

impl Default for MapLogicData {
    fn default() -> Self {
        Self {
            layers: Default::default(),
            climatemap: vec![0u8; CLIMATEMAP_SIZE * CLIMATEMAP_SIZE],
        }
    }
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

    pub fn get_climatemap(&self) -> &[u8] {
        &self.climatemap
    }

    pub fn load_climatemap(&mut self) -> Result<(), crate::config::Error> {
        let result = load_image_grey(CLIMATEMAP_NAME, CLIMATEMAP_SIZE as u32, CLIMATEMAP_SIZE as u32);
        if let Ok(vec) = result {
            self.climatemap = vec;
            Ok(())
        } else {
            result.map(|_| {})
        }
    }

    pub fn set_climatemap(&mut self, vec: Vec<u8>) {
        self.climatemap = vec;
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

/// Create a new texture.
pub fn make_image(width: u32, height: u32, data: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
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

/// Convert logical layer data to a texture.
/// For most cases, this just expands greyscale to grey RGBA.
pub fn data_to_view(data_layers: &MapLogicData, layer: MapDataLayer) -> Vec<u8> {
    let data = data_layers.get_layer(layer);
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
        MapDataLayer::Resources => expand_monochrome(data),
        MapDataLayer::RealTopography => expand_monochrome(data),
        MapDataLayer::TopographyFilter => expand_monochrome(data),
        MapDataLayer::Climate => expand_monochrome(data), // TODO fix for atlas sim
    }
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

/// Startup system
///
/// Initialize each map layer.
pub fn startup_layers(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut graphics: ResMut<MapGraphicsData>,
    mut logics: ResMut<MapLogicData>,
) {
    // Create the default texture and material.
    let empty_texture = make_image(1, 1, vec![0, 0, 0, 255]);
    let empty_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(empty_texture.clone())),
        unlit: true,
        ..default()
    });
    graphics.empty_material = empty_material;
    // Initialize all graphic and logical map layers.
    for layer in MAP_DATA_LAYERS {
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(empty_texture.clone())),
            unlit: true,
            ..default()
        });
        graphics
            .layers
            .insert(layer, MapGraphicsLayer::new(material.clone()));
        logics.put_layer(layer, vec![]);
    }
}

/// Spawn a default sphere for the globe world model.
pub fn spawn_default_globe(commands: &mut Commands, meshes: &mut Assets<Mesh>, graphics: &MapGraphicsData) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius: 2.0,
                    stacks: 180,
                    sectors: 360,
                }
                .into(),
            ),
            material: graphics.empty_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        WorldGlobeMesh,
    ));
}

/// Spawn a default plane for the flat world model.
pub fn spawn_default_plane(commands: &mut Commands, meshes: &mut Assets<Mesh>, graphics: &MapGraphicsData) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::default().into()),
            material: graphics.empty_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                FRAC_PI_2,
                0.0,
                0.0,
            )),
            ..Default::default()
        },
        WorldMapMesh,
        CurrentWorldModel,
    ));
}
