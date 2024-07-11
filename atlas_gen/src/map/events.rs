use atlas_lib::{
    base::{events::EventStruct, map::resize_helper}, bevy::prelude::*, bevy_prng::WyRand, bevy_rand::resource::GlobalEntropy, config::{
        load_config, load_image, load_image_grey, save_config, save_image, save_image_grey, AtlasConfig,
    }, domain::{
        graphics::{
            get_material, MapGraphicsData, MapLogicData, WorldGlobeMesh, WorldMapMesh, CLIMATEMAP_NAME,
            CLIMATEMAP_SIZE,
        },
        map::{MapDataLayer, EXPORT_DATA_LAYERS},
    }
};

use crate::{
    config::{AtlasGenConfig, CONFIG_NAME},
    map::generation::{after_generate, generate},
};

/// Run condition
///
/// Check if "load layer data" event needs handling.
pub fn check_event_loaded(events: Res<EventStruct>) -> bool {
    events.load_layer_request.is_some()
}

/// Run condition
///
/// Check if "save layer data" event needs handling.
pub fn check_event_saved(events: Res<EventStruct>) -> bool {
    events.save_layer_request.is_some()
}

/// Run condition
///
/// Check if "save layer image" event needs handling.
pub fn check_event_rendered(events: Res<EventStruct>) -> bool {
    events.render_layer_request.is_some()
}

/// Run condition
///
/// Check if "clear layer image" event needs handling.
pub fn check_event_clear(events: Res<EventStruct>) -> bool {
    events.clear_layer_request.is_some()
}

/// Run condition
///
/// Check if "generate layer data" event needs handling.
pub fn check_event_generate(events: Res<EventStruct>) -> bool {
    events.generate_request.is_some()
}

/// Run condition
///
/// Check if "reload climatemap.png" event needs handling.
pub fn check_event_climatemap(events: Res<EventStruct>) -> bool {
    events.load_climatemap_request.is_some()
}

/// Run condition
///
/// Check if "import world" event needs handling.
pub fn check_event_import(events: Res<EventStruct>) -> bool {
    events.import_world_request.is_some()
}

/// Run condition
///
/// Check if "export world" event needs handling.
pub fn check_event_export(events: Res<EventStruct>) -> bool {
    events.export_world_request.is_some()
}

/// Update system
///
/// Load new layer data.
pub fn update_event_loaded(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasGenConfig>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let (layer, data) = events.load_layer_request.take().expect("Always Some");
    // Assign data.
    logics.put_layer(layer, data);
    // Handle post generation, which refreshes the texture and dependant layers.
    post_generation(layer, &mut logics, &mut events, &mut config, vec![layer], &mut rng);
}

/// Update system
///
/// Save layer data.
pub fn update_event_saved(
    mut events: ResMut<EventStruct>,
    logics: Res<MapLogicData>,
    config: Res<AtlasGenConfig>,
) {
    let (layer, path) = events.save_layer_request.take().expect("Always Some");
    // Get layer data.
    let data = logics.get_layer(layer);
    let (width, height) = config.get_world_size();
    // Save in color for preview (purely cosmetic) or resources (specially coded), otherwise in greyscale.
    let result = match layer {
        MapDataLayer::Preview => save_image(path, data, width, height),
        MapDataLayer::Resources => save_image(path, data, width, height),
        _ => save_image_grey(path, data, width, height),
    };
    events.error_window = result.err().map(|x| x.to_string());
}

/// Update system
///
/// Save visual layer data.
pub fn update_event_rendered(
    mut events: ResMut<EventStruct>,
    config: Res<AtlasGenConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    images: Res<Assets<Image>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let (layer, path) = events.render_layer_request.take().expect("Always Some");
    let layer = graphics.get_layer_mut(layer);
    // Don't try to save an invalid layer.
    if layer.invalid {
        events.error_window = Some("Cannot render an invalid/uninitialized layer!".to_string());
        return;
    }
    // Access the layer's material's texture.
    let material = get_material(&materials, &layer.material);
    let image = material
        .base_color_texture
        .clone()
        .expect("Material should have a texture");
    let image = images.get(image).expect("Image handle should be valid");
    // Save the texture with correct dimensions.
    let (width, height) = config.get_world_size();
    let result = save_image(path, &image.data, width, height);
    events.error_window = result.err().map(|x| x.to_string());
}

/// Update system
///
/// Clear layer data.
pub fn update_event_clear(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut graphics: ResMut<MapGraphicsData>,
) {
    let layer = events.clear_layer_request.take().expect("Always Some");
    // Fill logic layer with 0s, mark texture for regeneration.
    logics.get_layer_mut(layer).fill(0);
    graphics.get_layer_mut(layer).invalid = true;
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}

/// Update system
///
/// Generate layer data.
pub fn update_event_generate(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasGenConfig>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let (layer, regen_influence) = events.generate_request.take().expect("Always Some");
    let mut regen_layers: Vec<MapDataLayer> = vec![];
    // If this layer has an associated influence layer, forcefully regenerate it as well.
    if regen_influence {
        if let Some(layer2) = layer.get_influence_layer() {
            regen_layers.extend(generate(layer2, &mut logics, &mut config, &mut rng));
        }
    }
    // Run generation procedure based on generator type and layer.
    regen_layers.extend(generate(layer, &mut logics, &mut config, &mut rng));
    // Handle post generation.
    post_generation(layer, &mut logics, &mut events, &mut config, regen_layers, &mut rng);
}

/// Update system
///
/// Reload climatemap.png.
pub fn update_event_climatemap(mut events: ResMut<EventStruct>, mut logics: ResMut<MapLogicData>) {
    events.load_climatemap_request.take();
    let result = logics.load_climatemap();
    events.error_window = result.err().map(|x| x.to_string());
}

/// Update system
///
/// Import the world: preview, all layers, config, and climate map.
pub fn update_event_import(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasGenConfig>,
    map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    commands: Commands,
) {
    let base_path = events.import_world_request.take().expect("Always Some");
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
    // Import climate map.
    let path = base_path.join(CLIMATEMAP_NAME);
    match load_image_grey(path, CLIMATEMAP_SIZE as u32, CLIMATEMAP_SIZE as u32) {
        Ok(data) => logics.set_climatemap(data),
        Err(error) => {
            events.error_window = Some(error.to_string());
            return;
        }
    };
    // Resize if needed.
    resize_helper(commands, config.as_ref(), map, globe, logics);
    // Refresh layers.
    events.regen_layer_request = Some(regen_layers);
}

/// Update system
///
/// Export the world: preview, all layers, config, and climate map.
pub fn update_event_export(
    mut events: ResMut<EventStruct>,
    logics: ResMut<MapLogicData>,
    config: Res<AtlasGenConfig>,
) {
    let base_path = events.export_world_request.take().expect("Always Some");
    // Export all layers.
    let (width, height) = (config.general.world_size[0], config.general.world_size[1]);
    for (layer, name) in EXPORT_DATA_LAYERS {
        let data = logics.get_layer(layer);
        let path = base_path.join(name);
        let result = match layer {
            MapDataLayer::Preview => save_image(path, data, width, height),
            _ => save_image_grey(path, data, width, height),
        };
        events.error_window = result.err().map(|x| x.to_string());
        if events.error_window.is_some() {
            return;
        }
    }
    // Export climate map.
    let climatemap = logics.get_climatemap();
    let path = base_path.join(CLIMATEMAP_NAME);
    let result = save_image_grey(path, climatemap, CLIMATEMAP_SIZE as u32, CLIMATEMAP_SIZE as u32);
    events.error_window = result.err().map(|x| x.to_string());
    if events.error_window.is_some() {
        return;
    }
    // Export config.
    let path = base_path.join(CONFIG_NAME);
    let result = save_config(config.as_ref(), path);
    events.error_window = result.err().map(|x| x.to_string());
}

/// Helper function
///
/// Regenerate dependant layers.
fn post_generation(
    layer: MapDataLayer,
    logics: &mut MapLogicData,
    events: &mut EventStruct,
    config: &mut AtlasGenConfig,
    mut regen_layers: Vec<MapDataLayer>,
    rng: &mut GlobalEntropy<WyRand>,
) {
    // Adjust other layers if needed.
    let regen_layers_2 = after_generate(layer, logics, config, rng);
    regen_layers.extend(regen_layers_2);
    // Trigger texture regeneration.
    events.regen_layer_request = Some(regen_layers);
}
