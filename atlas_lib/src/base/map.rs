use std::{f32::consts::FRAC_PI_2, marker::PhantomData};

use bevy::{
    app::{Plugin, Startup},
    prelude::*,
    render::mesh::{PlaneMeshBuilder, SphereMeshBuilder},
};

use crate::{
    base::{
        events::EventStruct,
        ui::{UiStateBase, UiUpdate},
    },
    config::{AtlasConfig, WorldModel},
    domain::{
        graphics::{
            data_to_view, get_material_mut, make_image, CurrentWorldModel, MapGraphicsData, MapGraphicsLayer,
            MapLogicData, WorldGlobeMesh, WorldMapMesh,
        },
        map::MAP_DATA_LAYERS,
    },
};

#[derive(Default)]
pub struct MapPluginBase<C: AtlasConfig> {
    __: PhantomData<C>,
}

impl<C: AtlasConfig> Plugin for MapPluginBase<C> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MapGraphicsData>()
            .init_resource::<MapLogicData>()
            .add_systems(Startup, (startup_layers, startup_model).chain())
            .add_systems(
                Update,
                (
                    update_event_changed.run_if(check_event_changed),
                    update_event_world_model::<C>.run_if(check_event_world_model),
                    update_event_regen::<C>.run_if(check_event_regen),
                ),
            )
            .add_systems(UiUpdate, update_model_rotation::<C>);
    }
}

/// Startup system
///
/// Initialize each map layer.
fn startup_layers(
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

/// Startup system
///
/// Spawn the map and globe world models.
fn startup_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: ResMut<MapGraphicsData>,
    mut events: ResMut<EventStruct>,
) {
    spawn_default_globe(&mut commands, &mut meshes, &graphics);
    spawn_default_plane(&mut commands, &mut meshes, &graphics);
    // Trigger model change.
    events.world_model_changed = Some(());
}

/// Spawn a default sphere for the globe world model.
fn spawn_default_globe(commands: &mut Commands, meshes: &mut Assets<Mesh>, graphics: &MapGraphicsData) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                SphereMeshBuilder::new(
                    2.0,
                    bevy::render::mesh::SphereKind::Uv {
                        sectors: 360,
                        stacks: 180,
                    },
                )
                .build(),
            ),
            material: graphics.empty_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        WorldGlobeMesh,
    ));
}

/// Spawn a default plane for the flat world model.
fn spawn_default_plane(commands: &mut Commands, meshes: &mut Assets<Mesh>, graphics: &MapGraphicsData) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(PlaneMeshBuilder::new(Direction3d::Y, Vec2::ONE)),
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

/// Run Condition
///
/// Check if "change world model" UI event needs handling.
fn check_event_world_model(events: Res<EventStruct>) -> bool {
    events.world_model_changed.is_some()
}

/// Run Condition
///
/// Check if "change viewed layer" UI event needs handling.
fn check_event_changed(events: Res<EventStruct>) -> bool {
    events.viewed_layer_changed.is_some()
}

/// Run condition
///
/// Check if "regen layer image" event needs handling.
fn check_event_regen(events: Res<EventStruct>) -> bool {
    events.regen_layer_request.is_some()
}

/// Update system
///
/// Update rotation mode and apply transformations.
fn update_model_rotation<C: AtlasConfig>(
    mut model: Query<&mut Transform, With<CurrentWorldModel>>,
    config: Res<C>,
    mut ui_base: ResMut<UiStateBase>,
) {
    ui_base.camera.rotate_mode = match config.get_preview_model() {
        WorldModel::Flat => false,
        WorldModel::Globe => true,
    };
    if ui_base.camera.rotate_mode {
        let mut transform = model.single_mut();
        transform.rotation = ui_base.camera.rotation;
    }
}

/// Update system
///
/// Regenerate graphical layer based on logical layer data.
fn update_event_regen<C: AtlasConfig>(
    mut events: ResMut<EventStruct>,
    config: Res<C>,
    mut graphics: ResMut<MapGraphicsData>,
    logics: Res<MapLogicData>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layers = events.regen_layer_request.take().expect("Always Some");
    for layer in layers {
        // Convert logical data to image data.
        let mut data = data_to_view(&logics, layer, config.as_ref());
        // Fetch handles.
        let layer = graphics.get_layer_mut(layer);
        let material = get_material_mut(&mut materials, &layer.material);
        // Assign new texture.
        let (width, height) = config.get_world_size();
        let image = images.add(make_image(width, height, std::mem::take(&mut data)));
        material.base_color_texture = Some(image);
        // Graphical layer becomes valid again.
        layer.invalid = false;
    }
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}

/// Update system
///
/// Handle "change world model" UI event.
fn update_event_world_model<C: AtlasConfig>(
    commands: Commands,
    mut events: ResMut<EventStruct>,
    config: Res<C>,
    map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    graphics: Res<MapGraphicsData>,
    logics: ResMut<MapLogicData>,
) {
    events.world_model_changed = None;
    resize_helper(commands, config.as_ref(), map, globe, logics);
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}

/// Update system
///
/// Assign respective layer material to the world model.
fn update_event_changed(
    mut events: ResMut<EventStruct>,
    mut graphics: ResMut<MapGraphicsData>,
    mut world: Query<&mut Handle<StandardMaterial>, With<CurrentWorldModel>>,
) {
    // Set layer as current.
    let layer = events.viewed_layer_changed.take().expect("Always Some");
    graphics.current = layer;
    // Change worls model's material to this layer's material.
    let layer = graphics.get_layer_mut(layer);
    let mut mat = world.single_mut();
    *mat = if layer.invalid {
        graphics.empty_material.clone()
    } else {
        layer.material.clone()
    };
}

/// Helper function
///
/// Switch and resize world models.
pub fn resize_helper(
    mut commands: Commands,
    config: &impl AtlasConfig,
    mut map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    mut globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    mut logics: ResMut<MapLogicData>,
) {
    // Run queries.
    let (map_en, mut map_vis, mut map_tran) = map.single_mut();
    let (globe_en, mut globe_vis) = globe.single_mut();
    let (width, height) = config.get_world_size();
    logics.resize_all_layers((width * height) as usize);
    match config.get_preview_model() {
        WorldModel::Flat => {
            *map_vis = Visibility::Visible;
            *globe_vis = Visibility::Hidden;
            map_tran.scale.x = width as f32 / 100.0;
            map_tran.scale.z = height as f32 / 100.0;
            commands.entity(map_en).insert(CurrentWorldModel);
            commands.entity(globe_en).remove::<CurrentWorldModel>();
        }
        WorldModel::Globe => {
            *map_vis = Visibility::Hidden;
            *globe_vis = Visibility::Visible;
            commands.entity(globe_en).insert(CurrentWorldModel);
            commands.entity(map_en).remove::<CurrentWorldModel>();
        }
    }
}
