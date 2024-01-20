use bevy::prelude::*;

mod config;
mod map;
mod ui;

/// Application entry point.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(config::ConfigPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(map::MapPlugin)
        .run();
}
