use std::f32::consts::FRAC_PI_2;

use atlas_lib::{
    base::{events::EventStruct, map::resize_helper, ui::UiStateBase},
    bevy::{prelude::*, render::mesh::PlaneMeshBuilder},
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::{
        load_config, load_image, load_image_grey,
        sim::{AtlasSimConfig, CONFIG_NAME},
        AtlasConfig,
    },
    domain::{
        graphics::{MapLogicData, WorldGlobeMesh, WorldMapMesh},
        map::{MapDataLayer, MapDataOverlay, EXPORT_DATA_LAYERS},
    },
};
use bevy_mod_picking::{prelude::*, PickableBundle};

use crate::{
    map::internal::{
        calc_start_point_weights, create_overlays, randomize_point_civ, randomize_point_color,
        randomize_start_points,
    },
    sim::{
        polity::{City, Ownership, Polity},
        SimControl, SimMapData,
    },
    ui::{MapOverlay, UpdateSelectionEvent},
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
    // Import all layers.
    let (width, height) = (config.general.world_size[0], config.general.world_size[1]);
    let mut regen_layers = vec![];
    for (layer, name) in EXPORT_DATA_LAYERS {
        let path = base_path.join(name);
        let result = match layer {
            MapDataLayer::Preview => load_image(path, width, height),
            MapDataLayer::Resources => load_image(path, width, height),
            _ => load_image_grey(path, width, height),
        };
        match result {
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
    query: Query<(Entity, &MapOverlay), With<MapOverlay>>,
    commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut ui_base: ResMut<UiStateBase>,
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
    randomize_point_civ(&mut config, rng.as_mut());
    randomize_point_color(&mut config, rng.as_mut());
    // Recreate overlay markers.
    create_overlays(&config, commands, &mut meshes, &mut materials, query);
    // Force show start point overlay.
    ui_base.overlays[0] = true;
    events.viewed_overlay_changed = Some((ui_base.overlays.clone(), false));
}

/// Update system
///
/// Switch the currently visible overlay.
pub fn update_event_overlay_changed(
    mut events: ResMut<EventStruct>,
    mut query: Query<(&mut Visibility, &MapOverlay), With<MapOverlay>>,
) {
    let (mask, deferred) = events.viewed_overlay_changed.take().expect("Always Some");
    if deferred {
        events.viewed_overlay_changed = Some((mask, false));
        return;
    }
    let mask = mask.map(|x| {
        if x {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    });
    // Show some overlays.
    for (mut vis, overlay) in query.iter_mut() {
        *vis = match overlay.overlay {
            MapDataOverlay::None => Visibility::Visible,
            MapDataOverlay::StartPoints => mask[0],
            MapDataOverlay::Polities => mask[1],
            MapDataOverlay::Cities => mask[2],
        };
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut ui_base: ResMut<UiStateBase>,
) {
    events.simulation_start_request.take();
    sim.paused = false;
    // Spawn polities.
    for start in &config.scenario.start_points {
        // Get all coords.
        let p = (start.position[0], start.position[1]);
        let i = config.map_to_index(p);
        let pw = config.map_to_world_centered(p);
        // Prep polity component.
        let mut polity = Polity {
            ownership: Ownership::Independent,
            color: Color::rgb_u8(
                start.polity.color[0],
                start.polity.color[1],
                start.polity.color[2],
            ),
            need_visual_update: true,
            land_claim_points: config.rules.starting_land_claim_points,
            population: start.polity.population,
            ..Default::default()
        };
        // Claim initial tile.
        polity.claim_tile(i, None, &mut extras, &config);
        polity.update_jobs(&config);
        polity.update_resources(&config);
        // Spawn.
        let ec = commands.spawn((
            polity,
            PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder::new(Direction3d::Y, Vec2::ONE).build()),
                material: materials.add(StandardMaterial::default()),
                transform: Transform::from_xyz(pw.0, pw.1, 0.01)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, FRAC_PI_2, 0.0, 0.0))
                    .with_scale(Vec3::new(0.01, 0.01, 0.01)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            MapOverlay::new(MapDataOverlay::Polities),
        ));
        // Post spawn actions.
        let polity_entity = ec.id();
        extras.tile_owner[i as usize] = Some(polity_entity.clone());
        // Create initial city.
        let city = City {
            owner: polity_entity,
            level: 1.0,
            need_visual_update: true,
        };
        commands.spawn((
            city,
            PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder::new(Direction3d::Y, Vec2::ONE / 50.0)),
                material: materials.add(StandardMaterial::default()),
                transform: Transform::from_xyz(pw.0, pw.1, 0.02).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    FRAC_PI_2,
                    0.0,
                    0.0,
                )),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::send_event::<UpdateSelectionEvent>(),
            MapOverlay::new(MapDataOverlay::Cities),
        ));
    }
    // Force hide start point overlay.
    ui_base.overlays[0] = false;
    events.viewed_overlay_changed = Some((ui_base.overlays.clone(), true));
}
