use atlas_lib::bevy::prelude::*;

use crate::config::AtlasGenConfig;

/// Plugin responsible for the generator configuration and I/O.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtlasGenConfig>();
    }
}
