use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

use atlas_lib::{ui::sidebar::*, MakeUi};

pub use crate::config::common_structs::*;

use crate::config::{climate_structs::make_default_biomes, ALTITUDE_MAX, ALTITUDE_MIN};

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
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
pub struct GeneralConfig {
    #[name("World Seed")]
    #[control(SidebarSliderRandom)]
    #[add(speed(100.0))]
    pub seed: u32,
    #[name("Tile Resolution [km]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(10.0..=200.0))]
    pub tile_resolution: f32,
    #[name("Altitude Limit for Preview [m]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(ALTITUDE_MIN..=ALTITUDE_MAX))]
    pub topo_display: f32,
    #[name("Preview Color Display")]
    #[control(SidebarEnumDropdown)]
    pub color_display: ColorDisplayMode,
    #[name("Preview Height Levels")]
    #[control(SidebarSlider)]
    #[add(clamp_range(3..=100))]
    #[add(speed(0.1))]
    pub height_levels: u32,
    #[name("World Model")]
    #[control(SidebarEnumSection)]
    pub world_model: WorldModel,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            seed: rand::random(),
            tile_resolution: 100.0,
            topo_display: 2600.0,
            color_display: Default::default(),
            height_levels: 10,
            world_model: Default::default(),
        }
    }
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
                north_arctic_value: 0.0,
                north_temperate_value: 12.0,
                north_tropic_value: 22.0,
                equator_value: 30.0,
                south_tropic_value: 22.0,
                south_temperate_value: 12.0,
                south_arctic_value: 0.0,
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
                non_linear_tropics: true,
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
pub struct ClimateConfig {
    pub biomes: Vec<BiomeConfig>,
    #[serde(skip)]
    pub default_biome: BiomeConfig,
}

impl Default for ClimateConfig {
    fn default() -> Self {
        Self {
            default_biome: BiomeConfig {
                name: "Default Biome".to_string(),
                color: [255, 0, 255],
                simple_color: [255, 0, 255],
                productivity: 1.0,
            },
            biomes: make_default_biomes(),
        }
    }
}

/// A single climate biome.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct BiomeConfig {
    pub name: String,
    pub color: [u8; 3],
    pub simple_color: [u8; 3],
    pub productivity: f32, // TODO: replace with per-job-yield (farming/plantation-farming/hunting-gathering/herding/woodcutting).
}

/// Config for the resource generation.
#[derive(Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
pub struct ResourcesConfig {}
