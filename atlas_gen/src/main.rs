use atlas_lib::bevy::prelude::*;

mod config;
mod event;
mod map;
mod ui;

/// Application entry point.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(config::ConfigPlugin)
        .add_plugins(event::EventPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(map::MapPlugin)
        .run();
}
