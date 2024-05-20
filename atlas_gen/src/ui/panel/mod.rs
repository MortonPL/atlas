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

pub use climate::*;
pub use continents::*;
pub use precipitation::*;
pub use resources::*;
pub use temperature::*;
pub use topography::*;
