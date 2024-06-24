//#![windows_subsystem = "windows"] // DEBUG

mod config;
mod map;
mod ui;

use atlas_lib::{
    base::events::EventPlugin, bevy::prelude::*, bevy_prng::WyRand, bevy_rand::prelude::EntropyPlugin,
};

/// Application entry point.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Atlas History Simulator {}", env!("CARGO_PKG_VERSION")),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(config::ConfigPlugin)
        .add_plugins(EventPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(map::MapPlugin)
        .add_systems(Startup, atlas_lib::set_window_icon)
        .run();
}
