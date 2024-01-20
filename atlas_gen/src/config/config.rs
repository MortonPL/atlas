use atlas_lib::MakeUi;
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
#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct GeneralConfig {
    pub world_model: WorldModel,
    pub tile_resolution: f32,
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

/// Config for layout of continets and oceans.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ContinentsConfig {
    #[name("# of Continents")]
    #[control(UiSlider)]
    #[add(clamp_range(0..=10))]
    #[hint("Balbinka")]
    pub num_continents: u8,
    #[name("# of Oceans")]
    #[control(UiSlider)]
    #[add(clamp_range(0..=10))]
    pub num_oceans: u8,
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
