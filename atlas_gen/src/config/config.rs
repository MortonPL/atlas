use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

use atlas_lib::{ui::sidebar::*, MakeUi, UiEditableEnumWithFields};

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

/// World model describes the geometric model of the world which
/// impacts the coordinate system, map visualisation and map border
/// behavior.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, UiEditableEnumWithFields)]
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

#[derive(Clone, Debug, Deserialize, Serialize, MakeUi)]
pub struct FlatWorldModel {
    #[name("World Size")]
    #[control(SidebarSliderN)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GlobeWorldModel;

#[derive(Debug, Deserialize, Resource, Serialize, UiEditableEnumWithFields)]
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
    pub topography: SimpleTopographyConfig,
    pub climate: SimpleClimateConfig,
    pub resources: SimpleResourcesConfig,
}

#[derive(Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
pub struct SimpleTopographyConfig {
    #[name("Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub algorithm: SimpleAlgorithm,
    #[name("Ocean Level")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub ocean_level: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Resource, Serialize, UiEditableEnumWithFields)] // TODO
pub enum SimpleAlgorithm {
    Perlin(PerlinConfig),
    PerlinFractal(u8),
    Simplex(SimplexConfig),
    SimplexFractal(u8),
    DiamondSquare(u8),
}

impl Default for SimpleAlgorithm {
    fn default() -> Self {
        Self::Perlin(Default::default())
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
pub struct PerlinConfig {
    #[name("Seed")]
    #[control(SidebarSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
    #[name("Scale")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.01..=0.99))]
    #[add(speed(0.1))]
    pub scale: f64,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
pub struct SimplexConfig {
    #[name("Seed")]
    #[control(SidebarSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
    #[name("Scale")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.01..=0.99))]
    #[add(speed(0.1))]
    pub scale: f64,
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
