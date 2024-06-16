use atlas_lib::{
    bevy::prelude::*,
    config::{
        load_config, load_image, load_image_grey, save_config, save_image, save_image_grey, WorldModel,
    },
    domain::{
        graphics::{
            get_material_mut, make_image, CurrentWorldModel, MapGraphicsData, MapLogicData, WorldGlobeMesh,
            WorldMapMesh, PREVIEW_NAME,
        },
        map::{MapDataLayer, EXPORT_DATA_LAYERS},
    },
};

use crate::{
    config::AtlasSimConfig,
    event::EventStruct,
    map::internal::{data_to_view, CONFIG_NAME},
};

/// Run Condition
///
/// Check if "change world model" UI event needs handling.
pub fn check_event_world_model(events: Res<EventStruct>) -> bool {
    events.world_model_changed.is_some()
}

/// Update system
///
/// Handle "change world model" UI event.
pub fn update_event_world_model(
    commands: Commands,
    mut events: ResMut<EventStruct>,
    config: Res<AtlasSimConfig>,
    map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    graphics: Res<MapGraphicsData>,
    logics: ResMut<MapLogicData>,
) {
    events.world_model_changed = None;
    resize_helper(commands, &config, map, globe, logics);
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}

/// Run Condition
///
/// Check if "change viewed layer" UI event needs handling.
pub fn check_event_changed(events: Res<EventStruct>) -> bool {
    events.viewed_layer_changed.is_some()
}

/// Update system
///
/// Assign respective layer material to the world model.
pub fn update_event_changed(
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

/// Run condition
///
/// Check if "import world" event needs handling.
pub fn check_event_import(events: Res<EventStruct>) -> bool {
    events.import_world_request.is_some()
}

/// Run condition
///
/// Check if "regen layer image" event needs handling.
pub fn check_event_regen(events: Res<EventStruct>) -> bool {
    events.regen_layer_request.is_some()
}

/// Update system
///
/// Regenerate graphical layer based on logical layer data.
pub fn update_event_regen(
    mut events: ResMut<EventStruct>,
    config: ResMut<AtlasSimConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    logics: Res<MapLogicData>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layers = events.regen_layer_request.take().expect("Always Some");
    for layer in layers {
        // Convert logical data to image data.
        let mut data = data_to_view(&logics, layer, &config);
        // Fetch handles.
        let layer = graphics.get_layer_mut(layer);
        let material = get_material_mut(&mut materials, &layer.material);
        // Assign new texture.
        let (width, height) = (config.general.world_size[0], config.general.world_size[1]);
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
/// Import the initial world for simulation.
pub fn update_event_import(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasSimConfig>,
    map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    commands: Commands,
) {
    let base_path = events.import_world_request.take().expect("Always Some");
    // Import config.
    let path = base_path.join(CONFIG_NAME);
    match load_config(path) {
        Ok(data) => {
            *config = data;
            events.world_model_changed = Some(());
        }
        Err(error) => {
            events.error_window = Some(error.to_string());
            return;
        }
    }
    // Import data layers.
    let (width, height) = (config.general.world_size[0], config.general.world_size[1]);
    let mut regen_layers = vec![];
    for (layer, name) in EXPORT_DATA_LAYERS {
        let path = base_path.join(name);
        match load_image_grey(path, width, height) {
            Ok(data) => {
                logics.put_layer(layer, data);
                regen_layers.push(layer);
            }
            Err(error) => {
                events.error_window = Some(error.to_string());
                return;
            }
        };
    }
    // Import preview.
    let path = base_path.join(PREVIEW_NAME);
    match load_image(path, width, height) {
        Ok(data) => logics.put_layer(MapDataLayer::Preview, data),
        Err(error) => {
            events.error_window = Some(error.to_string());
            return;
        }
    };
    regen_layers.push(MapDataLayer::Preview);
    // Resize if needed.
    resize_helper(commands, &config, map, globe, logics);
    // Refresh layers.
    events.regen_layer_request = Some(regen_layers);
}

/// Helper function
///
/// Switch and resize world models.
fn resize_helper(
    mut commands: Commands,
    config: &AtlasSimConfig,
    mut map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    mut globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    mut logics: ResMut<MapLogicData>,
) {
    // Run queries.
    let (map_en, mut map_vis, mut map_tran) = map.single_mut();
    let (globe_en, mut globe_vis) = globe.single_mut();
    let (width, height) = (config.general.world_size[0], config.general.world_size[1]);
    logics.resize_all_layers((width * height) as usize);
    match config.general.preview_model {
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
