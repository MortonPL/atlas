#[allow(clippy::module_inception)] // Reason: module is private anyways
mod panel;

mod general;
mod climate;
mod continents;
mod resources;
mod topography;
mod temperature;
mod humidity;

pub use general::*;
pub use panel::*;

use climate::*;
use continents::*;
use resources::*;
use topography::*;
use temperature::*;
use humidity::*;
