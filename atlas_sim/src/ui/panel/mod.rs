#[allow(clippy::module_inception)] // Reason: module is private anyways
mod panel;

mod general;

pub use panel::*;
pub use general::*;
