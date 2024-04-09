use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

use atlas_lib::{ui::sidebar::*, MakeUi};

pub use crate::config::config_enums::*;

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct GeneratorConfig {
    pub general: GeneralConfig,
    pub generator: GeneratorType,
}

// ******************************************************** //
// ******************** GENERAL CONFIG ******************** //
// ******************************************************** //

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
        }
    }
}

// ******************************************************** //
// **************** SIMPLE GENERATOR CONFIG *************** //
// ******************************************************** //

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SimpleGenerator {
    pub continents: SimpleContinentsConfig,
    pub topography: SimpleTopographyConfig,
    pub climate: SimpleClimateConfig,
    pub resources: SimpleResourcesConfig,
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct SimpleContinentsConfig {
    #[name("Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub algorithm: SimpleAlgorithm,
    #[name("Sea Level")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub sea_level: f32,
    #[name("Influence Map Type")]
    #[control(SidebarEnumDropdown)]
    pub influence_map_type: InfluenceMapType,
    #[name("Influence Map Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub influence_map_strength: f32,
    pub config: FbmConfig,
}

impl Default for SimpleContinentsConfig {
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

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct SimpleTopographyConfig {
    #[name("Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub algorithm: SimpleAlgorithm,
    #[name("Coastal Erosion Range")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=7))]
    #[add(speed(0.5))]
    pub coastal_erosion: u8,
    #[name("Influence Map Type")]
    #[control(SidebarEnumDropdown)]
    pub influence_map_type: InfluenceMapType,
    #[name("Influence Map Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub influence_map_strength: f32,
    pub config: FbmConfig,
}

impl Default for SimpleTopographyConfig {
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

#[derive(Clone, Copy, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct FbmConfig {
    #[name("Seed")]
    #[control(SidebarSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
    #[name("Detail")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=12))]
    #[add(speed(0.5))]
    pub detail: usize,
    #[name("Scale (Frequency)")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.1..=10.0))]
    #[add(speed(0.1))]
    pub frequency: f64,
    #[name("Neatness (Lacunarity)")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1.0..=10.0))]
    #[add(speed(0.1))]
    pub neatness: f64,
    #[name("Roughness (Persistance)")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub roughness: f64,
    #[name("Bias")]
    #[control(SidebarSlider)]
    #[add(clamp_range(-1.0..=1.0))]
    #[add(speed(10.0))]
    pub bias: f64,
    #[name("Range")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.1..=10.0))]
    #[add(speed(0.1))]
    pub range: f64,
    #[name("Offset")]
    #[control(SidebarSliderN)]
    pub offset: [f64; 2],
}

impl Default for FbmConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            detail: 6,
            frequency: 3.0,
            neatness: 2.0,
            roughness: 0.5,
            bias: 0.0,
            range: 1.0,
            offset: Default::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct SimpleClimateConfig {}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct SimpleResourcesConfig {}

// ******************************************************** //
// *************** ADVANCED GENERATOR CONFIG ************** //
// ******************************************************** //

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AdvancedGenerator {
    pub continents: ContinentsConfig,
    pub topography: TopographyConfig,
    pub climate: ClimateConfig,
    pub resources: ResourcesConfig,
}

/// Config for layout of continets and oceans.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ContinentsConfig {
    #[name("# of Continents")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=10))]
    pub num_continents: u8,
    #[name("# of Oceans")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=10))]
    pub num_oceans: u8,
}

impl Default for ContinentsConfig {
    fn default() -> Self {
        Self {
            num_continents: 2,
            num_oceans: 1,
        }
    }
}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct TopographyConfig {}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct ClimateConfig {}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct ResourcesConfig {}
