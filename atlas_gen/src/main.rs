use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod config;
mod ui;

use config::GeneratorConfig;

/// Application entry point.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<GeneratorConfig>()
        .add_plugins(ui::UiPlugin)
        .run();
}
