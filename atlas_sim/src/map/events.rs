use std::{cmp::min, f32::consts::FRAC_PI_2};

use atlas_lib::{
    base::{events::EventStruct, map::resize_helper},
    bevy::{prelude::*, render::mesh::PlaneMeshBuilder},
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
    map::internal::{
        calc_start_point_weights, create_overlays, randomize_civ_points, randomize_start_points,
    },
    sim::{
        polity::{Ownership, Polity},
        SimControl, SimMapData,
    },
    ui::{MapOverlay, MapOverlayPolity, MapOverlayStart},
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

/// Run condition
///
/// Check if "start simulation" event needs handling.
pub fn check_event_start_simulation(events: Res<EventStruct>) -> bool {
    events.simulation_start_request.is_some()
}

/// Update system
///
/// Import the initial world for simulation.
pub fn update_event_import_start(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasSimConfig>,
    mut extras: ResMut<SimMapData>,
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
    extras.tile_owner.resize((width * height) as usize, None);
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<MapOverlayStart>>,
    commands: Commands,
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
    // Recreate overlay markers.
    create_overlays(&config, commands, &mut meshes, &mut materials, query);
    // Switch / refresh overlay.
    events.viewed_overlay_changed = Some(MapDataOverlay::StartPoints);
}

/// Update system
///
/// Switch the currently visible overlay.
pub fn update_event_overlay_changed(
    mut events: ResMut<EventStruct>,
    mut q_starts: Query<&mut Visibility, (With<MapOverlayStart>, Without<MapOverlayPolity>)>,
    mut q_polities: Query<&mut Visibility, (With<MapOverlayPolity>, Without<MapOverlayStart>)>,
) {
    let overlay = events.viewed_overlay_changed.take().expect("Always Some");
    // Hide all overlays.
    for mut vis in q_starts.iter_mut() {
        *vis = Visibility::Hidden;
    }
    for mut vis in q_polities.iter_mut() {
        *vis = Visibility::Hidden;
    }
    // Show some overlays.
    match overlay {
        MapDataOverlay::None => { /* do nothing */ }
        MapDataOverlay::StartPoints => {
            for mut vis in q_starts.iter_mut() {
                *vis = Visibility::Visible;
            }
        }
        MapDataOverlay::Polities => {
            for mut vis in q_polities.iter_mut() {
                *vis = Visibility::Visible;
            }
        }
    }
}

/// Update system
///
/// Set up everything needed to run the simulation.
pub fn update_event_start_simulation(
    mut events: ResMut<EventStruct>,
    config: Res<AtlasSimConfig>,
    mut sim: ResMut<SimControl>,
    mut extras: ResMut<SimMapData>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    events.simulation_start_request.take();
    sim.paused = false;
    // Spawn polities.
    for start in &config.scenario.start_points {
        // Get all coords.
        let i = min(start.owner as usize, config.scenario.start_civs.len() - 1);
        let owner = &config.scenario.start_civs[i];
        let p = (start.position[0], start.position[1]);
        let i = config.map_to_index(p);
        let pw = config.map_to_world_centered(p);
        // Spawn.
        let ec = commands.spawn((
            Polity {
                tiles: vec![i],
                border_tiles: config.get_border_tiles(i),
                centroid: Vec2 {
                    x: p.0 as f32,
                    y: p.1 as f32,
                },
                xywh: [p.0, p.1, 1, 1],
                ownership: Ownership::Independent,
                color: [255, 0, 0], // TODO
                need_visual_update: true,
            },
            PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder::new(Direction3d::Y, Vec2::ONE).build()),
                material: materials.add(StandardMaterial::default()),
                transform: Transform::from_xyz(pw.0, pw.1, 0.0)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, FRAC_PI_2, 0.0, 0.0))
                    .with_scale(Vec3::new(0.01, 0.01, 0.01)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            MapOverlay,
            MapOverlayPolity,
        ));
        // Post spawn actions.
        extras.tile_owner[i as usize] = Some(ec.id());
    }
}
