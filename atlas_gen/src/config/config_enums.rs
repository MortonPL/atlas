use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use atlas_lib::{ui::sidebar::*, MakeUi, UiEditableEnum, UiEditableEnumWithFields};

use crate::config::{AdvancedGenerator, FbmConfig, SimpleGenerator};

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

/// Algorithm describes the noise algorithm that should be used to generate a layer,
/// as well as its paramateres.
#[derive(Clone, Copy, Debug, Deserialize, Resource, Serialize, UiEditableEnum)]
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

/// Influence map type describes what shape should be generated for the influence map.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, UiEditableEnumWithFields)]
pub enum InfluenceMapType {
    None(()),
    Circle(InfluenceCircleConfig),
    Strip(InfluenceStripConfig),
    Archipelago(InfluenceArchipelagoConfig),
    Fbm(InfluenceFbmConfig),
}

impl Default for InfluenceMapType {
    fn default() -> Self {
        Self::None(Default::default())
    }
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct InfluenceCircleConfig {
    #[name("Radius")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=500))]
    #[add(speed(10.0))]
    pub radius: u32,
    #[name("Offset")]
    #[control(SidebarSliderN)]
    pub offset: [u32; 2],
    #[name("Midpoint")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.1..=1.0))]
    #[add(speed(0.1))]
    pub midpoint: f32,
    #[name("Midpoint Value")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub midpoint_value: f32,
}

impl Default for InfluenceCircleConfig {
    fn default() -> Self {
        Self {
            radius: 100,
            offset: Default::default(),
            midpoint: 0.5,
            midpoint_value: 0.5,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct InfluenceStripConfig {}

impl Default for InfluenceStripConfig {
    fn default() -> Self {
        Self {  }
    }
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct InfluenceArchipelagoConfig {
    #[name("Seed")]
    #[control(SidebarSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
}

impl Default for InfluenceArchipelagoConfig {
    fn default() -> Self {
        Self {
            seed: 0,
        }
    }
}

#[derive(Clone, Default, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct InfluenceFbmConfig {
    #[name("Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub algorithm: SimpleAlgorithm,
    pub config: FbmConfig,
}
