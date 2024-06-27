use atlas_lib::{
    base::{events::EventStruct, map::resize_helper},
    bevy::prelude::*,
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::{load_config, load_image, load_image_grey, AtlasConfig},
    domain::{
        graphics::{MapLogicData, WorldGlobeMesh, WorldMapMesh, PREVIEW_NAME},
        map::{MapDataLayer, MapDataOverlay, EXPORT_DATA_LAYERS},
    },
};

use crate::{
    config::{AtlasSimConfig, CONFIG_NAME},
    map::internal::{calc_start_point_weights, randomize_start_points, randomize_civ_points},
    ui::MapOverlay,
};

/// Run condition
///
/// Check if "import initial world" event needs handling.
pub fn check_event_import_start(events: Res<EventStruct>) -> bool {
    events.import_start_request.is_some()
}

/// Run condition
///
/// Check if "randomize start points" event needs handling.
pub fn check_event_random_start(events: Res<EventStruct>) -> bool {
    events.randomize_starts_request.is_some()
}

/// Run condition
///
/// Check if "changed viewed overlay" event needs handling.
pub fn check_event_overlay_changed(events: Res<EventStruct>) -> bool {
    events.viewed_overlay_changed.is_some()
}

/// Update system
///
/// Import the initial world for simulation.
pub fn update_event_import_start(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasSimConfig>,
    map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    commands: Commands,
) {
    let base_path = events.import_start_request.take().expect("Always Some");
    // Import config.
    let path = base_path.join(CONFIG_NAME);
    match load_config(path) {
        Ok(data) => {
            *config = data;
            events.world_model_changed = Some(());
        }
        Err(error) => {
            events.error_window = Some(error.to_string());
            return;
        }
    }
    // Import data layers.
    let (width, height) = (config.general.world_size[0], config.general.world_size[1]);
    let mut regen_layers = vec![];
    for (layer, name) in EXPORT_DATA_LAYERS {
        let path = base_path.join(name);
        match load_image_grey(path, width, height) {
            Ok(data) => {
                logics.put_layer(layer, data);
                regen_layers.push(layer);
            }
            Err(error) => {
                events.error_window = Some(error.to_string());
                return;
            }
        };
    }
    // Import preview.
    let path = base_path.join(PREVIEW_NAME);
    match load_image(path, width, height) {
        Ok(data) => logics.put_layer(MapDataLayer::Preview, data),
        Err(error) => {
            events.error_window = Some(error.to_string());
            return;
        }
    };
    regen_layers.push(MapDataLayer::Preview);
    // Resize if needed.
    resize_helper(commands, config.as_ref(), map, globe, logics);
    // Refresh layers.
    events.regen_layer_request = Some(regen_layers);
}

/// Update system
///
/// Randomize scenario start points.
pub fn update_event_random_start(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasSimConfig>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    events.randomize_starts_request.take().expect("Always Some");
    let (width, height) = config.get_world_size();
    let (width, height) = (width as usize, height as usize);
    // Generate tile weights.
    let (weights, strip_weights) = calc_start_point_weights(&config, &mut logics, width, height);
    // Randomize starting points.
    if !randomize_start_points(&mut config, rng.as_mut(), &weights, &strip_weights, width) {
        events.error_window =
            Some("Failed to choose unique random locations for all points. Try again.".to_string());
    }
    randomize_civ_points(&mut config, rng.as_mut());
    // Switch / refresh overlay.
    events.viewed_overlay_changed = Some(MapDataOverlay::StartPoints);

    // DEBUG
    let weights = weights
        .into_iter()
        .map(|x| ((x * 255) / 9000000).min(255) as u8)
        .collect();
    logics.put_layer(MapDataLayer::ContinentsInfluence, weights);
    events.regen_layer_request = Some(vec![MapDataLayer::ContinentsInfluence]);
}

/// Update system
///
/// Switch the currently visible overlay.
pub fn update_event_overlay_changed(
    mut events: ResMut<EventStruct>,
    mut commands: Commands,
    mut query: Query<Entity, With<MapOverlay>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<AtlasSimConfig>,
) {
    let overlay = events.viewed_overlay_changed.take().expect("Always Some");
    // Clear all overlays.
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
    match overlay {
        MapDataOverlay::None => { /* do nothing */ }
        MapDataOverlay::StartPoints => {
            let mesh = meshes.add(Cuboid::from_size(Vec3::ONE / 50.0).mesh());
            let material = materials.add(StandardMaterial {
                base_color: Color::RED,
                unlit: true,
                ..Default::default()
            });
            for point in &config.scenario.start_points {
                let coords = config.map_to_world((point.position[0], point.position[1]));
                commands.spawn((
                    MaterialMeshBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform::from_xyz(coords.0, coords.1, 0.0),
                        ..default()
                    },
                    MapOverlay,
                ));
            }
        }
    }
}
