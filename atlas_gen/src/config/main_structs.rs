use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui,
    serde_derive::{Deserialize, Serialize},
    ui::sidebar::*,
    ui::UiEditableEnum,
    MakeUi, UiEditableEnum,
};

pub use crate::config::common_structs::*;

use crate::config::{climate_structs::make_default_biomes, ALTITUDE_MAX, ALTITUDE_MIN};

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct AtlasGenConfig {
    pub general: GeneralConfig,
    pub continents: ContinentsConfig,
    pub topography: TopographyConfig,
    pub temperature: TemperatureConfig,
    pub precipitation: PrecipitationConfig,
    pub climate: ClimateConfig,
    pub resources: ResourcesConfig,
}

/// Config for the general map settings.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
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
    #[name("World Model")]
    #[control(SidebarEnumSection)]
    pub world_model: WorldModel,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            altitude_limit: 2600.0,
            color_display: Default::default(),
            height_levels: 10,
            world_model: Default::default(),
        }
    }
}

/// Config for the continents generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
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

impl Default for ContinentsConfig {
    fn default() -> Self {
        Self {
            sea_level: 0.4,
            algorithm: Default::default(),
            influence_shape: Default::default(),
        }
    }
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
#[serde(crate = "atlas_lib::serde")]
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

impl Default for TopographyConfig {
    fn default() -> Self {
        Self {
            coastal_erosion: 1,
            algorithm: NoiseAlgorithm::Perlin(FbmConfig {
                midpoint: QuadPointLerp {
                    start: 0.0,
                    midpoint: 0.1,
                    midpoint2: 0.2,
                    end: 0.4,
                    midpoint_position: 0.70,
                    midpoint2_position: 0.95,
                    ..Default::default()
                },
                ..Default::default()
            }),
            influence_shape: Default::default(),
        }
    }
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
#[serde(crate = "atlas_lib::serde")]
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

impl Default for TemperatureConfig {
    fn default() -> Self {
        Self {
            latitudinal: LatitudinalTemperatureLerp {
                north_pole_value: -50.0,
                north_arctic_value: -15.0,
                north_temperate_value: 11.0,
                north_tropic_value: 23.0,
                equator_value: 30.0,
                south_tropic_value: 23.0,
                south_temperate_value: 11.0,
                south_arctic_value: -15.0,
                south_pole_value: -50.0,
                non_linear_tropics: false,
            },
            lapse_rate: 5.0,
            algorithm_strength: 0.1,
            algorithm: Default::default(),
            influence_shape: Default::default(),
        }
    }
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
#[serde(crate = "atlas_lib::serde")]
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

impl Default for PrecipitationConfig {
    fn default() -> Self {
        Self {
            latitudinal: LatitudinalPrecipitationLerp {
                south_pole_value: 0.0,
                south_arctic_value: 300.0,
                south_temperate_value: 1800.0,
                south_tropic_value: 100.0,
                equator_value: 4000.0,
                north_tropic_value: 100.0,
                north_temperate_value: 1800.0,
                north_arctic_value: 300.0,
                north_pole_value: 0.0,
                non_linear_tropics: false,
            },
            amp_point: 2000.0,
            drop_per_height: 1.5,
            algorithm_strength: 0.1,
            algorithm: Default::default(),
            influence_shape: Default::default(),
        }
    }
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

/// Config for the climate generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct ClimateConfig {
    #[serde(skip)]
    #[name("Preview Mode")]
    #[control(SidebarEnumDropdown)]
    pub preview_mode: ClimatePreviewMode,
    #[name("")]
    #[control(SidebarStructList)]
    pub biomes: Vec<BiomeConfig>,
    #[serde(skip)]
    pub default_biome: BiomeConfig,
}

impl Default for ClimateConfig {
    fn default() -> Self {
        Self {
            preview_mode: ClimatePreviewMode::DetailedColor,
            default_biome: BiomeConfig {
                name: "Default Biome".to_string(),
                color: [255, 0, 255],
                simple_color: [255, 0, 255],
                productivity: 1.0,
                habitability: 1.0,
            },
            biomes: make_default_biomes(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "atlas_lib::serde")]
pub enum ClimatePreviewMode {
    SimplifiedColor,
    #[default]
    DetailedColor,
    Habitability,
}

/// A single climate biome.
#[derive(Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct BiomeConfig {
    #[name("Name")]
    #[control(SidebarHeader)]
    pub name: String,
    #[name("Color")]
    #[control(SidebarColor)]
    pub color: [u8; 3],
    #[name("Color (Simplified View)")]
    #[control(SidebarColor)]
    pub simple_color: [u8; 3],
    #[name("Habitability")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub habitability: f32,
    #[name("Productivity")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub productivity: f32, // TODO: replace with per-job-yield (farming/plantation-farming/hunting-gathering/herding/woodcutting).
}

/// Config for the resource generation.
#[derive(Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct ResourcesConfig {}
