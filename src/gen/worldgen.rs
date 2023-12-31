use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, Rng, RngCore};
use std::{collections::VecDeque, f32::consts::PI, ops::Deref, ops::DerefMut};

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::files::genfile::{FillTypes, Genfile, NoiseTypes, OperateMode, OperateOnType};

struct WorldGenRng(Box<dyn RngCore + 'static>);

impl Deref for WorldGenRng {
    type Target = dyn RngCore + 'static;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WorldGenRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_tilemap(20, 20));
        app.insert_non_send_resource(WorldGenRng(Box::new(ThreadRng::default())));
        app.add_startup_system(transforms)
            .add_startup_system(create_meshes);
    }
}

#[derive(Debug)]
struct Tile {
    x: f32,
    y: f32,
    z: f32,
    idx: usize,
    set: bool,
}

#[derive(Debug, Resource)]
struct Tilemap {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

fn create_tilemap(width: usize, height: usize) -> Tilemap {
    let w: i32 = width.try_into().unwrap();
    let h: i32 = height.try_into().unwrap();
    const DIST: f32 = 18.0;
    let mut tiles: Vec<Tile> = Vec::with_capacity((w * h).try_into().unwrap());
    for j in 0..(w * h) {
        tiles.push(Tile {
            x: (j % w - w / 2) as f32 * DIST,
            y: (j / w - h / 2) as f32 * DIST,
            z: 0.0,
            idx: 0,
            set: false,
        });
    }
    Tilemap {
        tiles,
        width,
        height,
    }
}

#[derive(Debug, Clone, Copy)]
struct Span {
    pub x1: i32,
    pub x2: i32,
    pub y: i32,
    pub dy: i32,
}

fn span_fill(
    tilemap: &mut Tilemap,
    idx: usize,
    num_origins: usize,
    min: f32,
    max: f32,
    rng: &mut dyn RngCore,
) {
    let mut queue: VecDeque<Span> = default();
    for _ in 0..num_origins {
        let mut originx = rng.gen_range(0..tilemap.width);
        let mut originy = rng.gen_range(0..tilemap.height);

        let set = |x: i32, y: i32, tilemap: &mut Tilemap| {
            if y < tilemap.height as i32 && y >= 0 && x < tilemap.width as i32 && x >= 0 {
                let x = x as usize;
                let y = y as usize;
                tilemap.tiles[y * tilemap.width + x] = Tile {
                    idx,
                    set: true,
                    ..tilemap.tiles[y * tilemap.width + x]
                };
            }
        };
        let inside = |x: i32, y: i32, tilemap: &mut Tilemap| {
            if y < tilemap.height as i32 && y >= 0 && x < tilemap.width as i32 && x >= 0 {
                let x = x as usize;
                let y = y as usize;
                !tilemap.tiles[y * tilemap.width + x].set
                    && tilemap.tiles[y * tilemap.width + x].z <= max
                    && tilemap.tiles[y * tilemap.width + x].z >= min
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
        queue.push_back(Span {
            x1: originx as i32,
            x2: originx as i32,
            y: originy as i32,
            dy: 1,
        });
        // add below origin if possible
        if originy > 0 {
            queue.push_back(Span {
                x1: originx as i32,
                x2: originx as i32,
                y: (originy - 1) as i32,
                dy: -1,
            });
        }

        while !queue.is_empty() {
            let mut span = queue.pop_back().unwrap();
            let mut x = span.x1;
            // go through left of span
            if inside(x, span.y, tilemap) {
                while inside(x - 1, span.y, tilemap) {
                    set(x - 1, span.y, tilemap);
                    x -= 1;
                }
            }
            // queue new span in reverse horizontal direction if we moved at all
            if x < span.x1 {
                queue.push_back(Span {
                    x1: x,
                    x2: span.x1 - 1,
                    y: span.y - span.dy,
                    dy: -span.dy,
                });
            }
            // scan in span
            while span.x1 <= span.x2 {
                // scan right of start of span
                while inside(span.x1, span.y, tilemap) {
                    set(span.x1, span.y, tilemap);
                    span.x1 += 1;
                }

                // go up in our direction, mark entire explored line as span
                queue.push_back(Span {
                    x1: x,
                    x2: span.x1 - 1,
                    y: span.y + span.dy,
                    dy: span.dy,
                });
                // if we went past the end of this span, add in reverse
                if span.x1 - 1 > span.x2 {
                    queue.push_back(Span {
                        x1: span.x2 + 1,
                        x2: span.x1 - 1,
                        y: span.y - span.dy,
                        dy: -span.dy,
                    });
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

fn transforms(genfile: Res<Genfile>, mut tilemap: ResMut<Tilemap>, mut rng: NonSendMut<WorldGenRng>) {
    for transform in &genfile.transforms {
        match &transform.mode {
            OperateMode::Fill(f) => match f {
                FillTypes::Simple(s) => match s.value {
                    OperateOnType::Index(i) => {
                        for tile in &mut tilemap.tiles {
                            *tile = Tile { idx: i, ..*tile };
                        }
                    }
                    OperateOnType::Height(h) => {
                        for tile in &mut tilemap.tiles {
                            tile.z = h;
                        }
                    }
                    _ => {}
                },
                FillTypes::Flood(f) => {
                    if let OperateOnType::Index(i) = f.value {
                        span_fill(&mut tilemap, i, f.origins, f.min, f.max, &mut rng.0);
                    }
                }
                FillTypes::Conditional(c) => match c.value {
                    #[allow(clippy::single_match)]
                    OperateOnType::Index(i) => {
                        for tile in &mut tilemap.tiles {
                            if tile.z >= c.min && tile.z <= c.max {
                                *tile = Tile { idx: i, ..*tile };
                            }
                        }
                    }
                    _ => {}
                },
            },
            OperateMode::Noise(n) => match n {
                NoiseTypes::Pepper(p) => match p.value {
                    OperateOnType::Index(i) => {
                        let mut idx = -1;
                        if let Some(on) = &p.on {
                            idx = match on {
                                OperateOnType::Index(i) => *i as i32,
                                _ => -1,
                            };
                        }
                        for tile in &mut tilemap.tiles {
                            if rng.gen::<f32>() <= p.frequency
                                && (idx == -1 || tile.idx == idx as usize)
                            {
                                *tile = Tile { idx: i, ..*tile };
                            }
                        }
                    }
                    OperateOnType::Height(h) => {
                        for tile in &mut tilemap.tiles {
                            if rng.gen::<f32>() <= p.frequency {
                                tile.z += h + rng.gen_range(-p.deviation..=p.deviation);
                            }
                        }
                    }
                    _ => {}
                },
                NoiseTypes::Perlin(p) => {
                    if let OperateOnType::Height(h) = p.value {
                        let perlin = Perlin::new(rng.gen());
                        let w = tilemap.width;
                        let hh = tilemap.height;
                        for (i, tile) in tilemap.tiles.iter_mut().enumerate() {
                            let x = (i % w) as f64 / w as f64;
                            let y = (i / w) as f64 / hh as f64;
                            let v = perlin.get([x * p.scale as f64, y * p.scale as f64]);
                            tile.z += v as f32 * h + p.offset;
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

fn create_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tilemap: Res<Tilemap>,
    genfile: Res<Genfile>,
    asset_server: Res<AssetServer>,
) {
    for tile in &tilemap.tiles {
        let color = genfile.tiles[tile.idx].color;
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::RegularPolygon::new(12.0, 4)))
                .into(),
            transform: Transform::from_xyz(tile.x, tile.y, 0.0)
                .with_rotation(Quat::from_rotation_z(PI / 4.0)),
            material: materials.add(ColorMaterial::from(Color::Rgba {
                red: color[0],
                green: color[1],
                blue: color[2],
                alpha: 1.0,
            })),
            ..default()
        });
        commands.spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("{:.1}", tile.z),
                    style: TextStyle {
                        font: asset_server.load("fonts/default.ttf"),
                        font_size: 15.0,
                        color: Color::BLACK,
                    },
                }],
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
