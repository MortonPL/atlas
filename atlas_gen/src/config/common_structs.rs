use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use atlas_lib::{ui::sidebar::*, MakeUi, MakeUiEnum, UiEditableEnum, UiEditableEnumWithFields};

use crate::config::{CELSIUS_MAX, CELSIUS_MIN, PRECIP_MAX, PRECIP_MIN};

/// World model describes the geometric model of the world which
/// impacts the coordinate system, map visualisation and map border
/// behavior.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUiEnum, UiEditableEnumWithFields)]
#[serde(rename_all = "lowercase")]
pub enum WorldModel {
    Flat(FlatWorldModel),
    #[empty]
    Globe(()),
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

#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum TopographyDisplayMode {
    #[default]
    Nothing,
    Absolute128,
    Absolute255,
    Highest,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum ColorDisplayMode {
    #[default]
    Topography,
    SimplifiedClimate,
    DetailedClimate,
}

/// Algorithm describes the noise algorithm that should be used to generate a layer,
/// as well as its paramateres.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUiEnum, UiEditableEnumWithFields)]
#[serde(rename_all = "lowercase")]
pub enum NoiseAlgorithm {
    Perlin(FbmConfig),
    OpenSimplex(FbmConfig),
    SuperSimplex(FbmConfig),
    #[empty]
    FromImage(()),
}

impl Default for NoiseAlgorithm {
    fn default() -> Self {
        Self::Perlin(Default::default())
    }
}

#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct FbmConfig {
    #[name("Seed")]
    #[control(SidebarSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
    #[name("Detail")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=12))]
    #[add(speed(0.5))]
    pub detail: u8,
    #[name("Scale (Frequency)")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.1..=10.0))]
    #[add(speed(0.1))]
    pub frequency: f32,
    #[name("Neatness (Lacunarity)")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1.0..=10.0))]
    #[add(speed(0.1))]
    pub neatness: f32,
    #[name("Roughness (Persistance)")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub roughness: f32,
    #[name("Bias")]
    #[control(SidebarSlider)]
    #[add(clamp_range(-1.0..=1.0))]
    #[add(speed(0.1))]
    pub bias: f32,
    #[name("Second Bias")]
    #[control(SidebarSlider)]
    #[add(clamp_range(-1.0..=1.0))]
    #[add(speed(0.1))]
    pub bias2: f32,
    #[name("Range")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.1..=10.0))]
    #[add(speed(0.1))]
    pub range: f32,
    #[name("Offset")]
    #[control(SidebarSliderN)]
    pub offset: [f32; 2],
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
            bias2: 0.0,
            range: 1.0,
            offset: Default::default(),
        }
    }
}

/// Influence map type describes what shape should be generated for the influence map.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUiEnum, UiEditableEnumWithFields)]
#[serde(rename_all = "lowercase")]
pub enum InfluenceShape {
    #[empty]
    None(()),
    Circle(InfluenceCircleConfig),
    Strip(InfluenceStripConfig),
    Fbm(InfluenceFbmConfig),
    FromImage(InfluenceImageConfig),
}

impl Default for InfluenceShape {
    fn default() -> Self {
        Self::None(())
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
pub enum InfluenceMode {
    #[default]
    ScaleDown,
    ScaleUp,
    ScaleUpDown,
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct InfluenceCircleConfig {
    #[name("Influence Mode")]
    #[control(SidebarEnumDropdown)]
    pub influence_mode: InfluenceMode,
    #[name("Influence Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub influence_strength: f32,
    #[name("Radius")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=500))]
    #[add(speed(10.0))]
    pub radius: u32,
    #[name("Offset")]
    #[control(SidebarSliderN)]
    pub offset: [i32; 2],
    #[name("Midpoint")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.01..=0.99))]
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
            influence_mode: Default::default(),
            influence_strength: 1.0,
            radius: 100,
            offset: Default::default(),
            midpoint: 0.5,
            midpoint_value: 0.5,
        }
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct InfluenceStripConfig {
    #[name("Influence Mode")]
    #[control(SidebarEnumDropdown)]
    pub influence_mode: InfluenceMode,
    #[name("Influence Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub influence_strength: f32,
    #[name("Thickness")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=500))]
    #[add(speed(10.0))]
    pub thickness: u32,
    #[name("Length")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=500))]
    #[add(speed(10.0))]
    pub length: u32,
    #[name("Angle")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=89))]
    #[add(speed(10.0))]
    pub angle: i32,
    #[name("Flip Horizontally")]
    #[control(SidebarCheckbox)]
    pub flip: bool,
    #[name("Offset")]
    #[control(SidebarSliderN)]
    pub offset: [i32; 2],
    #[name("Midpoint")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.01..=0.99))]
    #[add(speed(0.1))]
    pub midpoint: f32,
    #[name("Midpoint Value")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub midpoint_value: f32,
}

impl Default for InfluenceStripConfig {
    fn default() -> Self {
        Self {
            influence_mode: Default::default(),
            influence_strength: 1.0,
            thickness: 100,
            length: 100,
            angle: 0,
            flip: false,
            offset: Default::default(),
            midpoint: 0.5,
            midpoint_value: 0.5,
        }
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct InfluenceFbmConfig {
    #[name("Influence Mode")]
    #[control(SidebarEnumDropdown)]
    pub influence_mode: InfluenceMode,
    #[name("Influence Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub influence_strength: f32,
    #[name("Algorithm")]
    #[control(SidebarEnumSection)]
    pub algorithm: NoiseAlgorithm,
}

impl Default for InfluenceFbmConfig {
    fn default() -> Self {
        Self {
            influence_mode: Default::default(),
            influence_strength: 1.0,
            algorithm: Default::default(),
        }
    }
}

impl AsRef<NoiseAlgorithm> for InfluenceFbmConfig {
    fn as_ref(&self) -> &NoiseAlgorithm {
        &self.algorithm
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct InfluenceImageConfig {
    #[name("Influence Mode")]
    #[control(SidebarEnumDropdown)]
    pub influence_mode: InfluenceMode,
    #[name("Influence Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub influence_strength: f32,
}

impl Default for InfluenceImageConfig {
    fn default() -> Self {
        Self {
            influence_mode: Default::default(),
            influence_strength: 1.0,
        }
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct LatitudinalTemperatureLerp {
    #[name("Value At South Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_pole_value: f32,
    #[name("Value At 69 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_arctic_value: f32,
    #[name("Value At 46 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_temperate_value: f32,
    #[name("Value At 23 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_tropic_value: f32,
    #[name("Value At Equator")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub equator_value: f32,
    #[name("Value At 23 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_tropic_value: f32,
    #[name("Value At 46 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_temperate_value: f32,
    #[name("Value At 69 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_arctic_value: f32,
    #[name("Value At North Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_pole_value: f32,
    #[name("Non-Linear Tropics Bias")]
    #[control(SidebarCheckbox)]
    pub non_linear_tropics: bool,
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct LatitudinalPrecipitationLerp {
    #[name("Value At South Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_pole_value: f32,
    #[name("Value At 69 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_arctic_value: f32,
    #[name("Value At 46 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_temperate_value: f32,
    #[name("Value At 23 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_tropic_value: f32,
    #[name("Value At Equator")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub equator_value: f32,
    #[name("Value At 23 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_tropic_value: f32,
    #[name("Value At 46 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_temperate_value: f32,
    #[name("Value At 69 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_arctic_value: f32,
    #[name("Value At North Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_pole_value: f32,
    #[name("Non-Linear Tropics Bias")]
    #[control(SidebarCheckbox)]
    pub non_linear_tropics: bool,
}
