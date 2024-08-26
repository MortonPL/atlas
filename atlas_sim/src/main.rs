#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod map;
mod sim;
mod ui;

use atlas_lib::{
    base::events::EventPlugin,
    bevy::{core::TaskPoolThreadAssignmentPolicy, prelude::*},
    bevy_prng::WyRand,
    bevy_rand::prelude::EntropyPlugin,
    config::{sim::AtlasSimConfig, ConfigPlugin},
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
        )
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(ConfigPlugin::<AtlasSimConfig>::default())
        .add_plugins(EventPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(sim::SimPlugin)
        .add_systems(Startup, atlas_lib::set_window_icon)
        .run();
}
