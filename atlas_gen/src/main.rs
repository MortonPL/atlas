#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use atlas_lib::{
    base::events::EventPlugin,
    bevy::{core::TaskPoolThreadAssignmentPolicy, prelude::*},
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
                .set(ImagePlugin::default_nearest())
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        compute: TaskPoolThreadAssignmentPolicy {
                            min_threads: 1,
                            max_threads: 1,
                            percent: 1.0,
                        },
                        ..Default::default()
                    },
                }),
            EntropyPlugin::<WyRand>::default(),
            ConfigPlugin::<AtlasGenConfig>::default(),
            EventPlugin,
            ui::UiPlugin,
            map::MapPlugin,
        ))
        .add_systems(Startup, atlas_lib::set_window_icon)
        .run();
}
