mod climate_structs;
mod common_structs;
mod conversions;
mod main_structs;

pub use conversions::*;
pub use main_structs::*;

use atlas_lib::bevy::prelude::*;

/// Plugin responsible for the generator configuration and I/O.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtlasGenConfig>();
    }
}
