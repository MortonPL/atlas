use bevy::prelude::*;

use crate::{
    config::{save_image, SessionConfig, WorldModel},
    event::EventStruct,
    map::{
        generation::generate,
        internal::{
            data_to_png, get_material, get_material_mut, make_image, png_to_data, CurrentWorldModel,
            MapGraphicsData, MapLogicData, WorldGlobeMesh, WorldMapMesh,
        },
    },
};

use super::{generation::after_generate, ViewedMapLayer};

/// Run Condition
///
/// Check if "change world model" UI event needs handling.
pub fn check_event_world_model(events: Res<EventStruct>) -> bool {
    events.world_model_changed.is_some()
}

/// Update system
///
/// Handle "change world model" UI event.
pub fn update_event_world_model(
    mut commands: Commands,
    mut events: ResMut<EventStruct>,
    mut map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    mut globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    mut graphics: ResMut<MapGraphicsData>,
    mut logics: ResMut<MapLogicData>,
) {
    // Run queries.
    let (map_en, mut map_vis, mut map_tran) = map.single_mut();
    let (globe_en, mut globe_vis) = globe.single_mut();
    // Switch model visibility and tags.
    let model = events.world_model_changed.take().expect("Always Some");
    match model {
        WorldModel::Flat(x) => {
            *map_vis = Visibility::Visible;
            *globe_vis = Visibility::Hidden;
            map_tran.scale.x = x.world_size[0] as f32 / 100.0;
            map_tran.scale.z = x.world_size[1] as f32 / 100.0;
            commands.entity(map_en).insert(CurrentWorldModel);
            commands.entity(globe_en).remove::<CurrentWorldModel>();
            logics.resize_all_layers((x.world_size[0] * x.world_size[1]) as usize);
        }
        WorldModel::Globe(_) => {
            *map_vis = Visibility::Hidden;
            *globe_vis = Visibility::Visible;
            commands.entity(globe_en).insert(CurrentWorldModel);
            commands.entity(map_en).remove::<CurrentWorldModel>();
            // Resize all layers according to world size // TODO
        }
    }
    // Invalidate all layers - world models have different world size rules.
    for layer in graphics.layers.values_mut() {
        layer.invalid = true;
    }
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}

/// Run Condition
///
/// Check if "change viewed layer" UI event needs handling.
pub fn check_event_changed(events: Res<EventStruct>) -> bool {
    events.viewed_layer_changed.is_some()
}

/// Update system
///
/// Assign respective layer material to the world model.
pub fn update_event_changed(
    mut events: ResMut<EventStruct>,
    mut graphics: ResMut<MapGraphicsData>,
    mut world: Query<&mut Handle<StandardMaterial>, With<CurrentWorldModel>>,
) {
    // Set layer as current.
    let layer = events.viewed_layer_changed.take().expect("Always Some");
    graphics.current = layer;
    // Change worls model's material to this layer's material.
    let layer = graphics.get_layer_mut(layer);
    let mut mat = world.single_mut();
    *mat = if layer.invalid {
        graphics.empty_material.clone()
    } else {
        layer.material.clone()
    };
}

/// Run condition
///
/// Check if "regen layer image" event needs handling.
pub fn check_event_regen(events: Res<EventStruct>) -> bool {
    events.regen_layer_request.is_some()
}

/// Update system
///
/// Regenerate graphical layer based on logical layer data.
pub fn update_event_regen(
    mut events: ResMut<EventStruct>,
    config: ResMut<SessionConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    logics: Res<MapLogicData>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layers = events.regen_layer_request.take().expect("Always Some");
    for layer in layers {
        // Convert logical data to image data.
        let mut data = data_to_png(&logics, layer);
        // Fetch handles.
        let layer = graphics.get_layer_mut(layer);
        let material = get_material_mut(&mut materials, &layer.material);
        // Assign new texture.
        let (width, height) = config.general.world_model.get_dimensions();
        let image = images.add(make_image(width, height, std::mem::take(&mut data)));
        material.base_color_texture = Some(image);
        // Graphical layer becomes valid again.
        layer.invalid = false;
    }
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}

/// Run condition
///
/// Check if "load layer image" event needs handling.
pub fn check_event_loaded(events: Res<EventStruct>) -> bool {
    events.load_layer_request.is_some()
}

/// Update system
///
/// Load new layer data.
pub fn update_event_loaded(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    config: Res<SessionConfig>,
) {
    let (layer, data) = events.load_layer_request.take().expect("Always Some");
    // Convert image data to logic data.
    let data = png_to_data(data, layer);
    // Assign data.
    logics.put_layer(layer, data);
    // Handle post generation.
    post_generation(layer, &mut logics, &mut events, &config, vec![layer]);
}

/// Run condition
///
/// Check if "save layer image" event needs handling.
pub fn check_event_saved(events: Res<EventStruct>) -> bool {
    events.save_layer_request.is_some()
}

/// Update system
///
/// Save new layer data.
pub fn update_event_saved(
    mut events: ResMut<EventStruct>,
    config: Res<SessionConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    images: Res<Assets<Image>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let (layer, path) = events.save_layer_request.take().expect("Always Some");
    let layer = graphics.get_layer_mut(layer);
    // Don't try to save an invalid layer.
    if layer.invalid {
        return; // TODO handle nicely
    }
    // Access the layer's material's texture.
    let material = get_material(&materials, &layer.material);
    let image = material
        .base_color_texture
        .clone()
        .expect("Material should have a texture");
    let image = images.get(image).expect("Image handle should be valid");
    // Save the texture with correct dimensions.
    let (width, height) = config.general.world_model.get_dimensions();
    save_image(path, &image.data, width, height).unwrap(); // TODO error handling
}

/// Run condition
///
/// Check if "reset layer image" event needs handling.
pub fn check_event_reset(events: Res<EventStruct>) -> bool {
    events.reset_layer_request.is_some()
}

/// Update system
///
/// Reset/Invalidate layer data.
pub fn update_event_reset(mut events: ResMut<EventStruct>, mut graphics: ResMut<MapGraphicsData>) {
    let layer = events.reset_layer_request.take().expect("Always Some");
    let layer = graphics.get_layer_mut(layer);
    // Invalidate.
    layer.invalid = true;
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}

/// Run condition
///
/// Check if "generate layer data" event needs handling.
pub fn check_event_generate(events: Res<EventStruct>) -> bool {
    events.generate_request.is_some()
}

/// Update system
///
/// Generate layer data.
pub fn update_event_generate(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    config: Res<SessionConfig>,
) {
    let layer = events.generate_request.take().expect("Always Some");
    // Run generation procedure based on generator type and layer.
    let regen_layers = generate(layer, &mut logics, &config);
    // Handle post generation.
    post_generation(layer, &mut logics, &mut events, &config, regen_layers);
}

/// Helper function
///
/// Regenerate dependant layers.
fn post_generation(
    layer: ViewedMapLayer,
    logics: &mut MapLogicData,
    events: &mut EventStruct,
    config: &SessionConfig,
    mut regen_layers: Vec<ViewedMapLayer>,
) {
    // Adjust other layers if needed.
    let regen_layers_2 = after_generate(layer, logics, config);
    regen_layers.extend(regen_layers_2);
    // Trigger texture regeneration.
    events.regen_layer_request = Some(regen_layers);
}
