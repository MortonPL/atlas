use bevy::prelude::Resource;
use serde_derive::{Deserialize, Serialize};

use atlas_lib::{ui::sidebar::*, MakeUi};

pub use crate::config::common_structs::*;

/// Complete configuration for the map generator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct SessionConfig {
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
    #[name("Tile Resolution")]
    #[control(SidebarSlider)]
    #[add(clamp_range(10.0..=200.0))]
    pub tile_resolution: f32,
    #[name("Preview Topography Display")]
    #[control(SidebarEnumDropdown)]
    pub topo_display: TopographyDisplayMode,
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
            topo_display: Default::default(),
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
    #[name("Algorithm")]
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
    #[add(clamp_range(0..=7))]
    #[add(speed(0.5))]
    pub coastal_erosion: u8,
    #[name("Algorithm")]
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
                range: 0.2,
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
    #[name("Temperature Latitudinal Settings [C]")]
    #[control(SidebarStructSection)]
    pub latitudinal: LatitudinalTemperatureLerp,
    #[name("Moist Adiabatic Lapse Rate [C/km]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10.0))]
    #[add(speed(0.1))]
    pub lapse_rate: f32,
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
    #[name("Precipitation Latitudinal Settings [mm]")]
    #[control(SidebarStructSection)]
    pub latitudinal: LatitudinalPrecipitationLerp,
    #[name("Minimum Altitude for Precipitation Drop [m]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10200.0))]
    pub drop_off_point: f32,
    #[name("Precipitation Drop [mm/m]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10.0))]
    #[add(speed(0.1))]
    pub drop_per_height: f32,
    #[name("Noise Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub algorithm_strength: f32,
    #[name("Algorithm")]
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
            drop_off_point: 2000.0,
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
#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct ClimateConfig {
    pub climates: Vec<BiomeConfig>,
    #[serde(skip)]
    pub default_climate: BiomeConfig,
}

impl Default for ClimateConfig {
    fn default() -> Self {
        Self {
            default_climate: BiomeConfig {
                name: "Default Biome".to_string(),
                color: [255, 0, 255],
                simple_color: [255, 0, 255],
                productivity: 1.0,
            },
            climates: vec![
                BiomeConfig {
                    // 0
                    name: "Default Biome".to_string(),
                    color: [255, 0, 255],
                    simple_color: [255, 0, 255],
                    productivity: 1.0,
                },
                BiomeConfig {
                    // 1
                    name: "Polar Desert".to_string(),
                    color: [225, 245, 250],
                    simple_color: [225, 245, 250],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 2
                    name: "Scorched Desert".to_string(),
                    color: [255, 215, 0],
                    simple_color: [225, 220, 55],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 3
                    name: "Arctic Desert (Wet)".to_string(),
                    color: [145, 160, 160],
                    simple_color: [225, 245, 250],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 4
                    name: "Tundra (Wet)".to_string(),
                    color: [90, 195, 155],
                    simple_color: [140, 195, 175],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 5
                    name: "Boreal Forest (Wet)".to_string(),
                    color: [40, 140, 100],
                    simple_color: [90, 170, 140],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 6
                    name: "Temperate Rainforest (Wet)".to_string(),
                    color: [90, 230, 45],
                    simple_color: [90, 200, 75],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 7
                    name: "Tropical Rainforest (Wet)".to_string(),
                    color: [30, 255, 0],
                    simple_color: [25, 210, 0],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 8
                    name: "Arctic Desert".to_string(),
                    color: [170, 185, 190],
                    simple_color: [225, 245, 250],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 9
                    name: "Tundra".to_string(),
                    color: [140, 195, 175],
                    simple_color: [140, 195, 175],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 10
                    name: "Boreal Forest".to_string(),
                    color: [90, 170, 140],
                    simple_color: [90, 170, 140],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 11
                    name: "Cold Desert (Arid)".to_string(),
                    color: [160, 155, 140],
                    simple_color: [185, 175, 140],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 12
                    name: "Cold Desert".to_string(),
                    color: [185, 175, 140],
                    simple_color: [185, 175, 140],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 13
                    name: "Temperate Grassland".to_string(),
                    color: [180, 190, 130],
                    simple_color: [90, 200, 75],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 14
                    name: "Temperate Shrubland".to_string(),
                    color: [150, 190, 130],
                    simple_color: [90, 200, 75],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 15
                    name: "Temperate Woodland".to_string(),
                    color: [90, 200, 75],
                    simple_color: [90, 200, 75],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 16
                    name: "Temperate Forest".to_string(),
                    color: [50, 185, 65],
                    simple_color: [90, 200, 75],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 17
                    name: "Temperate Rainforest".to_string(),
                    color: [0, 180, 50],
                    simple_color: [90, 200, 75],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 18
                    name: "Hot Desert (Arid)".to_string(),
                    color: [220, 195, 80],
                    simple_color: [225, 220, 55],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 19
                    name: "Hot Desert".to_string(),
                    color: [225, 220, 55],
                    simple_color: [225, 220, 55],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 20
                    name: "Savanna".to_string(),
                    color: [180, 210, 45],
                    simple_color: [130, 210, 0],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 21
                    name: "Tropical Forest".to_string(),
                    color: [130, 210, 0],
                    simple_color: [130, 210, 0],
                    productivity: 0.1, // TODO find good value
                },
                BiomeConfig {
                    // 22
                    name: "Tropical Rainforest".to_string(),
                    color: [25, 210, 0],
                    simple_color: [25, 210, 0],
                    productivity: 0.1, // TODO find good value
                },
            ],
        }
    }
}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct BiomeConfig {
    pub name: String,
    pub color: [u8; 3],
    pub simple_color: [u8; 3],
    pub productivity: f32,
}

/// Config for the resource generation.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct ResourcesConfig {}
