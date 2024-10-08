#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod base;
pub mod config;
pub mod domain;
pub mod ui;

pub use atlas_macro::*;

pub use bevy;
pub use bevy_egui;
pub use bevy_prng;
pub use bevy_rand;
pub use egui_file;
pub use png;
pub use rand;
pub use rand_distr;
pub use rstar;
pub use serde;
pub use serde_derive;
pub use serde_with;
pub use thiserror;
pub use toml;
pub use weighted_rand;
pub use winit;

/// Helpers
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use winit::platform::windows::IconExtWindows;
use winit::window::Icon;

/// Set the runtime window icon from resource.
pub fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    if let Ok(icon) = Icon::from_resource(32512, None) {
        for window in windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
    } else {
        error!("Failed to load icon resource!");
    }
}
