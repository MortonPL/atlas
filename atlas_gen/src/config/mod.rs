mod climate;
mod common;
mod conversions;
mod latitudinal;
mod main_structs;
mod resource;

pub use common::*;
pub use conversions::*;
pub use latitudinal::*;
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
