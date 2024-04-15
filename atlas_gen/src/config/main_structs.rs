use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

use atlas_lib::{ui::sidebar::*, MakeUi};

pub use crate::config::common_structs::*;

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct SessionConfig {
    pub general: GeneralConfig,
    pub continents: ContinentsConfig,
    pub topography: TopographyConfig,
    pub climate: ClimateConfig,
    pub resources: ResourcesConfig,
}

/// Config for the general map settings.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct GeneralConfig {
    #[name("World Seed")]
    #[control(SidebarSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
    #[name("Tile Resolution")]
    #[control(SidebarSlider)]
    #[add(clamp_range(10.0..=200.0))]
    pub tile_resolution: f32,
    #[name("Topography Display Mode")]
    #[control(SidebarEnumDropdown)]
    pub topo_display: TopographyDisplayMode,
    #[name("World Model")]
    #[control(SidebarEnumDropdown)]
    pub world_model: WorldModel,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            seed: rand::random(),
            tile_resolution: 100.0,
            world_model: Default::default(),
            topo_display: Default::default(),
        }
    }
}

/// Config for the continents generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ContinentsConfig {
    #[name("Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub algorithm: NoiseAlgorithm,
    #[name("Sea Level")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub sea_level: f32,
    #[name("Influence Map Type")]
    #[control(SidebarEnumDropdown)]
    pub influence_map_type: InfluenceShape,
    #[name("Influence Map Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub influence_map_strength: f32,
    pub config: FbmConfig,
}

impl Default for ContinentsConfig {
    fn default() -> Self {
        Self {
            algorithm: Default::default(),
            sea_level: 0.4,
            influence_map_type: Default::default(),
            influence_map_strength: 1.0,
            config: Default::default(),
        }
    }
}

/// Config for the topography (heightmap) generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct TopographyConfig {
    #[name("Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub algorithm: NoiseAlgorithm,
    #[name("Coastal Erosion Range")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=7))]
    #[add(speed(0.5))]
    pub coastal_erosion: u8,
    #[name("Influence Map Type")]
    #[control(SidebarEnumDropdown)]
    pub influence_map_type: InfluenceShape,
    #[name("Influence Map Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub influence_map_strength: f32,
    pub config: FbmConfig,
}

impl Default for TopographyConfig {
    fn default() -> Self {
        Self {
            algorithm: Default::default(),
            coastal_erosion: 1,
            influence_map_type: Default::default(),
            influence_map_strength: 1.0,
            config: Default::default(),
        }
    }
}

/// Config for the climate generation.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct ClimateConfig {}

/// Config for the resource generation.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct ResourcesConfig {}
