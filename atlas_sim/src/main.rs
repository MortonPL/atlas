use atlas_lib::bevy::prelude::*;

mod config;
mod event;
mod ui;

/// Application entry point.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(config::ConfigPlugin)
        .add_plugins(event::EventPlugin)
        .add_plugins(ui::Uiplugin)
        .run();
}
