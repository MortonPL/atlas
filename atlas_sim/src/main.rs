#![windows_subsystem = "windows"]

use atlas_lib::bevy::prelude::*;

mod config;
mod event;
mod ui;

/// Application entry point.
fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Atlas History Simulator {}", env!("CARGO_PKG_VERSION")),
            ..Default::default()
        }),
        ..Default::default()
    }))
        .add_plugins(config::ConfigPlugin)
        .add_plugins(event::EventPlugin)
        .add_plugins(ui::Uiplugin)
        .add_systems(Startup, atlas_lib::set_window_icon)
        .run();
}
