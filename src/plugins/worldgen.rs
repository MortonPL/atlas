use std::{f32::consts::PI, collections::VecDeque};
use rand::{Rng, rngs::ThreadRng};

use bevy::{prelude::*};
use bevy::sprite::MaterialMesh2dBundle;

use crate::structs::{Genfile, OperateMode, OperateOnType, NoiseTypes, FillTypes};

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_tilemap(6, 6));
        app.add_startup_system(transforms).add_startup_system(create_meshes);
    }
}

struct Tile {
    x: f32,
    y: f32,
    z: f32,
    color: [f32; 3],
    set: bool,
}

#[derive(Resource)]
struct Tilemap {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

fn create_tilemap(width: usize, height: usize) -> Tilemap {
    let w: i32 = width.try_into().unwrap();
    let h: i32 = height.try_into().unwrap();
    let mut tiles: Vec<Tile> = Vec::with_capacity((w * h).try_into().unwrap());
    for j in 0..(w * h) {
        tiles.push(Tile{x: (j % w - w / 2) as f32 * 15.0, y: (j / w - h / 2) as f32 * 15.0, z: 0.0, color: [0.0, 0.0, 0.0], set: false});
    }
    Tilemap{tiles, width, height}
}

#[derive(Clone)]
struct Span {
    pub x1: i32,
    pub x2: i32,
    pub y: i32,
    pub dy: i32,
}

fn span_fill(tilemap: &mut Tilemap, color: &[f32; 3], num_origins: usize, max: f32, rng: &mut ThreadRng) {
    let mut queue: VecDeque<Span> = default();
    for _ in 0..num_origins {
        let mut originx= rng.gen_range(0..tilemap.width);
        let mut originy = rng.gen_range(0..tilemap.height);

        let set = |x: i32, y: i32, tilemap: &mut Tilemap| {
            if y < tilemap.height as i32 && y >= 0 && x < tilemap.width as i32 && x >= 0 {
                let x = x as usize;
                let y = y as usize;
                tilemap.tiles[y * tilemap.width + x] = Tile{color: *color, set: true, ..tilemap.tiles[y * tilemap.width + x]};
            }
        };
        let inside = |x: i32, y: i32, tilemap: &mut Tilemap| {
            if y < tilemap.height as i32 && y >= 0 && x < tilemap.width as i32 && x >= 0 {
                let x = x as usize;
                let y = y as usize;
                !tilemap.tiles[y * tilemap.width + x].set && tilemap.tiles[y * tilemap.width + x].z <= max
            } else {
                false
            }
        };

        let mut attempts = 0;
        while !inside(originx as i32, originy as i32, tilemap) {
            attempts += 1;
            if attempts <= 3 {
                originx = rng.gen_range(0..tilemap.width);
                originy = rng.gen_range(0..tilemap.height);
            } else {
                break;
            }
        }
        if attempts > 3 {
            continue;
        }

        // add origin
        queue.push_back(Span{ x1: originx as i32, x2: originx as i32, y: originy as i32, dy: 1});
        // add below origin if possible
        if originy > 0 {
            queue.push_back(Span{ x1: originx as i32, x2: originx as i32, y: (originy - 1) as i32, dy: -1});
        }

        println!("{} {}", originx, originy);
        while !queue.is_empty() {
            let mut span = queue.pop_back().unwrap();
            let mut x = span.x1;
            println!("start: {} {} {} {} {}", span.x1, span.x2, span.y, span.dy, x);
            // go through left of span
            if inside(x, span.y, tilemap) {
                while inside(x - 1, span.y, tilemap) {
                    set(x - 1, span.y, tilemap);
                    x -= 1;
                }
            }
            println!("#1: \t\t\t{} {} {} {} {}", span.x1, span.x2, span.y, span.dy, x);
            // queue new span in reverse horizontal direction if we moved at all
            if x < span.x1 {
                queue.push_back(Span{x1: x, x2: span.x1 - 1, y: span.y - span.dy, dy: -span.dy});
            }
            println!("#2: \t\t\t{} {} {} {} {}", span.x1, span.x2, span.y, span.dy, x);
            // scan in span
            while span.x1 <= span.x2 {
                // scan right of start of span
                while inside(span.x1, span.y, tilemap) {
                    set(span.x1, span.y, tilemap);
                    span.x1 += 1;
                }

                // go up in our direction, mark entire explored line as span
                queue.push_back(Span{x1: x, x2: span.x1 - 1, y: span.y + span.dy, dy: span.dy});
                // if we went past the end of this span, add in reverse
                if span.x1 - 1 > span.x2 {
                    queue.push_back(Span{x1: span.x2 + 1, x2: span.x1 - 1, y: span.y - span.dy, dy: -span.dy});
                }
                span.x1 += 1;
                // ???
                while span.x1 < span.x2 && !inside(span.x1, span.y, tilemap) {
                    span.x1 += 1;
                }
                x = span.x1;
            }
        }
    }
}


fn transforms(genfile: Res<Genfile>, mut tilemap: ResMut<Tilemap>) {
    let mut rng = rand::thread_rng();

    for transform in &genfile.transforms {
        match &transform.mode {
            OperateMode::Fill(f) => match f {
                FillTypes::Simple(s) => match s.value {
                    OperateOnType::Index(i) => {
                        let color = genfile.tiles[i].color;
                        for tile in &mut tilemap.tiles {
                            *tile = Tile{color, ..*tile};
                        }
                    },
                    OperateOnType::Height(h) => {
                        for tile in &mut tilemap.tiles {
                            tile.z = h;
                        }
                    },
                    _ => {},
                },
                FillTypes::Flood(f) => match f.value {
                    OperateOnType::Index(i) => {
                        let color = genfile.tiles[i].color;
                        span_fill(&mut tilemap, &color, f.origins, f.max, &mut rng);
                    },
                    _ => {},
                },
            },
            OperateMode::Noise(n) => match n {
                NoiseTypes::Pepper(p) => match p.value {
                    OperateOnType::Index(i) => {
                        let color = genfile.tiles[i].color;
                        for tile in &mut tilemap.tiles {
                            if rng.gen::<f32>() <= p.frequency {
                                *tile = Tile{color, ..*tile};
                            }
                        }
                    },
                    OperateOnType::Height(h) => {
                        for tile in &mut tilemap.tiles {
                            if rng.gen::<f32>() <= p.frequency {
                                tile.z = h;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {},
        }
    }
}

fn create_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tilemap: Res<Tilemap>,
    asset_server: Res<AssetServer>,
) {
    for tile in &tilemap.tiles {
        commands.spawn(MaterialMesh2dBundle{
            mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(10.0, 4))).into(),
            transform: Transform::from_xyz(tile.x, tile.y, 0.0).with_rotation(Quat::from_rotation_z(PI/4.0)),
            material: materials.add(ColorMaterial::from(Color::Rgba{red: tile.color[0], green: tile.color[1], blue: tile.color[2], alpha: 1.0})),
            ..default()
        });
        commands.spawn(Text2dBundle{
            text: Text {
                sections: vec![TextSection{value: tile.z.to_string(), style: TextStyle { font: asset_server.load("fonts/default.ttf"), font_size: 15.0, color: Color::BLACK }}],
                alignment: TextAlignment::Center,
                linebreak_behaviour: bevy::text::BreakLineOn::WordBoundary,
            },
            transform: Transform::from_xyz(tile.x, tile.y, 1.),
            text_anchor: bevy::sprite::Anchor::Center,
            text_2d_bounds: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            global_transform: Default::default(),
        });
    }

    commands.spawn(Camera2dBundle::default());
}
