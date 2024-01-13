use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct GeneratorConfig {
    pub general: GeneralConfig,
    pub topography: TopographyConfig,
    pub climate: ClimateConfig,
}

/// Config for the general map settings.
#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct GeneralConfig {
    pub world_model: WorldModel,
    pub tile_resolution: f32,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            world_model: Default::default(),
            tile_resolution: 100.0,
        }
    }
}

/// World model describes the geometric model of the world which
/// impacts the coordinate system, map visualisation and map border
/// behavior.
#[derive(Debug, Deserialize, Resource, Serialize)]
pub enum WorldModel {
    Flat(FlatWorldModel),
    Globe(GlobeWorldModel),
}

impl WorldModel {
    pub fn str(&self) -> &'static str {
        match self {
            Self::Flat(_) => "Flat",
            Self::Globe(_) => "Globe",
        }
    }
}

impl Default for WorldModel {
    fn default() -> Self {
        WorldModel::Flat(FlatWorldModel::default())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlatWorldModel {
    pub world_size: [u32; 2],
}

impl Default for FlatWorldModel {
    fn default() -> Self {
        Self {
            world_size: [300, 200],
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GlobeWorldModel;

#[derive(Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct TopographyConfig {}

#[derive(Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct ClimateConfig {}
