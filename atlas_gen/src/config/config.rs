use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct GeneratorConfig {
    pub general: GeneralConfig,
    pub topography: TopographyConfig,
    pub climate: ClimateConfig,
}

/// Config for the general map settings.
#[derive(Debug, Deserialize, PartialEq, Resource, Serialize)]
pub struct GeneralConfig {
    pub world_model: WorldModel,
    pub world_size: [u32; 2],
    pub tile_resolution: f32,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            world_model: Default::default(),
            world_size: [300, 200],
            tile_resolution: 100.0,
        }
    }
}

/// World model describes the geometric model of the world which
/// impacts the coordinate system, map visualisation and map border
/// behavior.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub enum WorldModel {
    #[default]
    Flat,
    Globe,
}

impl WorldModel {
    pub fn str(self) -> &'static str {
        match self {
            Self::Flat => "Flat",
            Self::Globe => "Globe",
        }
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct TopographyConfig {}

#[derive(Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct ClimateConfig {}
