use std::{fs, collections::HashMap};
use std::process::exit;
use toml;

use bevy::prelude::*;

use crate::structs::{Genfile, OperateMode, NoiseTypes, OperateOnType, TileType};
pub struct FilePlugin;

impl Plugin for FilePlugin {
    fn build(&self, app: &mut App) {
        let filename = "default.genfile.toml";
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file {}!", filename);
                exit(1);
            }
        };

        let mut genfile: Genfile = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Could not parse file {}! Message: {}", filename, e.message());
                exit(1);
            }
        };

        let mut tile_types: Vec<TileType> = Default::default();
        let mut tempmap: HashMap<String, usize> = Default::default();
        for tile_type in genfile.tiles {
            if tempmap.contains_key(&tile_type.name) {
                continue;
            }
            tempmap.insert(tile_type.name.clone(), tile_types.len());
            tile_types.push(tile_type);
        }
        genfile.tiles = tile_types;

        for transform_type in &mut genfile.transforms {
            match &mut transform_type.mode {
                OperateMode::Fill(v) => match v {
                    OperateOnType::Tile(t) => {*v = OperateOnType::Index(*tempmap.get(t).unwrap())},
                    _ => {},
                }
                OperateMode::Noise(nt) => match nt {
                    NoiseTypes::Pepper(pp) => match &pp.value {
                        OperateOnType::Tile(t) => {pp.value = OperateOnType::Index(*tempmap.get(t).unwrap())},
                        _ => {},
                    },
                },
                _ => {},
            };
        }

        app.insert_resource(genfile);
    }
}
