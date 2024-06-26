//#![windows_subsystem = "windows"] DEBUG

use atlas_lib::bevy::prelude::*;
mod config;
mod event;
mod map;
mod ui;

/// Application entry point.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Atlas Map Generator {}", env!("CARGO_PKG_VERSION")),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(config::ConfigPlugin)
        .add_plugins(event::EventPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(map::MapPlugin)
        .add_systems(Startup, atlas_lib::set_window_icon)
        .run();
}
