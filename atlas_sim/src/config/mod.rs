mod climate;
mod main_structs;

pub use climate::*;
pub use main_structs::*;

use atlas_lib::bevy::prelude::*;

pub const CONFIG_NAME: &str = "atlassim.toml";

/// Plugin responsible for the simulator configuration and I/O.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtlasSimConfig>();
    }
}
