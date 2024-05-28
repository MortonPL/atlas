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
