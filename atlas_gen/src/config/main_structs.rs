use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui,
    config::{AtlasConfig, ClimatePreviewMode, WorldModel, MAX_WORLD_SIZE},
    serde_derive::{Deserialize, Serialize},
    ui::{sidebar::*, UiEditableEnum},
    MakeUi, UiEditableEnum,
};

use crate::config::{
    climate::{make_default_biomes, BiomeConfig},
    resource::{ResourceChunk, ResourceType, make_default_resources},
    FbmConfig, InfluenceShape, LatitudinalPrecipitationLerp, LatitudinalTemperatureLerp, NoiseAlgorithm,
    QuadPointLerp, ALTITUDE_MAX, ALTITUDE_MIN,
};

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

    fn climate_index_to_color(&self, i: u8) -> [u8; 4] {
        let biome = self.get_biome(i);
        [biome.color[0], biome.color[1], biome.color[2], 255]
    }

    fn climate_index_to_color_simple(&self, i: u8) -> [u8; 4] {
        let biome = self.get_biome(i);
        [
            biome.simple_color[0],
            biome.simple_color[1],
            biome.simple_color[2],
            255,
        ]
    }
}

impl AtlasGenConfig {
    /// Get reference to a biome based on its index.
    pub fn get_biome(&self, i: u8) -> &BiomeConfig {
        let i = i as usize;
        if i > self.climate.biomes.len() {
            &self.climate.default_biome
        } else {
            &self.climate.biomes[i]
        }
    }
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
    #[add(clamp_range(100..=MAX_WORLD_SIZE))]
    pub world_size: [u32; 2],
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            altitude_limit: 2600.0,
            color_display: Default::default(),
            height_levels: 10,
            preview_model: Default::default(),
            generation_model: Default::default(),
            world_size: [360, 180],
        }
    }
}

/// How map should be colored in the map preview.
#[derive(Clone, Copy, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "atlas_lib::serde")]
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
            latitudinal: Default::default(),
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
            latitudinal: Default::default(),
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
    #[name("Mountains Biome Index")]
    #[control(SidebarSlider)]
    pub mountains_biome: u8,
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
            mountains_biome: 24,
            default_biome: BiomeConfig {
                name: "Default Biome".to_string(),
                color: [255, 0, 255],
                simple_color: [255, 0, 255],
                resources: vec![],
                habitability: 0.0,
            },
            biomes: make_default_biomes(),
        }
    }
}

/// Config for the resource generation.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct ResourcesConfig {
    #[name("Chunk Size")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=255))]
    pub chunk_size: u8,
    #[name("Resource Types")]
    #[control(SidebarStructList)]
    pub types: Vec<ResourceType>,
    #[name("Resource Chunks")]
    #[control(SidebarStructList)]
    pub chunks: Vec<ResourceChunk>,
}

impl Default for ResourcesConfig {
    fn default() -> Self {
        Self {
            chunk_size: 36,
            chunks: Default::default(),
            types: make_default_resources(),
        }
    }
}
