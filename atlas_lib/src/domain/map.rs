use atlas_macro::UiEditableEnum;
use bevy::prelude::*;

use crate::ui::UiEditableEnum;

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
    #[invisible] // TODO
    Resources,
    #[invisible]
    RealTopography,
    #[invisible]
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
            MapDataLayer::Resources => None,
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
pub const MAP_DATA_LAYERS: [MapDataLayer; 13] = [
    MapDataLayer::Preview,
    MapDataLayer::Continents,
    MapDataLayer::Topography,
    MapDataLayer::Temperature,
    MapDataLayer::Precipitation,
    MapDataLayer::Climate,
    MapDataLayer::Resources,
    MapDataLayer::RealTopography,
    MapDataLayer::TopographyFilter,
    // Influence
    MapDataLayer::ContinentsInfluence,
    MapDataLayer::TopographyInfluence,
    MapDataLayer::TemperatureInfluence,
    MapDataLayer::PrecipitationInfluence,
];

/// Array of all exportable [`MapDataLayer`]s.
pub const EXPORT_DATA_LAYERS: [(MapDataLayer, &str); 7] = [
    (MapDataLayer::Continents, "continents.png"),
    (MapDataLayer::Topography, "topography.png"),
    (MapDataLayer::RealTopography, "realtopography.png"),
    (MapDataLayer::Temperature, "temperature.png"),
    (MapDataLayer::Precipitation, "precipitation.png"),
    (MapDataLayer::Climate, "climate.png"),
    (MapDataLayer::Resources, "resources.png"),
];

/// A graphical overlay over the map, but not necessarily a full layer.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, UiEditableEnum)]
pub enum MapDataOverlay {
    #[default]
    None,
    StartPoints,
    Polities,
}
