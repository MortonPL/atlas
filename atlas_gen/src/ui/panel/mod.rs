#[allow(clippy::module_inception)] // Reason: module is private anyways
mod panel;

mod climate;
mod continents;
mod general;
mod precipitation;
mod resources;
mod temperature;
mod topography;

pub use general::*;
pub use panel::*;

use climate::*;
use continents::*;
use precipitation::*;
use resources::*;
use temperature::*;
use topography::*;
