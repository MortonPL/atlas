mod climate;
mod common;
mod conversions;
mod latitudinal;
mod main_structs;

pub use conversions::*;
pub use main_structs::*;
pub use climate::*;

use atlas_lib::bevy::prelude::*;

/// Plugin responsible for the generator configuration and I/O.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtlasGenConfig>();
    }
}
