use bevy::prelude::*;

use atlas_lib::UiEditableEnum;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::{
        events::{
            check_event_changed, check_event_generate, check_event_loaded, check_event_regen,
            check_event_reset, check_event_saved, check_event_world_model, update_event_changed,
            update_event_generate, update_event_loaded, update_event_regen, update_event_reset,
            update_event_saved, update_event_world_model,
        },
        internal::{
            make_image, spawn_default_globe, spawn_default_plane, MapGraphicsData, MapGraphicsLayer,
            MapLogicData,
        },
    },
};

/// Plugin responsible for the world graphics and generation.
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
            .add_systems(Update, update_event_regen.run_if(check_event_regen))
            .add_systems(Update, update_event_generate.run_if(check_event_generate));
    }
}

/// A map layer containing some sort of data.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, UiEditableEnum)]
pub enum MapDataLayer {
    #[default]
    Preview,
    Continents,
    Topography,
    Temperature,
    Precipitation,
    Climate,
    Fertility,
    Resource,
    Richness,
    //#[invisible] // DEBUG
    RealTopography,
    //#[invisible] // DEBUG
    TopographyFilter,
    // Influence
    ContinentsInfluence,
    TopographyInfluence,
    TemperatureInfluence,
    PrecipitationInfluence,
}

impl MapDataLayer {
    pub fn get_influence_layer(&self) -> Option<Self> {
        match self {
            MapDataLayer::Preview => None,
            MapDataLayer::Continents => Some(MapDataLayer::ContinentsInfluence),
            MapDataLayer::Topography => Some(MapDataLayer::TopographyInfluence),
            MapDataLayer::Temperature => Some(MapDataLayer::TemperatureInfluence),
            MapDataLayer::Precipitation => Some(MapDataLayer::PrecipitationInfluence),
            MapDataLayer::Climate => None,
            MapDataLayer::Fertility => todo!(),
            MapDataLayer::Resource => todo!(),
            MapDataLayer::Richness => todo!(),
            MapDataLayer::RealTopography => None,
            MapDataLayer::TopographyFilter => None,
            MapDataLayer::ContinentsInfluence => Some(MapDataLayer::ContinentsInfluence),
            MapDataLayer::TopographyInfluence => Some(MapDataLayer::TopographyInfluence),
            MapDataLayer::TemperatureInfluence => Some(MapDataLayer::TemperatureInfluence),
            MapDataLayer::PrecipitationInfluence => Some(MapDataLayer::PrecipitationInfluence),
        }
    }
}

/// Array of all [`MapDataLayer`] variants.
const MAP_DATA_LAYERS: [MapDataLayer; 15] = [
    MapDataLayer::Preview,
    MapDataLayer::Continents,
    MapDataLayer::Topography,
    MapDataLayer::Temperature,
    MapDataLayer::Precipitation,
    MapDataLayer::Climate,
    MapDataLayer::Fertility,
    MapDataLayer::Resource,
    MapDataLayer::Richness,
    MapDataLayer::RealTopography,
    MapDataLayer::TopographyFilter,
    // Influence
    MapDataLayer::ContinentsInfluence,
    MapDataLayer::TopographyInfluence,
    MapDataLayer::TemperatureInfluence,
    MapDataLayer::PrecipitationInfluence,
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
    let empty_texture = make_image(1, 1, vec![0, 0, 0, 255]);
    let empty_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(empty_texture.clone())),
        unlit: true,
        ..default()
    });
    graphics.empty_material = empty_material;
    // Initialize all graphic and logical map layers.
    for layer in MAP_DATA_LAYERS {
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(empty_texture.clone())),
            unlit: true,
            ..default()
        });
        graphics
            .layers
            .insert(layer, MapGraphicsLayer::new(material.clone()));
        logics.put_layer(layer, vec![]);
    }
}

/// Startup system
///
/// Spawn the map and globe world models.
fn startup_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: ResMut<MapGraphicsData>,
    config: Res<SessionConfig>,
    mut events: ResMut<EventStruct>,
) {
    spawn_default_globe(&mut commands, &mut meshes, &graphics);
    spawn_default_plane(&mut commands, &mut meshes, &graphics);
    // Trigger model change.
    events.world_model_changed = Some(config.general.world_model.clone());
}
