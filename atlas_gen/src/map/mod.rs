use std::f32::consts::FRAC_PI_2;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};

use crate::config::{GeneratorConfig, WorldModel};

/// Plugin responsible for the world map graphics.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapGraphicsData>()
            .add_systems(Startup, startup_graphics)
            .add_systems(Startup, startup_model.after(startup_graphics))
            .add_systems(Update, update_validate_layers)
            .add_systems(Update, update.after(update_validate_layers));
    }
}

#[derive(Default, Resource)]
pub struct MapGraphicsData {
    pub current: ViewedMapLayer,
    pub previous: ViewedMapLayer,
    pub layer_none: MapGraphicsLayer,
    pub layer_cont: MapGraphicsLayer,
    pub layer_topo: MapGraphicsLayer,
    pub layer_climate: MapGraphicsLayer,
    pub layer_all: MapGraphicsLayer,
}

#[derive(Default)]
pub struct MapGraphicsLayer {
    pub material: Handle<StandardMaterial>,
    pub image: Handle<Image>,
    pub invalidated: bool,
    pub outdated: bool,
}

impl MapGraphicsLayer {
    pub fn new(material: Handle<StandardMaterial>, image: Handle<Image>) -> Self {
        Self {
            material,
            image,
            invalidated: false,
            outdated: false,
        }
    }
}

#[derive(Default, Clone, PartialEq, Copy)]
pub enum ViewedMapLayer {
    #[default]
    None,
    Continental,
    Topograpy,
    Climate,
    All,
}

// World map model tag.
#[derive(Component)]
pub struct WorldMapMesh;

// World globe model tag.
#[derive(Component)]
pub struct WorldGlobeMesh;

/// Startup system
///
/// Spawn the map and globe world models.
fn startup_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: ResMut<MapGraphicsData>,
) {
    let material = graphics.layer_none.material.clone();

    // Sphere / globe
    let mesh = meshes.add(shape::UVSphere::default().into());
    commands.spawn((
        PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        WorldGlobeMesh,
    ));
    // Plane / map
    let mesh = meshes.add(shape::Plane::default().into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                FRAC_PI_2,
                FRAC_PI_2,
                0.0,
            )),
            ..Default::default()
        },
        WorldMapMesh,
    ));
}

/// Startup system
///
/// Create materials for each map layer.
fn startup_graphics(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut graphics: ResMut<MapGraphicsData>,
) {
    let image = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );
    let image = images.add(image);
    let material_base = StandardMaterial {
        base_color_texture: Some(image.clone()),
        ..Default::default()
    };
    let material = materials.add(material_base.clone());
    graphics.layer_none = MapGraphicsLayer::new(material.clone(), image.clone());
    let material = materials.add(material_base.clone());
    graphics.layer_cont = MapGraphicsLayer::new(material.clone(), image.clone());
    let material = materials.add(material_base.clone());
    graphics.layer_topo = MapGraphicsLayer::new(material.clone(), image.clone());
    let material = materials.add(material_base.clone());
    graphics.layer_climate = MapGraphicsLayer::new(material.clone(), image.clone());
    let material = materials.add(material_base);
    graphics.layer_all = MapGraphicsLayer::new(material, image);
    graphics.layer_none.invalidated = false;
}

/// Update system
///
/// Display map or globe model depending on configuration.
fn update(
    config: Res<GeneratorConfig>,
    mut map: Query<
        (
            &mut Visibility,
            &mut Transform,
            &mut Handle<StandardMaterial>,
        ),
        With<WorldMapMesh>,
    >,
    mut globe: Query<
        (&mut Visibility, &mut Handle<StandardMaterial>),
        (With<WorldGlobeMesh>, Without<WorldMapMesh>),
    >,
    mut graphics: ResMut<MapGraphicsData>,
) {
    let (mut map_vis, mut map_tran, mut map_mat) = map.single_mut();
    let (mut globe_vis, mut globe_mat) = globe.single_mut();

    let layer = match graphics.current {
        ViewedMapLayer::None => &graphics.layer_none,
        ViewedMapLayer::Continental => &graphics.layer_cont,
        ViewedMapLayer::Topograpy => &graphics.layer_topo,
        ViewedMapLayer::Climate => &graphics.layer_climate,
        ViewedMapLayer::All => &graphics.layer_all,
    };

    match &config.general.world_model {
        WorldModel::Flat(x) => {
            *map_vis = Visibility::Visible;
            *globe_vis = Visibility::Hidden;
            map_tran.scale.x = x.world_size[0] as f32 / 100.0;
            map_tran.scale.z = x.world_size[1] as f32 / 100.0;
            if layer.invalidated {
                *map_mat = graphics.layer_none.material.clone();
                dbg!("Layer invalidated without recovery");
            } else if graphics.current != graphics.previous {
                *map_mat = layer.material.clone();
                dbg!("Changing layers");
            }
        }
        WorldModel::Globe(_) => {
            *map_vis = Visibility::Hidden;
            *globe_vis = Visibility::Visible;
            if graphics.current != graphics.previous {
                *globe_mat = layer.material.clone();
            }
        }
    };
    graphics.previous = graphics.current;
}

/// Update system
///
/// Recreate material textures if they have been invalidated.
pub fn update_validate_layers(
    mut graphics: ResMut<MapGraphicsData>,
    config: ResMut<GeneratorConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    if graphics.layer_cont.invalidated {
        dbg!("Invalidating");
        let (width, height) = config.general.world_model.get_dimensions();
        let image = Image::new_fill(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            &[0, 0, 0, 255],
            TextureFormat::Rgba8Unorm,
        );
        let image = images.add(image);
        graphics.layer_cont.image = image.clone();
        let material = materials.get_mut(&graphics.layer_cont.material).unwrap();
        material.base_color_texture = Some(image);

        graphics.layer_cont.invalidated = false;
        graphics.layer_cont.outdated = true;
    } else if graphics.layer_cont.outdated {
        let image = images.get_mut(&graphics.layer_cont.image).unwrap();
        image.data = config.continents.data.clone();
        let material = materials.get_mut(&graphics.layer_cont.material).unwrap();
        material.base_color_texture = Some(graphics.layer_cont.image.clone());

        graphics.layer_cont.outdated = false;
    }
}
