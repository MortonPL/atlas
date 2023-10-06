use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::MaterialMesh2dBundle;
use rand::{rngs::ThreadRng, Rng};
use std::f32::consts::PI;
use voronoi::{make_polygons, voronoi, Point};

pub struct TectonicsPlugin;

impl Plugin for TectonicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TectonicPlates::default());
        app.add_startup_system(make_voronoi);
        app.add_startup_system(create_meshes);
    }
}

#[derive(Debug, Default, Resource)]
pub struct TectonicPlates {
    plates: Vec<TectonicPlate>,
}

#[derive(Debug, Default, Resource)]
enum TectonicPlateTypeEnum {
    #[default]
    Oceanic,
    Continental,
}

#[derive(Debug, Default, Resource)]
pub struct TectonicPlate {
    pos: Vec2,
    faces: Vec<Point>,
    move_dir: f32,
    move_speed: f32,
    relief_height: i32,
    age: i32,
    plate_type: TectonicPlateTypeEnum,
}

fn make_voronoi(mut plates: ResMut<TectonicPlates>) {
    // sample N Points2D in Width, Hight range
    let num_of_plates: usize = 20;
    const HEIGHT: f64 = 300.;
    const WIDTH: f64 = 300.;
    let mut array = vec![0.; num_of_plates * 2];
    let mut rng = ThreadRng::default();
    rng.fill(&mut array[..]);
    let points: Vec<Point> = (0..num_of_plates * 2)
        .step_by(2)
        .map(|i| Point::new(array[i] * WIDTH, array[i + 1] * HEIGHT))
        .collect();
    // create a Voronoi diagram
    let cells = voronoi(points, WIDTH);
    // obtain edges
    let cells = make_polygons(&cells);
    // create tectonic plates
    for cell in cells {
        let len = cell.len() as f32;
        let x: f32 = cell.iter().map(|p| p.x() as f32).sum::<f32>() / len;
        let y: f32 = cell.iter().map(|p| p.y() as f32).sum::<f32>() / len;
        let plate = TectonicPlate {
            pos: Vec2::new(x, y),
            faces: cell.to_vec(),
            move_dir: rng.gen_range(-PI..PI),
            move_speed: rng.gen(),
            relief_height: rng.gen(),
            age: rng.gen(),
            plate_type: if rng.gen_bool(0.7) {
                TectonicPlateTypeEnum::Oceanic
            } else {
                TectonicPlateTypeEnum::Continental
            },
        };
        plates.plates.push(plate);
    }
    // upsample and randomly warp edge points
    // draw
}

/// Create an arrow-shaped Bevy Mesh with given length.
fn make_arrow_mesh(len: f32) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(7);
    let indices = vec![0, 2, 1, 6, 4, 3, 5, 4, 6];
    const BODY_WIDTH: f32 = 0.33;
    let length = len * 5.0;

    // head
    vertices.extend(
        [
            [-1.0, length, 0.0],
            [0.0, length + 1.0, 0.0],
            [1.0, length, 0.0],
        ]
        .iter(),
    );
    // body
    vertices.extend([
        [BODY_WIDTH, length, 0.0],
        [BODY_WIDTH, 0.0, 0.0],
        [-BODY_WIDTH, 0.0, 0.0],
        [-BODY_WIDTH, length, 0.0],
    ]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

/// Create a Bevy Mesh from a face (vector of 2D vertices).
fn mesh_from_face(face: &Vec<Point>) -> Mesh {
    let size = face.len();
    let vertices: Vec<[f32; 3]> = face
        .iter()
        .map(|p| [p.x() as f32, p.y() as f32, 0.])
        .collect();
    let indices = (1..(size - 1) as u32)
        .into_iter()
        .map(|i| [0, i + 1, i])
        .flatten()
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

fn create_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    plates: Res<TectonicPlates>,
) {
    let mut rng = ThreadRng::default();
    for plate in &plates.plates {
        let color = if matches!(plate.plate_type, TectonicPlateTypeEnum::Oceanic) {
            vec![0., 0., rng.gen_range(0.3..0.7)]
        } else {
            vec![0., rng.gen_range(0.3..0.7), 0.]
        };
        let mesh = mesh_from_face(&plate.faces);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(ColorMaterial::from(Color::Rgba {
                red: color[0],
                green: color[1],
                blue: color[2],
                alpha: 1.0,
            })),
            ..default()
        });
        let mesh = make_arrow_mesh(plate.move_speed);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform::from_translation(plate.pos.extend(1.0))
                .with_scale(Vec3 {
                    x: 5.0,
                    y: 5.0,
                    z: 5.0,
                })
                .with_rotation(Quat::from_rotation_z(plate.move_dir)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        });
    }

    commands.spawn(Camera2dBundle::default());
}
