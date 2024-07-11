mod climate;
mod main_structs;
mod resource;
mod rules;

pub use climate::*;
pub use main_structs::*;
pub use resource::*;
pub use rules::*;

use atlas_lib::bevy::prelude::*;

pub const CONFIG_NAME: &str = "atlassim.toml";

/// Plugin responsible for the simulator configuration and I/O.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtlasSimConfig>();
    }
}
