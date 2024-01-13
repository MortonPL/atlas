use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::config::{GeneratorConfig, WorldModel};

/// Plugin responsible for the world map graphics.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}

// World map model tag.
#[derive(Component)]
pub struct WorldMapMesh;

// World globe model tag.
#[derive(Component)]
pub struct WorldGlobeMesh;

/// Startup system
///
/// Spawn the map and globe world models.
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sphere / globe
    let mesh = meshes.add(shape::UVSphere::default().into());
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });
    commands.spawn((PbrBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }, WorldGlobeMesh));
    // Plane / map
    let mesh = meshes.add(shape::Plane::default().into());
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });
    commands.spawn((PbrBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(EulerRot::XYZ, FRAC_PI_2, 0.0, 0.0)),
        ..Default::default()
    }, WorldMapMesh));
}

/// Update system
/// 
/// Display map or globe model depending on configuration.
fn update(
    config: Res<GeneratorConfig>,
    mut map: Query<&mut Visibility, With<WorldMapMesh>>,
    mut globe: Query<&mut Visibility, (With<WorldGlobeMesh>, Without<WorldMapMesh>)>
) {
    let mut map = map.single_mut();
    let mut globe = globe.single_mut();
    match config.general.world_model {
        WorldModel::Flat => {*map = Visibility::Visible; *globe = Visibility::Hidden;},
        WorldModel::Globe => {*map = Visibility::Hidden; *globe = Visibility::Visible;},
    };
}
