use std::{collections::HashMap, error, fs};

use bevy::prelude::*;

use crate::files::genfile::{FillTypes, Genfile, NoiseTypes, OperateMode, OperateOnType, TileType};

type GenericResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct FilePlugin;
impl Plugin for FilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(foo);
    }
}

// TODO rename
fn foo(mut commands: Commands) {
    let filename = "default.genfile.toml";

    let mut genfile = match parse_file(filename) {
        Ok(g) => g,
        Err(e) => {
            error!({ info = e }, "Error on parsing file");
            return;
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

    index_types(&mut genfile, &tempmap);

    commands.insert_resource(genfile);
}

fn parse_file(filename: &str) -> GenericResult<Genfile> {
    let contents = fs::read_to_string(filename)?;
    let genfile = toml::from_str(&contents)?;
    Ok(genfile)
}

//TODO move to Genfile enums as member method
#[allow(clippy::single_match)]
fn index_types(genfile: &mut Genfile, tempmap: &HashMap<String, usize>) {
    for transform_type in &mut genfile.transforms {
        match &mut transform_type.mode {
            OperateMode::Fill(ft) => match ft {
                FillTypes::Simple(sp) => match &sp.value {
                    OperateOnType::Tile(t) => {
                        sp.value = OperateOnType::Index(*tempmap.get(t).unwrap())
                    }
                    _ => {}
                },
                FillTypes::Conditional(cp) => match &cp.value {
                    OperateOnType::Tile(t) => {
                        cp.value = OperateOnType::Index(*tempmap.get(t).unwrap())
                    }
                    _ => {}
                },
                FillTypes::Flood(fp) => match &fp.value {
                    OperateOnType::Tile(t) => {
                        fp.value = OperateOnType::Index(*tempmap.get(t).unwrap())
                    }
                    _ => {}
                },
            },
            OperateMode::Noise(nt) => match nt {
                NoiseTypes::Pepper(pp) => {
                    match &pp.value {
                        OperateOnType::Tile(t) => {
                            pp.value = OperateOnType::Index(*tempmap.get(t).unwrap())
                        }
                        _ => {}
                    }
                    if let Some(on) = &mut pp.on {
                        match on {
                            OperateOnType::Tile(t) => {
                                *on = OperateOnType::Index(*tempmap.get(t).unwrap())
                            }
                            _ => {}
                        }
                    }
                }
                NoiseTypes::Perlin(pp) => match &pp.value {
                    OperateOnType::Tile(t) => {
                        pp.value = OperateOnType::Index(*tempmap.get(t).unwrap())
                    }
                    _ => {}
                },
            },
            _ => {}
        };
    }
}
