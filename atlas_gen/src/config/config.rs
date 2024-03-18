use atlas_lib::{ui::UiControl, MakeUi, UiConfigurableEnum};
use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

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
    #[control(UiSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
    #[name("Tile Resolution")]
    #[control(UiSlider)]
    #[add(clamp_range(10.0..=200.0))]
    pub tile_resolution: f32,
    #[name("World Model")]
    #[control(UiEnumDropdown)]
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

/// World model describes the geometric model of the world which
/// impacts the coordinate system, map visualisation and map border
/// behavior.
#[derive(Debug, Deserialize, Resource, Serialize, UiConfigurableEnum)]
#[serde(rename_all = "lowercase")]
pub enum WorldModel {
    Flat(FlatWorldModel),
    Globe(GlobeWorldModel),
}

impl WorldModel {
    pub fn get_dimensions(&self) -> (u32, u32) {
        match self {
            Self::Flat(x) => (x.world_size[0], x.world_size[1]),
            Self::Globe(_) => (100, 100),
        }
    }
}

impl Default for WorldModel {
    fn default() -> Self {
        WorldModel::Flat(FlatWorldModel::default())
    }
}

#[derive(Debug, Deserialize, Serialize, MakeUi)]
pub struct FlatWorldModel {
    #[name("World Size")]
    #[control(UiSliderN)]
    #[add(clamp_range(100..=500))]
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

#[derive(Debug, Deserialize, Resource, Serialize, UiConfigurableEnum)]
#[serde(rename_all = "lowercase")]
pub enum GeneratorType {
    Simple(SimpleGenerator),
    Advanced(AdvancedGenerator),
}

impl Default for GeneratorType {
    fn default() -> Self {
        GeneratorType::Simple(SimpleGenerator::default())
    }
}

// ******************************************************** //
// **************** SIMPLE GENERATOR CONFIG *************** //
// ******************************************************** //

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SimpleGenerator {
    pub topology: SimpleTopographyConfig,
    pub climate: SimpleClimateConfig,
}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct SimpleClimateConfig {}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct SimpleTopographyConfig {}

// ******************************************************** //
// *************** ADVANCED GENERATOR CONFIG ************** //
// ******************************************************** //

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AdvancedGenerator {
    pub continents: ContinentsConfig,
    pub topography: TopographyConfig,
    pub climate: ClimateConfig,
}

/// Config for layout of continets and oceans.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ContinentsConfig {
    #[name("# of Continents")]
    #[control(UiSlider)]
    #[add(clamp_range(1..=10))]
    pub num_continents: u8,
    #[name("# of Oceans")]
    #[control(UiSlider)]
    #[add(clamp_range(1..=10))]
    pub num_oceans: u8,
    // Internal - no UI
    pub data: Vec<u8>,
}

impl Default for ContinentsConfig {
    fn default() -> Self {
        Self {
            num_continents: 2,
            num_oceans: 1,
            data: Default::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct TopographyConfig {}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct ClimateConfig {}
