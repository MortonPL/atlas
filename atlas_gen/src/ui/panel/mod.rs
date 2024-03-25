pub mod advanced;
mod general;
#[allow(clippy::module_inception)] // Reason: module is private anyways
mod panel;
pub mod simple;

pub use general::*;
pub use panel::*;
