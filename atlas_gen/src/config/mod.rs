mod climate;
mod common;
mod conversions;
mod latitudinal;
mod main_structs;

pub use climate::*;
pub use conversions::*;
pub use main_structs::*;

use atlas_lib::bevy::prelude::*;

pub const CONFIG_NAME: &str = "atlasgen.toml";

/// Plugin responsible for the generator configuration and I/O.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtlasGenConfig>();
    }
}
