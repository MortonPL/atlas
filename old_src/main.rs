use bevy::prelude::*;

mod camera;
mod files;
mod gen;
mod sim;
mod world;
use crate::files::FilePlugin;
use crate::gen::WorldGeneratorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FilePlugin)
        .add_plugin(WorldGeneratorPlugin)
        .run();
}
