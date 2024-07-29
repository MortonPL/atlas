//#![windows_subsystem = "windows"] // DEBUG

use atlas_lib::{
    base::events::EventPlugin,
    bevy::prelude::*,
    bevy_prng::WyRand,
    bevy_rand::plugin::EntropyPlugin,
    config::{gen::AtlasGenConfig, ConfigPlugin},
};
mod map;
mod ui;

/// Application entry point.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Atlas Map Generator {}", env!("CARGO_PKG_VERSION")),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            EntropyPlugin::<WyRand>::default(),
            ConfigPlugin::<AtlasGenConfig>::default(),
            EventPlugin,
            ui::UiPlugin,
            map::MapPlugin,
        ))
        .add_systems(Startup, atlas_lib::set_window_icon)
        .run();
}
