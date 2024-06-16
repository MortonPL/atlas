use std::path::Path;

use atlas_lib::bevy::{ecs as bevy_ecs, prelude::*};

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
    /// The whole world should be imported from files.
    pub import_world_request: Option<Box<Path>>,
    /// The whole world should be exported to files.
    pub export_world_request: Option<Box<Path>>,
    /// An error has occured, and a popup window should display it.
    pub error_window: Option<String>,
}

impl Default for EventStruct {
    fn default() -> Self {
        Self {
            import_world_request: Default::default(),
            export_world_request: Default::default(),
            error_window: Default::default(),
        }
    }
}
