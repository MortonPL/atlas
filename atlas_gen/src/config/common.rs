use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::Resource},
    bevy_egui,
    serde_derive::{Deserialize, Serialize},
    ui::{sidebar::*, UiEditableEnum},
    MakeUi, MakeUiEnum, UiEditableEnum,
};

pub const MAX_WORLD_SIZE: u32 = 1000;

/// Algorithm describes the noise algorithm that should be used to generate a layer,
/// as well as its paramateres.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUiEnum, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "atlas_lib::serde")]
pub enum NoiseAlgorithm {
    Perlin(FbmConfig),
    OpenSimplex(FbmConfig),
    SuperSimplex(FbmConfig),
    #[empty]
    FromImage,
}

impl Default for NoiseAlgorithm {
    fn default() -> Self {
        Self::Perlin(Default::default())
    }
}

/// Fbm generic noise sampling parameters.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct FbmConfig {
    #[name("Seed")]
    #[control(SidebarSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
    #[name("Detail (Octaves)")]
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
    #[name("Offset")]
    #[control(SidebarSliderN)]
    pub offset: [f32; 2],
    #[name("Quad Point Interpolation")]
    #[control(SidebarStructSection)]
    pub midpoint: QuadPointLerp,
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
            midpoint: QuadPointLerp {
                start: 0.0,
                midpoint: 0.3333,
                midpoint2: 0.6666,
                end: 1.0,
                midpoint_position: 0.3333,
                midpoint2_position: 0.6666,
                ..Default::default()
            },
            offset: Default::default(),
        }
    }
}

/// Configuration for a three-segment lerper.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct QuadPointLerp {
    #[name("Start Value")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub start: f32,
    #[name("Point 2 Value")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub midpoint: f32,
    #[name("Point 3 Value")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub midpoint2: f32,
    #[name("End Value")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub end: f32,
    #[name("Point 2 Position")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=0.99))]
    #[add(speed(0.1))]
    pub midpoint_position: f32,
    #[name("Point 3 Position")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=0.99))]
    #[add(speed(0.1))]
    pub midpoint2_position: f32,
    #[serde(skip)]
    pub diff1: f32,
    #[serde(skip)]
    pub diff2: f32,
}

impl Default for QuadPointLerp {
    fn default() -> Self {
        Self {
            start: 1.0,
            midpoint: 0.6666,
            midpoint2: 0.3333,
            end: 0.0,
            midpoint_position: 0.3333,
            midpoint2_position: 0.6666,
            diff1: 0.0,
            diff2: 0.0,
        }
    }
}

/// What shape should be generated for the influence map.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUiEnum, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "atlas_lib::serde")]
pub enum InfluenceShape {
    #[empty]
    None,
    Circle(InfluenceCircleConfig),
    Strip(InfluenceStripConfig),
    Fbm(InfluenceFbmConfig),
    FromImage(InfluenceImageConfig),
}

impl Default for InfluenceShape {
    fn default() -> Self {
        Self::None
    }
}

/// How influence values should affect data values.
#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "atlas_lib::serde")]
pub enum InfluenceMode {
    /// Influence < 1 will scale data down.
    #[default]
    ScaleDown,
    /// Influence > 0 will scale data up.
    ScaleUp,
    /// Influence > 0.5 will scale data up, influence < 0.5 will scale down.
    ScaleUpDown,
}

/// A circle defined by offset (from center) and radius. Value falloff
/// from the center of the circle is controlled by "midpoint" settings.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
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
    #[add(clamp_range(1..=MAX_WORLD_SIZE))]
    #[add(speed(10.0))]
    pub radius: u32,
    #[name("Offset")]
    #[control(SidebarSliderN)]
    pub offset: [i32; 2],
    #[name("Quad Point Interpolation")]
    #[control(SidebarStructSection)]
    pub midpoint: QuadPointLerp,
}

impl Default for InfluenceCircleConfig {
    fn default() -> Self {
        Self {
            influence_mode: Default::default(),
            influence_strength: 1.0,
            radius: 100,
            offset: Default::default(),
            midpoint: Default::default(),
        }
    }
}

/// A strip consisting of a fat line segment with two circles at the end.
/// Both length and thickness of the line are controllable, and the segement can be offset (from map center) and rotated.
/// Value falloff from the line segment is controlled by "midpoint" settings.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
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
    #[add(clamp_range(1..=MAX_WORLD_SIZE))]
    #[add(speed(10.0))]
    pub thickness: u32,
    #[name("Length")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=MAX_WORLD_SIZE))]
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
    #[name("Quad Point Interpolation")]
    #[control(SidebarStructSection)]
    pub midpoint: QuadPointLerp,
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
            midpoint: Default::default(),
        }
    }
}

/// Data from fBm noise sampling.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
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

/// Data from an external image.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
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
