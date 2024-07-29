use crate::{bevy::prelude::*, domain::map::MapDataLayer};
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
    /// The currently viewed map overlays have been changed.
    pub viewed_overlay_changed: Option<([bool; 3], bool)>,
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
    /// The simulation should begin.
    pub simulation_start_request: Option<()>,
    /// An error has occured, and a popup window should display it.
    pub error_window: Option<String>,
}

impl Default for EventStruct {
    fn default() -> Self {
        Self {
            world_model_changed: None,
            viewed_layer_changed: None,
            viewed_overlay_changed: None,
            load_layer_request: None,
            save_layer_request: None,
            render_layer_request: None,
            clear_layer_request: None,
            regen_layer_request: None,
            generate_request: None,
            load_climatemap_request: Some(()), // Load climate map on app start.
            import_start_request: None,
            import_world_request: None,
            export_world_request: None,
            randomize_starts_request: None,
            simulation_start_request: None,
            error_window: None,
        }
    }
}
