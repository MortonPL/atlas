use crate::{
    bevy::prelude::*,
    domain::map::{MapDataLayer, MapDataOverlay},
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
    /// The currently viewed map overlay has been changed.
    pub viewed_overlay_changed: Option<MapDataOverlay>,
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
            world_model_changed: Default::default(),
            viewed_layer_changed: Default::default(),
            viewed_overlay_changed: Default::default(),
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
            simulation_start_request: Default::default(),
            error_window: Default::default(),
        }
    }
}
