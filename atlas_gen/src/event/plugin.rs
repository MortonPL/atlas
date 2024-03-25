use std::path::Path;

use bevy::prelude::*;

use crate::{config::WorldModel, map::ViewedMapLayer};

/// Plugin responsible for holding event data.
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventStruct>();
    }
}

#[derive(Default, Resource)]
pub struct EventStruct {
    pub world_model_changed: Option<WorldModel>,
    pub viewed_layer_changed: Option<ViewedMapLayer>,
    pub load_layer_request: Option<(ViewedMapLayer, Vec<u8>)>,
    pub save_layer_request: Option<(ViewedMapLayer, Box<Path>)>,
    pub reset_layer_request: Option<ViewedMapLayer>,
    pub regen_layer_request: Option<ViewedMapLayer>,
}
