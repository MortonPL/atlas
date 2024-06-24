use crate::{
    bevy::prelude::*,
    config::{AtlasConfig, WorldModel},
    domain::{
        graphics::{CurrentWorldModel, MapGraphicsData, MapLogicData, WorldGlobeMesh, WorldMapMesh},
        map::MapDataLayer,
    },
};
use std::path::Path;

/// Plugin responsible for holding event requests and their data.
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventStruct>();
    }
}

/// Global resource that hold event requests. All requests are `Option`s:
/// `Some` means an event has been requested, `None` means nothing happened.
/// These fields are checked by run condition systems in other plugins.
///
/// This way, we can request something to happen next frame, from any place in the code
/// (as long as this struct can be accessed/is passed down).
#[derive(Resource)]
pub struct EventStruct {
    /// The world model in the config has been changed.
    pub world_model_changed: Option<()>,
    /// The currently viewed map layer has been changed.
    pub viewed_layer_changed: Option<MapDataLayer>,
    /// A map layer should be loaded from data.
    pub load_layer_request: Option<(MapDataLayer, Vec<u8>)>,
    /// A map layer should be saved to file.
    pub save_layer_request: Option<(MapDataLayer, Box<Path>)>,
    /// A map layer should be rendered to file.
    pub render_layer_request: Option<(MapDataLayer, Box<Path>)>,
    /// A map layer should be cleared.
    pub clear_layer_request: Option<MapDataLayer>,
    /// Some map layer textures should be regenerated.
    pub regen_layer_request: Option<Vec<MapDataLayer>>,
    /// This map layer requests data generation (should the influence layer be regenerated too?).
    pub generate_request: Option<(MapDataLayer, bool)>,
    /// "climatemap.png" should be reloaded.
    pub load_climatemap_request: Option<()>,
    /// The initial world map should be imported from files.
    pub import_start_request: Option<Box<Path>>,
    /// The whole world should be imported from files.
    pub import_world_request: Option<Box<Path>>,
    /// The whole world should be exported to files.
    pub export_world_request: Option<Box<Path>>,
    /// The scenario starting points should be randomized.
    pub randomize_starts_request: Option<()>,
    /// An error has occured, and a popup window should display it.
    pub error_window: Option<String>,
}

impl Default for EventStruct {
    fn default() -> Self {
        Self {
            world_model_changed: Default::default(),
            viewed_layer_changed: Default::default(),
            load_layer_request: Default::default(),
            save_layer_request: Default::default(),
            render_layer_request: Default::default(),
            clear_layer_request: Default::default(),
            regen_layer_request: Default::default(),
            generate_request: Default::default(),
            // Load climate map on start.
            load_climatemap_request: Some(()),
            import_start_request: Default::default(),
            import_world_request: Default::default(),
            export_world_request: Default::default(),
            randomize_starts_request: Default::default(),
            error_window: Default::default(),
        }
    }
}

/// Run Condition
///
/// Check if "change world model" UI event needs handling.
pub fn check_event_world_model(events: Res<EventStruct>) -> bool {
    events.world_model_changed.is_some()
}

/// Run Condition
///
/// Check if "change viewed layer" UI event needs handling.
pub fn check_event_changed(events: Res<EventStruct>) -> bool {
    events.viewed_layer_changed.is_some()
}

/// Run condition
///
/// Check if "regen layer image" event needs handling.
pub fn check_event_regen(events: Res<EventStruct>) -> bool {
    events.regen_layer_request.is_some()
}

/// Run condition
///
/// Check if "import world" event needs handling.
pub fn check_event_import(events: Res<EventStruct>) -> bool {
    events.import_world_request.is_some()
}

/// Run condition
///
/// Check if "export world" event needs handling.
pub fn check_event_export(events: Res<EventStruct>) -> bool {
    events.export_world_request.is_some()
}

/// Update system
///
/// Handle "change world model" UI event.
pub fn update_event_world_model<C: AtlasConfig>(
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
