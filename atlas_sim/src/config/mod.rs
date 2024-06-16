mod main_structs;
mod climate;

pub use main_structs::*;
pub use climate::*;

use atlas_lib::bevy::prelude::*;

/// Plugin responsible for the simulator configuration and I/O.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtlasSimConfig>();
    }
}
