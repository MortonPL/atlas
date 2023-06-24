use bevy::prelude::*;

mod components;
mod systems;
mod plugins;
mod structs;
use crate::components::Position;
use crate::systems::example_system;
use crate::plugins::{HelloWorldPlugin, FilePlugin, WorldGeneratorPlugin};

fn main() {
    let _p = Position{x: 0, y: 0};

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloWorldPlugin)
        .add_plugin(FilePlugin)
        .add_plugin(WorldGeneratorPlugin)
        .add_startup_system(example_system)
        .run();
}
