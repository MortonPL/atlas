use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod config;
mod ui;

use config::GeneratorConfig;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<GeneratorConfig>()
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui::ui_system)
        .run();
}
