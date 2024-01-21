use atlas_lib::{
    ui::UiControl,
    MakeUi, UiConfigurableEnum,
};
use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct GeneratorConfig {
    pub general: GeneralConfig,
    pub continents: ContinentsConfig,
    pub topography: TopographyConfig,
    pub climate: ClimateConfig,
}

/// Config for the general map settings.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct GeneralConfig {
    #[name("World Model")]
    #[control(UiEnumDropdown)]
    pub world_model: WorldModel,
    #[name("Tile Resolution")]
    #[control(UiSlider)]
    #[add(clamp_range(10.0..=200.0))]
    pub tile_resolution: f32,
    #[name("World Seed")]
    #[control(UiSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            world_model: Default::default(),
            tile_resolution: 100.0,
            seed: rand::random(),
        }
    }
}

/// World model describes the geometric model of the world which
/// impacts the coordinate system, map visualisation and map border
/// behavior.
#[derive(Debug, Deserialize, Resource, Serialize, UiConfigurableEnum)]
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
