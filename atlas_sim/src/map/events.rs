use atlas_lib::{
    base::events::{resize_helper, EventStruct},
    bevy::{prelude::*, utils::HashSet},
    config::{load_config, load_image, load_image_grey, AtlasConfig},
    domain::{
        graphics::{
            get_material_mut, make_image, MapGraphicsData, MapLogicData, WorldGlobeMesh, WorldMapMesh,
            PREVIEW_NAME,
        },
        map::{MapDataLayer, EXPORT_DATA_LAYERS},
    },
};
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::{
    config::{AtlasSimConfig, StartPointAlgorithm},
    map::internal::{data_to_view, CONFIG_NAME},
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
    logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasSimConfig>,
) {
    events.randomize_starts_request.take().expect("Always Some");
    // Generate tile weights.
    let conts = logics.get_layer(MapDataLayer::Continents);
    let (width, height) = config.get_world_size();
    let size = (width * height) as usize;
    let mut weights: Vec<f32> = conts.iter().map(|x| if *x <= 127 { 0.0 } else { 1.0 }).collect();
    // Factor in habitability if needed.
    match config.scenario.random_point_algorithm {
        StartPointAlgorithm::Weighted => {
            let climate = logics.get_layer(MapDataLayer::Climate);
            weights = climate
                .iter()
                .zip(weights.iter())
                .map(|(c, w)| fetch_climate(*c as usize, &config).habitability * w)
                .collect()
        }
        _ => {}
    };
    // Randomize all points.
    let table = WalkerTableBuilder::new(&weights).build();
    let mut used_positions = HashSet::<u32>::default();
    for point in &mut config.scenario.start_points {
        // Ensure that the position is not in use. If it is, try again. If that fails too, show an error.
        let mut success = false;
        for _ in 0..2 {
            let i = table.next() as u32;
            if !used_positions.contains(&i) {
                used_positions.insert(i);
                success = true;
                point.position[0] = i % height;
                point.position[1] = i / width;
                break;
            }
        }
        if !success {
            events.error_window =
                Some("Failed to choose unique random locations for all points. Try again.".to_string());
        }
    }
}
