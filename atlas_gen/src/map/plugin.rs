use std::f32::consts::FRAC_PI_2;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};

use atlas_lib::UiConfigurableEnum;

use crate::{
    config::{GeneratorConfig, WorldModel},
    map::internal::{MapGraphicsData, MapGraphicsLayer, WorldGlobeMesh, WorldMapMesh},
};

/// Plugin responsible for the world map graphics.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapGraphicsData>()
            .add_systems(Startup, startup_materials)
            .add_systems(Startup, startup_model.after(startup_materials))
            .add_systems(Update, update_validate_layers)
            .add_systems(Update, update.after(update_validate_layers));
    }
}

/// Which layer is currently visible in the viewport.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Resource, UiConfigurableEnum)]
pub enum ViewedMapLayer {
    #[default]
    Continents,
    Topography,
    Temperature,
    Humidity,
    Climate,
    Fertility,
    Resource,
    Richness,
}

/// Startup system
///
/// Create materials for each map layer.
fn startup_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut graphics: ResMut<MapGraphicsData>,
) {
    use ViewedMapLayer::*;

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

    for layer in [Continents, Topography, Temperature, Humidity, Climate, Fertility, Resource, Richness] {
        let material = materials.add(material_base.clone());
        graphics.layers.insert(layer, MapGraphicsLayer::new(material.clone(), image.clone()));
    }
}

/// Startup system
///
/// Spawn the map and globe world models.
fn startup_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: ResMut<MapGraphicsData>,
) {
    let layer = graphics.layers.get(&ViewedMapLayer::default()).expect("Uninitialized map layer materials");

    // Sphere / globe
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: layer.material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        WorldGlobeMesh,
    ));
    // Plane / map
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::default().into()),
            material: layer.material.clone(),
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
    let layer = graphics.layers.get(&graphics.current).expect("Uninitialized map layer materials");

    match &config.general.world_model {
        WorldModel::Flat(x) => {
            *map_vis = Visibility::Visible;
            *globe_vis = Visibility::Hidden;
            map_tran.scale.x = x.world_size[0] as f32 / 100.0;
            map_tran.scale.z = x.world_size[1] as f32 / 100.0;
            if layer.invalidated {
                //*map_mat = graphics.layer_none.material.clone();
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
    /*
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
        //image.data = config.continents.data.clone();
        let material = materials.get_mut(&graphics.layer_cont.material).unwrap();
        material.base_color_texture = Some(graphics.layer_cont.image.clone());

        graphics.layer_cont.outdated = false;
    }
    */
}
