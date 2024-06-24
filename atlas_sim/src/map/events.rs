use atlas_lib::{
    base::events::{resize_helper, EventStruct},
    bevy::{
        prelude::*,
        utils::{hashbrown::HashMap, HashSet},
    },
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::{load_config, load_image, load_image_grey, AtlasConfig},
    domain::{
        graphics::{
            get_material_mut, make_image, MapGraphicsData, MapLogicData, WorldGlobeMesh, WorldMapMesh,
            PREVIEW_NAME,
        },
        map::{MapDataLayer, MapDataOverlay, EXPORT_DATA_LAYERS},
    },
};
use weighted_rand::{
    builder::{NewBuilder, WalkerTableBuilder},
    table::WalkerTable,
};

use crate::{
    config::{AtlasSimConfig, StartPointAlgorithm},
    map::internal::{data_to_view, CONFIG_NAME},
    ui::MapOverlay,
};

use super::internal::fetch_climate;

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
/// Regenerate graphical layer based on logical layer data.
pub fn update_event_regen(
    mut events: ResMut<EventStruct>,
    config: ResMut<AtlasSimConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    logics: Res<MapLogicData>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layers = events.regen_layer_request.take().expect("Always Some");
    for layer in layers {
        // Convert logical data to image data.
        let mut data = data_to_view(&logics, layer, &config);
        // Fetch handles.
        let layer = graphics.get_layer_mut(layer);
        let material = get_material_mut(&mut materials, &layer.material);
        // Assign new texture.
        let (width, height) = (config.general.world_size[0], config.general.world_size[1]);
        let image = images.add(make_image(width, height, std::mem::take(&mut data)));
        material.base_color_texture = Some(image);
        // Graphical layer becomes valid again.
        layer.invalid = false;
    }
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
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
    // Generate tile weights.
    let conts = logics.get_layer(MapDataLayer::Continents);
    let (width, height) = config.get_world_size();
    let (width, height) = (width as usize, height as usize);
    let mut weights: Vec<u32> = conts.iter().map(|x| if *x <= 127 { 0 } else { 1 }).collect();
    // Factor in habitability if needed.
    match config.scenario.random_point_algorithm {
        StartPointAlgorithm::Weighted => {
            let climate = logics.get_layer(MapDataLayer::Climate);
            weights = climate
                .iter()
                .zip(weights.iter())
                .map(|(c, w)| (fetch_climate(*c as usize, &config).habitability * 1000.0) as u32 * w)
                .map(|x| x * x)
                .collect()
        }
        StartPointAlgorithm::WeightedArea => {
            let climate = logics.get_layer(MapDataLayer::Climate);
            let part_weights: Vec<u32> = climate
                .iter()
                .zip(weights.iter())
                .map(|(c, w)| (fetch_climate(*c as usize, &config).habitability * 1000.0) as u32 * w)
                .map(|x| x * x)
                .collect();
            for x in 1..(width - 1) {
                for y in 1..(height - 1) {
                    let i = width * y + x;
                    if part_weights[i] < 100 {
                        weights[i] = 0;
                    } else {
                        let (up, down) = (i - width, i + width);
                        weights[i] = part_weights[i - 1]
                            + part_weights[i]
                            + part_weights[i + 1]
                            + part_weights[up - 1]
                            + part_weights[up]
                            + part_weights[up + 1]
                            + part_weights[down - 1]
                            + part_weights[down]
                            + part_weights[down + 1];
                    }
                }
            }
        }
        _ => {}
    };
    // Prep one-tile-high strips.
    let strip_weights: Vec<u32> = weights.chunks(width).map(|x| x.iter().sum()).collect();
    let strip_table = WalkerTableBuilder::new(&strip_weights).build();
    let mut tables = HashMap::<usize, WalkerTable>::default();
    // Randomize all points.
    let mut used_positions = HashSet::<usize>::default();
    for point in &mut config.scenario.start_points {
        if point.locked {
            continue;
        }
        // Ensure that the position is not in use. If it is, try again. If that fails too, show an error.
        let mut success = false;
        for _ in 0..5 {
            // Get or make the table for this strip.
            let i = strip_table.next_rng(rng.as_mut());
            let table = if let Some(table) = tables.get(&i) {
                table
            } else {
                let start = i * width;
                let table = WalkerTableBuilder::new(&weights[start..(start + width)]).build();
                let (_, table) = tables.insert_unique_unchecked(i, table);
                table
            };
            let j = table.next_rng(rng.as_mut());
            let j = i * width + j;
            if !used_positions.contains(&j) {
                used_positions.insert(j);
                success = true;
                point.position[0] = (j % width) as u32;
                point.position[1] = (j / width) as u32;
                break;
            }
        }
        if !success {
            events.error_window =
                Some("Failed to choose unique random locations for all points. Try again.".to_string());
        }
    }
    // Switch / refresh overlay.
    events.viewed_overlay_changed = Some(MapDataOverlay::StartPoints);
    // DEBUG
    let weights = weights.into_iter().map(|x| ((x * 255) / 9000000).min(255) as u8).collect();
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
