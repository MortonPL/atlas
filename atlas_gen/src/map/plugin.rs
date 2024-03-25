use bevy::prelude::*;

use atlas_lib::UiEditableEnum;

use crate::{
    config::{save_image, GeneratorConfig, WorldModel},
    event::EventStruct,
    map::internal::{
        get_material, get_material_mut, magic_convert_data_to_png, magic_convert_png_to_data,
        make_default_material, make_image, spawn_default_globe, spawn_default_plane, CurrentWorldModel,
        MapGraphicsData, MapGraphicsLayer, MapLogicData, WorldGlobeMesh, WorldMapMesh,
    },
};

/// Plugin responsible for the world map graphics.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapGraphicsData>()
            .init_resource::<MapLogicData>()
            .add_systems(Startup, startup_layers)
            .add_systems(Startup, startup_model.after(startup_layers))
            .add_systems(Update, update_event_world_model.run_if(check_event_world_model))
            .add_systems(Update, update_event_changed.run_if(check_event_changed))
            .add_systems(Update, update_event_loaded.run_if(check_event_loaded))
            .add_systems(Update, update_event_saved.run_if(check_event_saved))
            .add_systems(Update, update_event_reset.run_if(check_event_reset))
            .add_systems(Update, update_event_regen.run_if(check_event_regen));
    }
}

/// Which layer is currently visible in the viewport.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, UiEditableEnum)]
pub enum ViewedMapLayer {
    #[default]
    Pretty,
    Continents,
    Topography,
    Temperature,
    Humidity,
    Climate,
    Fertility,
    Resource,
    Richness,
}

/// Array of all [`ViewedMapLayer`] variants.
const VIEWED_MAP_LAYERS: [ViewedMapLayer; 9] = [
    ViewedMapLayer::Pretty,
    ViewedMapLayer::Continents,
    ViewedMapLayer::Topography,
    ViewedMapLayer::Temperature,
    ViewedMapLayer::Humidity,
    ViewedMapLayer::Climate,
    ViewedMapLayer::Fertility,
    ViewedMapLayer::Resource,
    ViewedMapLayer::Richness,
];

/// Startup system
///
/// Initialize each map layer.
fn startup_layers(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut graphics: ResMut<MapGraphicsData>,
    mut logics: ResMut<MapLogicData>,
) {
    // Create the default texture and material.
    let (empty_texture, empty_material) = make_default_material(&mut materials, &mut images);
    graphics.empty_material = empty_material;
    // Initialize all graphic and logical map layers.
    for layer in VIEWED_MAP_LAYERS {
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(empty_texture.clone())),
            ..default()
        });
        graphics
            .layers
            .insert(layer, MapGraphicsLayer::new(material.clone()));
        logics.layers.insert(layer, vec![]);
    }
}

/// Startup system
///
/// Spawn the map and globe world models.
fn startup_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: ResMut<MapGraphicsData>,
    config: Res<GeneratorConfig>,
    mut events: ResMut<EventStruct>,
) {
    spawn_default_globe(&mut commands, &mut meshes, &graphics);
    spawn_default_plane(&mut commands, &mut meshes, &graphics);
    // Trigger model change.
    events.world_model_changed = Some(config.general.world_model.clone());
}

/// Run Condition
///
/// Check if "change world model" UI event needs handling.
fn check_event_world_model(events: Res<EventStruct>) -> bool {
    events.world_model_changed.is_some()
}

/// Update system
///
/// Handle "change world model" UI event.
fn update_event_world_model(
    mut commands: Commands,
    mut events: ResMut<EventStruct>,
    mut map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    mut globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    mut graphics: ResMut<MapGraphicsData>,
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
        }
        WorldModel::Globe(_) => {
            *map_vis = Visibility::Hidden;
            *globe_vis = Visibility::Visible;
            commands.entity(globe_en).insert(CurrentWorldModel);
            commands.entity(map_en).remove::<CurrentWorldModel>();
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
fn check_event_changed(events: Res<EventStruct>) -> bool {
    events.viewed_layer_changed.is_some()
}

/// Update system
///
/// Assign respective layer material to the world model.
fn update_event_changed(
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
fn check_event_regen(events: Res<EventStruct>) -> bool {
    events.regen_layer_request.is_some()
}

/// Update system
///
/// Regenerate graphical layer based on logical layer data.
fn update_event_regen(
    mut events: ResMut<EventStruct>,
    config: ResMut<GeneratorConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    logics: Res<MapLogicData>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layer = events.regen_layer_request.take().expect("Always Some");
    // Convert logical data to image data.
    let mut data = magic_convert_data_to_png(&logics, layer);
    // Fetch handles.
    let layer = graphics.get_layer_mut(layer);
    let material = get_material_mut(&mut materials, &layer.material);
    // Assign new texture.
    let (width, height) = config.general.world_model.get_dimensions();
    let image = images.add(make_image(width, height, std::mem::take(&mut data)));
    material.base_color_texture = Some(image);
    // Graphical layer becomes valid again.
    layer.invalid = false;
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}

/// Run condition
///
/// Check if "load layer image" event needs handling.
fn check_event_loaded(events: Res<EventStruct>) -> bool {
    events.load_layer_request.is_some()
}

/// Update system
///
/// Load new layer data.
fn update_event_loaded(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut graphics: ResMut<MapGraphicsData>,
) {
    let (layer, data) = events.load_layer_request.take().expect("Always Some");
    let graphic_layer = graphics.get_layer_mut(layer);
    // Convert image data to logic data.
    let data = magic_convert_png_to_data(&data, layer);
    // Assign data.
    logics.layers.insert(layer, data);
    // Trigger texture regeneration.
    graphic_layer.invalid = true;
    events.regen_layer_request = Some(layer);
}

/// Run condition
///
/// Check if "save layer image" event needs handling.
fn check_event_saved(events: Res<EventStruct>) -> bool {
    events.save_layer_request.is_some()
}

/// Update system
///
/// Save new layer data.
fn update_event_saved(
    mut events: ResMut<EventStruct>,
    config: ResMut<GeneratorConfig>,
    mut graphics: ResMut<MapGraphicsData>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
fn check_event_reset(events: Res<EventStruct>) -> bool {
    events.reset_layer_request.is_some()
}

/// Update system
///
/// Reset/Invalidate layer data.
fn update_event_reset(mut events: ResMut<EventStruct>, mut graphics: ResMut<MapGraphicsData>) {
    let layer = events.reset_layer_request.take().expect("Always Some");
    let layer = graphics.get_layer_mut(layer);
    // Invalidate.
    layer.invalid = true;
    // Trigger material refresh.
    events.viewed_layer_changed = Some(graphics.current);
}
