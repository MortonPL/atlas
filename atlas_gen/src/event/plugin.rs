use std::path::Path;

use bevy::prelude::*;

use crate::{config::WorldModel, map::MapDataLayer};

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
    pub world_model_changed: Option<WorldModel>,
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
    /// The whole world should be exported to files.
    pub export_world_request: Option<Box<Path>>,
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
            export_world_request: Default::default(),
        }
    }
}
