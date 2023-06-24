use std::f32::consts::PI;
use rand::Rng;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::structs::{Genfile, OperateMode, OperateOnType, NoiseTypes};

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_meshes);
    }
}

struct Tile {
    x: f32,
    y: f32,
    z: f32,
    color: [f32; 3],
}

pub fn create_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    genfile: Res<Genfile>,
) {
    const MAPSIZE: usize = 100;
    let mut tilemap: Vec<Tile> = Vec::with_capacity(MAPSIZE);
    for j in 0..MAPSIZE {
        tilemap.push(Tile{x: (j % 10) as f32 * 12.0, y: (j / 10) as f32 * 12.0, z: 0.0, color: [0.0, 0.0, 0.0]});
    }
    let mut rng = rand::thread_rng();

    for transform in &genfile.transforms {
        match &transform.mode {
            OperateMode::Fill(f) => match f {
                OperateOnType::Index(i) => {
                    let color = genfile.tiles[*i].color;
                    for tile in &mut tilemap {
                        tile.color = color;
                    }
                },
                _ => {},
            },
            OperateMode::Noise(n) => match n {
                NoiseTypes::Pepper(p) => match p.value {
                    OperateOnType::Index(i) => {
                        let color = genfile.tiles[i].color;
                        for tile in &mut tilemap {
                            if rng.gen::<f32>() <= p.frequency {
                                tile.color = color;
                            }
                        }
                    },
                    _ => {}
                }
            }
            _ => {},
        }
    }

    for tile in tilemap {
        commands.spawn(MaterialMesh2dBundle{
            mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(10.0, 4))).into(),
            transform: Transform::default().with_translation(Vec3 { x: tile.x, y: tile.y, z: tile.z }).with_rotation(Quat::from_rotation_z(PI/4.0)),
            material: materials.add(ColorMaterial::from(Color::Rgba{red: tile.color[0], green: tile.color[1], blue: tile.color[2], alpha: 1.0})),
            ..default()
        });
    }

    commands.spawn(Camera2dBundle::default());
}
