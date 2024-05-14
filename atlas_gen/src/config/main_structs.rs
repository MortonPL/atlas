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
    pub temperature: TemperatureConfig,
    pub precipitation: PrecipitationConfig,
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
    #[name("Preview Height Display")]
    #[control(SidebarEnumDropdown)]
    pub topo_display: TopographyDisplayMode,
    #[name("Preview Height Levels")]
    #[control(SidebarSlider)]
    #[add(clamp_range(3..=100))]
    #[add(speed(0.1))]
    pub height_levels: u32,
    #[name("World Model")]
    #[control(SidebarEnumSection)]
    pub world_model: WorldModel,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            seed: rand::random(),
            tile_resolution: 100.0,
            topo_display: Default::default(),
            height_levels: 10,
            world_model: Default::default(),
        }
    }
}

/// Config for the continents generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ContinentsConfig {
    #[name("Sea Level")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub sea_level: f32,
    #[name("Algorithm")]
    #[control(SidebarEnumSection)]
    pub algorithm: NoiseAlgorithm,
    #[name("Influence Shape")]
    #[control(SidebarEnumSection)]
    pub influence_shape: InfluenceShape,
}

impl Default for ContinentsConfig {
    fn default() -> Self {
        Self {
            sea_level: 0.4,
            algorithm: Default::default(),
            influence_shape: Default::default(),
        }
    }
}

impl AsRef<InfluenceShape> for ContinentsConfig {
    fn as_ref(&self) -> &InfluenceShape {
        &self.influence_shape
    }
}

impl AsRef<NoiseAlgorithm> for ContinentsConfig {
    fn as_ref(&self) -> &NoiseAlgorithm {
        &self.algorithm
    }
}

/// Config for the topography (heightmap) generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct TopographyConfig {
    #[name("Coastal Erosion Range")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=7))]
    #[add(speed(0.5))]
    pub coastal_erosion: u8,
    #[name("Algorithm")]
    #[control(SidebarEnumSection)]
    pub algorithm: NoiseAlgorithm,
    #[name("Influence Shape")]
    #[control(SidebarEnumSection)]
    pub influence_shape: InfluenceShape,
}

impl Default for TopographyConfig {
    fn default() -> Self {
        Self {
            coastal_erosion: 1,
            algorithm: Default::default(),
            influence_shape: Default::default(),
        }
    }
}

impl AsRef<InfluenceShape> for TopographyConfig {
    fn as_ref(&self) -> &InfluenceShape {
        &self.influence_shape
    }
}

impl AsRef<NoiseAlgorithm> for TopographyConfig {
    fn as_ref(&self) -> &NoiseAlgorithm {
        &self.algorithm
    }
}

/// Config for the temperature generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct TemperatureConfig {
    #[name("Temperature (C+100) At South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub south_value: u8,
    #[name("Temperature (C+100) At Equator")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub equator_value: u8,
    #[name("Temperature (C+100) At North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub north_value: u8,
    #[name("Temperature (C+100) Drop Per Height Unit")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10.0))]
    #[add(speed(0.1))]
    pub drop_per_height: f32,
    #[name("Noise Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub algorithm_strength: f32,
    #[name("Noise Algorithm")]
    #[control(SidebarEnumSection)]
    pub algorithm: NoiseAlgorithm,
    #[name("Influence Shape")]
    #[control(SidebarEnumSection)]
    pub influence_shape: InfluenceShape,
}

impl Default for TemperatureConfig {
    fn default() -> Self {
        Self {
            south_value: 60,
            equator_value: 130,
            north_value: 80,
            drop_per_height: 0.1,
            algorithm_strength: 0.1,
            algorithm: Default::default(),
            influence_shape: Default::default(),
        }
    }
}

impl AsRef<InfluenceShape> for TemperatureConfig {
    fn as_ref(&self) -> &InfluenceShape {
        &self.influence_shape
    }
}

impl AsRef<NoiseAlgorithm> for TemperatureConfig {
    fn as_ref(&self) -> &NoiseAlgorithm {
        &self.algorithm
    }
}

/// Config for the precipitation generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct PrecipitationConfig {
    #[name("Precipitation (x40mm) At South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub south_value: u8,
    #[name("Precipitation (x40mm) At 46 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub south_temperate_value: u8,
    #[name("Precipitation (x40mm) At 23 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub south_tropic_value: u8,
    #[name("Precipitation (x40mm) At Equator")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub equator_value: u8,
    #[name("Precipitation (x40mm) At 23 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub north_tropic_value: u8,
    #[name("Precipitation (x40mm) At 46 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub north_temperate_value: u8,
    #[name("Precipitation (x40mm) At North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub north_value: u8,
    #[name("Precipitation (x40mm) Drop Per Height Unit")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10.0))]
    #[add(speed(0.1))]
    pub drop_per_height: f32,
    #[name("Noise Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub algorithm_strength: f32,
    #[name("Algorithm")]
    #[control(SidebarEnumSection)]
    pub algorithm: NoiseAlgorithm,
    #[name("Influence Map Type")]
    #[control(SidebarEnumSection)]
    pub influence_shape: InfluenceShape,
}

impl Default for PrecipitationConfig {
    fn default() -> Self {
        Self {
            south_value: 128,
            south_temperate_value: 200,
            south_tropic_value: 25,
            equator_value: 230,
            north_tropic_value: 25,
            north_temperate_value: 200,
            north_value: 128,
            drop_per_height: 0.1,
            algorithm_strength: 0.1,
            algorithm: Default::default(),
            influence_shape: Default::default(),
        }
    }
}

impl AsRef<InfluenceShape> for PrecipitationConfig {
    fn as_ref(&self) -> &InfluenceShape {
        &self.influence_shape
    }
}

impl AsRef<NoiseAlgorithm> for PrecipitationConfig {
    fn as_ref(&self) -> &NoiseAlgorithm {
        &self.algorithm
    }
}

/// Config for the climate generation.
#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct ClimateConfig {
    pub climates: Vec<SingleClimateConfig>,
}

impl Default for ClimateConfig {
    fn default() -> Self {
        Self {
            climates: vec![SingleClimateConfig {
                name: "Subtropical Desert".to_string(),
                temperature: [103, 255],
                precipitation: [0, 64],
                color: [255, 200, 0],
            }],
        }
    }
}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct SingleClimateConfig {
    pub name: String,
    pub temperature: [u8; 2],
    pub precipitation: [u8; 2],
    pub color: [u8; 3],
}

/// Config for the resource generation.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct ResourcesConfig {}
