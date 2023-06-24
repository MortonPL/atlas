use bevy::prelude::{App, Plugin};

pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, _app: &mut App) {
        println!("Hellow world!");
    }
}
