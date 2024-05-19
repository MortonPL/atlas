use bevy::prelude::*;

use crate::config::AtlasGenConfig;

/// Plugin responsible for the generator configuration and I/O.
pub struct GenConfigPlugin;

impl Plugin for GenConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtlasGenConfig>();
    }
}
