use std::f32::consts::FRAC_PI_2;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};

use atlas_lib::UiEditableEnum;

use crate::{
    config::{save_image, GeneratorConfig, WorldModel},
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
            .add_systems(
                Update,
                update_event_layer_loaded.run_if(check_event_layer_loaded),
            )
            .add_systems(
                Update,
                update_event_layer_saved.run_if(check_event_layer_saved),
            );
    }
}

/// Which layer is currently visible in the viewport.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, UiEditableEnum)]
pub enum ViewedMapLayer {
    #[default]
    Pretty,
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
    let empty_texture = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );
    graphics.empty_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(empty_texture.clone())),
        ..default()
    });

    use ViewedMapLayer::*;
    for layer in [
        Pretty,
        Continents,
        Topography,
        Temperature,
        Humidity,
        Climate,
        Fertility,
        Resource,
        Richness,
    ] {
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(empty_texture.clone())),
            ..default()
        });
        graphics
            .layers
            .insert(layer, MapGraphicsLayer::new(material.clone()));
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
    config: Res<GeneratorConfig>,
    mut events: ResMut<EventStruct>,
) {
    // Sphere / globe
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: graphics.empty_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        WorldGlobeMesh,
    ));
    // Plane / map
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

    events.world_model_changed = Some(config.general.world_model.clone());
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
    mut graphics: ResMut<MapGraphicsData>,
) {
    let (map_en, mut map_vis, mut map_tran) = map.single_mut();
    let (globe_en, mut globe_vis) = globe.single_mut();

    if let Some(model) = &events.world_model_changed {
        match model {
            WorldModel::Flat(x) => {
                *map_vis = Visibility::Visible;
                *globe_vis = Visibility::Hidden;
                map_tran.scale.x = x.world_size[0] as f32 / 100.0;
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
        for layer in graphics.layers.values_mut() {
            layer.invalid = true;
        }
        events.viewed_layer_changed = Some(graphics.current);
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
    mut graphics: ResMut<MapGraphicsData>,
    mut world: Query<&mut Handle<StandardMaterial>, With<CurrentWorldModel>>,
) {
    let mut mat = world.single_mut();
    if let Some(layer) = events.viewed_layer_changed {
        graphics.current = layer;
        let layer = graphics
            .layers
            .get(&layer)
            .expect("MapGraphicsData should map all layers");
        if layer.invalid {
            *mat = graphics.empty_material.clone();
        } else {
            *mat = layer.material.clone();
        }
    }

    events.viewed_layer_changed = None;
}

/// Run condition
///
/// Check if "load layer image" event needs handling.
fn check_event_layer_loaded(events: Res<EventStruct>) -> bool {
    events.load_layer_request.is_some()
}

/// Update system
///
/// Load new layer data. TODO convert between logical layer data and graphical layer data.
fn update_event_layer_loaded(
    mut events: ResMut<EventStruct>,
    config: ResMut<GeneratorConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some((layer, data)) = &mut events.load_layer_request {
        let layer = graphics
            .layers
            .get_mut(layer)
            .expect("MapGraphicsData should map all layers");
        let material = materials
            .get_mut(&layer.material)
            .expect("Material handle should be valid");
        let (width, height) = config.general.world_model.get_dimensions();
        let image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            data.drain(..).collect(),
            TextureFormat::Rgba8Unorm,
        );
        let image = images.add(image);
        material.base_color_texture = Some(image);
        layer.invalid = false;
        events.viewed_layer_changed = Some(graphics.current);
    }

    events.load_layer_request = None;
}

/// Run condition
///
/// Check if "save layer image" event needs handling.
fn check_event_layer_saved(events: Res<EventStruct>) -> bool {
    events.save_layer_request.is_some()
}

/// Update system
///
/// Save new layer data.
fn update_event_layer_saved(
    mut events: ResMut<EventStruct>,
    config: ResMut<GeneratorConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some((layer, path)) = &mut events.save_layer_request {
        let layer = graphics
            .layers
            .get_mut(layer)
            .expect("MapGraphicsData should map all layers");

        if layer.invalid {
            events.save_layer_request = None;
            return;  // TODO handle nicely
        }

        let material = materials
            .get_mut(&layer.material)
            .expect("Material handle should be valid");
        let (width, height) = config.general.world_model.get_dimensions();
        if let Some(image) = &material.base_color_texture {
            let data = &images.get(image).expect("").data;
            save_image(path, data, width, height).unwrap(); // TODO error handling
        }
    }

    events.save_layer_request = None;
}
