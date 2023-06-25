use bevy::prelude::*;

mod components;
mod systems;
mod plugins;
mod structs;
use crate::plugins::{FilePlugin, WorldGeneratorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FilePlugin)
        .add_plugin(WorldGeneratorPlugin)
        .run();
}
