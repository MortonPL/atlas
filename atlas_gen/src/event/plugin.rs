use std::path::Path;

use bevy::prelude::*;

use crate::{config::WorldModel, map::MapDataLayer};

/// Plugin responsible for holding event data.
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventStruct>();
    }
}

#[derive(Default, Resource)]
pub struct EventStruct {
    /// The world model in the config has been changed.
    pub world_model_changed: Option<WorldModel>,
    /// The currently viewed map layer has been changed.
    pub viewed_layer_changed: Option<MapDataLayer>,
    /// A map layer should be loaded from data.
    pub load_layer_request: Option<(MapDataLayer, Vec<u8>)>,
    /// A map layer should be saved to file.
    pub save_layer_request: Option<(MapDataLayer, Box<Path>)>,
    /// A map layer should be reset to empty.
    pub reset_layer_request: Option<MapDataLayer>,
    /// Some map layer textures should be regenerated.
    pub regen_layer_request: Option<Vec<MapDataLayer>>,
    /// This map layer requests data generation.
    pub generate_request: Option<MapDataLayer>,
}
