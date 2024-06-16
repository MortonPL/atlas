use std::path::Path;

use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    domain::map::MapDataLayer,
};

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
    /// Some map layer textures should be regenerated.
    pub regen_layer_request: Option<Vec<MapDataLayer>>,
    /// The world map should be imported from files.
    pub import_world_request: Option<Box<Path>>,
    /// The whole world state should be imported from files.
    pub import_state_request: Option<Box<Path>>,
    /// The whole world state should be exported to files.
    pub export_state_request: Option<Box<Path>>,
    /// An error has occured, and a popup window should display it.
    pub error_window: Option<String>,
}

impl Default for EventStruct {
    fn default() -> Self {
        Self {
            world_model_changed: Default::default(),
            viewed_layer_changed: Default::default(),
            regen_layer_request: Default::default(),
            import_world_request: Default::default(),
            import_state_request: Default::default(),
            export_state_request: Default::default(),
            error_window: Default::default(),
        }
    }
}
