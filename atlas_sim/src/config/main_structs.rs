use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    config::WorldModel,
    serde_derive::{Deserialize, Serialize},
};

use crate::config::{make_default_biomes, BiomeConfig, ClimatePreviewMode};

/// Complete configuration for the history simulator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct AtlasSimConfig {
    pub general: GeneralConfig,
    pub climate: ClimateConfig,
}

/// Complete configuration for the history simulator.
#[derive(Debug, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct GeneralConfig {
    pub preview_model: WorldModel,
    pub tile_resolution: f32,
    pub world_size: [u32; 2],
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            tile_resolution: 10.0,
            world_size: [360, 180],
            preview_model: Default::default(),
        }
    }
}

/// Config for the climate rules.
#[derive(Debug, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct ClimateConfig {
    #[serde(skip)]
    pub preview_mode: ClimatePreviewMode,
    pub biomes: Vec<BiomeConfig>,
    #[serde(skip)]
    pub default_biome: BiomeConfig,
}

impl Default for ClimateConfig {
    fn default() -> Self {
        Self {
            preview_mode: ClimatePreviewMode::DetailedColor,
            default_biome: BiomeConfig {
                name: "Default Biome".to_string(),
                color: [255, 0, 255],
                simple_color: [255, 0, 255],
                habitability: 1.0,
                productivity: 1.0,
            },
            biomes: make_default_biomes(),
        }
    }
}
