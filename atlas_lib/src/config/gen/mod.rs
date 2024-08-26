mod defaults;

use crate::{
    bevy::prelude::*,
    bevy_egui,
    bevy_egui::egui::lerp,
    config::{
        climate::{
            BiomeConfig, ClimateConfig, ALTITUDE_MAX, ALTITUDE_MIN, CELSIUS_MAX, CELSIUS_MIN, PRECIP_MAX,
            PRECIP_MIN,
        },
        deposit::DepositsConfig,
        sim::AtlasSimConfig,
        AtlasConfig, ClimatePreviewMode, IntoSimConfig, WorldModel, MAX_WORLD_SIZE,
    },
    serde_derive::{Deserialize, Serialize},
    ui::{sidebar::*, UiEditableEnum},
    MakeUi, MakeUiEnum, UiEditableEnum,
};

pub const CONFIG_NAME: &str = "atlasgen.toml";

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct AtlasGenConfig {
    pub general: GeneralConfig,
    pub continents: ContinentsConfig,
    pub topography: TopographyConfig,
    pub temperature: TemperatureConfig,
    pub precipitation: PrecipitationConfig,
    pub climate: ClimateConfig,
    pub deposits: DepositsConfig,
}

impl AtlasConfig for AtlasGenConfig {
    fn get_world_size(&self) -> (u32, u32) {
        (self.general.world_size[0], self.general.world_size[1])
    }

    fn get_preview_model(&self) -> WorldModel {
        self.general.preview_model
    }

    fn get_climate_preview(&self) -> ClimatePreviewMode {
        self.climate.preview_mode
    }

    /// Get reference to a biome based on its index.
    fn get_biome(&self, i: u8) -> &BiomeConfig {
        let i = i as usize;
        if i > self.climate.biomes.len() {
            &self.climate.default_biome
        } else {
            &self.climate.biomes[i]
        }
    }
}

impl AtlasGenConfig {
    pub fn into_sim_config(&self) -> AtlasSimConfig {
        AtlasSimConfig {
            general: crate::config::sim::GeneralConfig {
                world_size: self.general.world_size.clone(),
            },
            scenario: Default::default(),
            climate: self.climate.into_sim_config(),
            rules: Default::default(),
            deposits: self.deposits.clone(),
        }
    }
}

/// Config for the general map settings.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct GeneralConfig {
    #[name("Altitude Limit for Preview [m]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(ALTITUDE_MIN..=ALTITUDE_MAX))]
    pub altitude_limit: f32,
    #[name("Preview Height Levels")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0..=100))]
    #[add(speed(0.1))]
    pub height_levels: u32,
    #[name("Preview Color Display")]
    #[control(SidebarEnumDropdown)]
    pub color_display: ColorDisplayMode,
    #[name("World Model Preview")]
    #[control(SidebarEnumDropdown)]
    pub preview_model: WorldModel,
    /* NOTE: Disabled. Doesn't work with the generator.
    #[name("World Model Generation")]
    #[control(SidebarEnumDropdown)]
    */
    #[serde(skip)]
    pub generation_model: WorldModel,
    #[name("World Size")]
    #[control(SidebarSliderN)]
    #[add(clamp_range(1..=MAX_WORLD_SIZE))]
    pub world_size: [u32; 2],
}

/// How map should be colored in the map preview.
#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum ColorDisplayMode {
    /// Use color palette depending on topography.
    #[default]
    Topography,
    /// Use climate colors (simplified).
    SimplifiedClimate,
    /// Use climate colors.
    DetailedClimate,
}

/// Config for the continents generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ContinentsConfig {
    #[name("Sea Level")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub sea_level: f32,
    #[name("Noise Algorithm")]
    #[control(SidebarEnumSection)]
    pub algorithm: NoiseAlgorithm,
    #[name("Influence Shape")]
    #[control(SidebarEnumSection)]
    pub influence_shape: InfluenceShape,
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
    #[add(clamp_range(0..=20))]
    #[add(speed(0.5))]
    pub coastal_erosion: u8,
    #[name("Noise Algorithm")]
    #[control(SidebarEnumSection)]
    pub algorithm: NoiseAlgorithm,
    #[name("Influence Shape")]
    #[control(SidebarEnumSection)]
    pub influence_shape: InfluenceShape,
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
    #[name("Moist Adiabatic Lapse Rate [C/km]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10.0))]
    #[add(speed(0.1))]
    pub lapse_rate: f32,
    #[name("Latitudinal Settings [C]")]
    #[control(SidebarStructSection)]
    pub latitudinal: LatitudinalTemperatureLerp,
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
    #[name("Altitude of Max Precipitation [m]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(ALTITUDE_MIN..=ALTITUDE_MAX))]
    pub amp_point: f32,
    #[name("Precipitation Drop [mm/m]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10.0))]
    #[add(speed(0.1))]
    pub drop_per_height: f32,
    #[name("Latitudinal Settings [mm]")]
    #[control(SidebarStructSection)]
    pub latitudinal: LatitudinalPrecipitationLerp,
    #[name("Noise Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub algorithm_strength: f32,
    #[name("Noise Algorithm")]
    #[control(SidebarEnumSection)]
    pub algorithm: NoiseAlgorithm,
    #[name("Influence Map Type")]
    #[control(SidebarEnumSection)]
    pub influence_shape: InfluenceShape,
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

/// Specialised multi-segment lerp operating on latitude coordinates.
/// HACK: Different type for temperature and precipitation, because clamp limits are different.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct LatitudinalTemperatureLerp {
    #[name("Value At North Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_pole_value: f32,
    #[name("Value At 69 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_arctic_value: f32,
    #[name("Value At 46 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_temperate_value: f32,
    #[name("Value At 23 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_tropic_value: f32,
    #[name("Value At Equator")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub equator_value: f32,
    #[name("Value At 23 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_tropic_value: f32,
    #[name("Value At 46 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_temperate_value: f32,
    #[name("Value At 69 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_arctic_value: f32,
    #[name("Value At South Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_pole_value: f32,
    #[name("Non-Linear Tropics Bias")]
    #[control(SidebarCheckbox)]
    pub non_linear_tropics: bool,
}

/// Specialised multi-segment lerp operating on latitude coordinates.
/// HACK: Different type for temperature and precipitation, because clamp limits are different.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct LatitudinalPrecipitationLerp {
    #[name("Value At North Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_pole_value: f32,
    #[name("Value At 69 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_arctic_value: f32,
    #[name("Value At 46 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_temperate_value: f32,
    #[name("Value At 23 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_tropic_value: f32,
    #[name("Value At Equator")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub equator_value: f32,
    #[name("Value At 23 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_tropic_value: f32,
    #[name("Value At 46 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_temperate_value: f32,
    #[name("Value At 69 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_arctic_value: f32,
    #[name("Value At South Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_pole_value: f32,
    #[name("Non-Linear Tropics Bias")]
    #[control(SidebarCheckbox)]
    pub non_linear_tropics: bool,
}

/// Fbm generic noise sampling parameters.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUi)]
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

/// Algorithm describes the noise algorithm that should be used to generate a layer,
/// as well as its paramateres.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUiEnum, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum NoiseAlgorithm {
    Perlin(FbmConfig),
    PerlinSurflet(FbmConfig),
    OpenSimplex(FbmConfig),
    SuperSimplex(FbmConfig),
    #[empty]
    FromImage,
}

/// Configuration for a three-segment lerper.
#[derive(Clone, Debug, Deserialize, Resource, Serialize, MakeUi)]
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

impl QuadPointLerp {
    /// Clone the struct and precalculate difference values.
    pub fn clone_precalc(&self) -> Self {
        Self {
            start: self.start,
            midpoint: self.midpoint,
            midpoint2: self.midpoint2,
            end: self.end,
            midpoint_position: self.midpoint_position,
            midpoint2_position: self.midpoint2_position,
            diff1: self.midpoint2_position - self.midpoint_position,
            diff2: 1.0 - self.midpoint2_position,
        }
    }

    /// Interpolate a value in [0.0, 1.0] range. NOTE: Self should have precalc'd diff1 and diff2 beforehand!
    pub fn lerp(&self, x: f32) -> f32 {
        if x <= self.midpoint_position {
            lerp(self.start..=self.midpoint, x / self.midpoint_position)
        } else if x <= self.midpoint2_position {
            lerp(
                self.midpoint..=self.midpoint2,
                (x - self.midpoint_position) / self.diff1,
            )
        } else {
            lerp(
                self.midpoint2..=self.end,
                (x - self.midpoint2_position) / self.diff2,
            )
        }
    }
}

/// What shape should be generated for the influence map.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUiEnum, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum InfluenceShape {
    #[empty]
    None,
    Circle(InfluenceCircleConfig),
    Strip(InfluenceStripConfig),
    Fbm(InfluenceFbmConfig),
    FromImage(InfluenceImageConfig),
}

/// How influence values should affect data values.
#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
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

/// A strip consisting of a fat line segment with two circles at the end.
/// Both length and thickness of the line are controllable, and the segement can be offset (from map center) and rotated.
/// Value falloff from the line segment is controlled by "midpoint" settings.
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

/// Data from fBm noise sampling.
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

impl AsRef<NoiseAlgorithm> for InfluenceFbmConfig {
    fn as_ref(&self) -> &NoiseAlgorithm {
        &self.algorithm
    }
}

/// Data from an external image.
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
