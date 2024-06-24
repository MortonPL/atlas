use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui,
    config::{AtlasConfig, ClimatePreviewMode, WorldModel},
    serde_derive::{Deserialize, Serialize},
    ui::{
        sidebar::{MakeUi, SidebarControl, SidebarSlider},
        UiEditableEnum,
    },
    MakeUi, UiEditableEnum,
};

use crate::config::{make_default_biomes, BiomeConfig};

/// Complete configuration for the history simulator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct AtlasSimConfig {
    pub general: GeneralConfig,
    pub scenario: ScenarioConfig,
    pub climate: ClimateConfig,
}

impl AtlasConfig for AtlasSimConfig {
    fn get_world_size(&self) -> (u32, u32) {
        (self.general.world_size[0], self.general.world_size[1])
    }

    fn get_preview_model(&self) -> WorldModel {
        WorldModel::Flat
    }

    fn get_climate_preview(&self) -> ClimatePreviewMode {
        self.climate.preview_mode
    }
}

impl AtlasSimConfig {
    pub fn world_to_map(&self, point: (f32, f32)) -> (u32, u32) {
        let (width, height) = self.get_world_size();
        (
            (point.0 * 100.0 + width as f32 / 2.0) as u32,
            (-point.1 * 100.0 + height as f32 / 2.0) as u32,
        )
    }

    pub fn map_to_world(&self, point: (u32, u32)) -> (f32, f32) {
        let (width, height) = self.get_world_size();
        (
            (point.0 as f32 - width as f32 / 2.0) / 100.0,
            (height as f32 / 2.0 - (point.1 as f32)) / 100.0,
        )
    }
}

/// Config for general world settings and preview.
#[derive(Debug, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct GeneralConfig {
    pub tile_resolution: f32,
    pub world_size: [u32; 2],
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            tile_resolution: 10.0,
            world_size: [360, 180],
        }
    }
}

/// Initial scenario config.
#[derive(Default, Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct ScenarioConfig {
    #[name("Number of Starting Points")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=255))]
    pub num_starts: u8,
    #[name("Random Start Point Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub random_point_algorithm: StartPointAlgorithm,
    #[name("Starting Points")]
    #[control(SidebarStructList)]
    pub start_points: Vec<StartingPoint>,
}

#[derive(Default, Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct StartingPoint {
    #[name("Locked")]
    #[control(SidebarCheckbox)]
    pub locked: bool,
    #[name("Position")]
    #[control(SidebarSliderN)]
    pub position: [u32; 2],
    #[name("Starting Point Owner")]
    #[control(SidebarEnumSubsection)]
    pub owner: StartingPointOwner,
}

#[derive(Default, Debug, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(crate = "atlas_lib::serde")]
#[serde(rename_all = "lowercase")]
pub enum StartingPointOwner {
    #[default]
    Random,
    Picked(u8),
}

impl MakeUi for StartingPointOwner {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        match self {
            StartingPointOwner::Random => {}
            StartingPointOwner::Picked(x) => {
                SidebarSlider::new(ui, "Index", x).show(None);
            }
        }
    }
}

#[derive(Default, Debug, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(crate = "atlas_lib::serde")]
#[serde(rename_all = "lowercase")]
pub enum StartPointAlgorithm {
    Uniform,
    Weighted,
    #[default]
    WeightedArea,
}

/// Config for the climate rules.
#[derive(Debug, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct ClimateConfig {
    #[serde(skip)]
    pub preview_mode: ClimatePreviewMode,
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
                habitability: 1.0,
                productivity: 1.0,
            },
            biomes: make_default_biomes(),
        }
    }
}
