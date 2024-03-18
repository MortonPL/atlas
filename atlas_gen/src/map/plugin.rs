use std::f32::consts::FRAC_PI_2;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};

use atlas_lib::UiConfigurableEnum;

use crate::{
    config::{GeneratorConfig, WorldModel},
    event::EventStruct,
    map::internal::{
        CurrentWorldModel, MapGraphicsData, MapGraphicsLayer, MapLogicData, WorldGlobeMesh,
        WorldMapMesh,
    },
};

/// Plugin responsible for the world map graphics.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapGraphicsData>()
            .init_resource::<MapLogicData>()
            .add_systems(Startup, startup_materials)
            .add_systems(Startup, startup_model.after(startup_materials))
            .add_systems(
                Update,
                update_event_world_model.run_if(check_event_world_model),
            )
            .add_systems(
                Update,
                update_event_layer_changed.run_if(check_event_layer_changed),
            )
            .add_systems(Update, update_validate_layers);
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
    mut logics: ResMut<MapLogicData>,
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
    graphics.empty_material = materials.add(material_base.clone());

    for layer in [
        Continents,
        Topography,
        Temperature,
        Humidity,
        Climate,
        Fertility,
        Resource,
        Richness,
    ] {
        let material = materials.add(material_base.clone());
        graphics.layers.insert(
            layer,
            MapGraphicsLayer::new(material.clone(), image.clone()),
        );
        logics.layers.insert(layer, vec![]);
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
    let layer = graphics
        .layers
        .get(&ViewedMapLayer::default())
        .expect("Uninitialized map layer materials");

    // Sphere / globe
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: layer.material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        WorldGlobeMesh,
        CurrentWorldModel,
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

/// Run Condition
///
/// Check if "change world model" UI event needs handling.
fn check_event_world_model(events: Res<EventStruct>) -> bool {
    events.world_model_changed.is_some()
}

/// Update system
///
/// Handle "change world model" UI event.
fn update_event_world_model(
    mut commands: Commands,
    mut events: ResMut<EventStruct>,
    mut map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    mut globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
) {
    let (map_en, mut map_vis, mut map_tran) = map.single_mut();
    let (globe_en, mut globe_vis) = globe.single_mut();

    if let Some(model) = &events.world_model_changed {
        match model {
            WorldModel::Flat(x) => {
                *map_vis = Visibility::Visible;
                *globe_vis = Visibility::Hidden;
                map_tran.scale.x = x.world_size[0] as f32 / 100.0; // TODO invalidate material, but only if world_size changed
                map_tran.scale.z = x.world_size[1] as f32 / 100.0;
                commands.entity(map_en).insert(CurrentWorldModel);
                commands.entity(globe_en).remove::<CurrentWorldModel>();
            }
            WorldModel::Globe(_) => {
                *map_vis = Visibility::Hidden;
                *globe_vis = Visibility::Visible;
                commands.entity(globe_en).insert(CurrentWorldModel);
                commands.entity(map_en).remove::<CurrentWorldModel>();
            }
        }
    }

    events.world_model_changed = None;
}

/// Run Condition
///
/// Check if "change viewed layer" UI event needs handling.
fn check_event_layer_changed(events: Res<EventStruct>) -> bool {
    events.viewed_layer_changed.is_some()
}

/// Update system
///
/// Assign respective layer material to the world model.
fn update_event_layer_changed(
    mut events: ResMut<EventStruct>,
    graphics: Res<MapGraphicsData>,
    mut world: Query<&mut Handle<StandardMaterial>, With<CurrentWorldModel>>,
) {
    let mut mat = world.single_mut();
    if let Some(layer) = events.viewed_layer_changed {
        let layer = graphics
            .layers
            .get(&layer)
            .expect("Uninitialized map layer materials");
        *mat = layer.material.clone();
    }

    events.viewed_layer_changed = None;
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
