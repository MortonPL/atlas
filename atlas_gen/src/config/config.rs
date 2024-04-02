use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

use atlas_lib::{ui::sidebar::*, MakeUi, UiEditableEnum, UiEditableEnumWithFields};

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
    #[name("Sea Level")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=255))]
    pub sea_level: u8,
    #[name("Force Island")]
    #[control(SidebarCheckbox)]
    pub force_island: bool,
    pub config: FbmConfig,
}

#[derive(Clone, Copy, Debug, Deserialize, Resource, Serialize, UiEditableEnum)] // TODO
pub enum SimpleAlgorithm {
    Perlin,
    OpenSimplex,
    SuperSimplex,
}

impl Default for SimpleAlgorithm {
    fn default() -> Self {
        Self::Perlin
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
