use bevy::prelude::*;

mod config;
mod map;
mod ui;

use config::GeneratorConfig;

/// Application entry point.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GeneratorConfig>()
        .add_plugins(ui::UiPlugin)
        .add_plugins(map::MapPlugin)
        .run();
}
