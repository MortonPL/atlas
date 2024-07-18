mod defaults;

use crate::{
    bevy::prelude::*,
    bevy_egui,
    config::{
        climate::BiomeConfig, climate::ClimateConfig, deposit::DepositsConfig, AtlasConfig,
        ClimatePreviewMode, WorldModel,
    },
    serde_derive::{Deserialize, Serialize},
    ui::sidebar::*,
    ui::{sidebar::SidebarControl, UiEditableEnum},
    MakeUi, UiEditableEnum,
};

pub const CONFIG_NAME: &str = "atlassim.toml";

/// Complete configuration for the history simulator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct AtlasSimConfig {
    pub general: GeneralConfig,
    pub scenario: ScenarioConfig,
    pub rules: RulesConfig,
    pub deposits: DepositsConfig,
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

/// Config for general world settings and preview.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct GeneralConfig {
    pub world_size: [u32; 2],
}

/// Initial scenario config.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ScenarioConfig {
    #[name("Number of Starting Points")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=255))]
    pub num_starts: u8,
    #[name("Number of Starting Civilizations")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=255))]
    pub num_civs: u8,
    #[name("Random Start Point Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub random_point_algorithm: StartPointAlgorithm,
    #[name("Random Start Civilization Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub random_civ_algorithm: StartCivAlgorithm,
    #[name("Starting Points")]
    #[control(SidebarStructList)]
    pub start_points: Vec<StartingPoint>,
    #[name("Starting Civilizations")]
    #[control(SidebarStructList)]
    pub start_civs: Vec<CivConfig>,
}

#[derive(Default, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct StartingPoint {
    #[name("Locked Position")]
    #[control(SidebarCheckbox)]
    pub position_locked: bool,
    #[name("Position")]
    #[control(SidebarSliderN)]
    pub position: [u32; 2],
    #[name("Locked Cilization")]
    #[control(SidebarCheckbox)]
    pub civ_locked: bool,
    #[name("Civilization Index")]
    #[control(SidebarSlider)]
    pub civ: u8,
    #[name("Locked Polity Color")]
    #[control(SidebarCheckbox)]
    pub color_locked: bool,
    #[name("Polity")]
    #[control(SidebarStructSection)]
    pub polity: PolityConfig,
}

#[derive(Default, Debug, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum StartPointAlgorithm {
    Uniform,
    Weighted,
    #[default]
    WeightedArea,
    WeightedSquared,
    WeightedSquaredArea,
}

#[derive(Default, Debug, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum StartCivAlgorithm {
    Repeated,
    #[default]
    Choice,
}

#[derive(Default, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct CivConfig {}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct PolityConfig {
    #[name("Color")]
    #[control(SidebarColor)]
    pub color: [u8; 3],
    #[name("Population")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000000.0))]
    pub population: f32,
}

/// Config for general world settings and preview.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct RulesConfig {
    #[name("Tile Resolution [km]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1.0..=100.0))]
    pub tile_resolution: f32,
    #[name("Starting Land Claim Points")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10000.0))]
    pub starting_land_claim_points: f32,
    #[name("Land Claim Cost Per Tile")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10000.0))]
    pub land_claim_cost: f32,
    #[name("Supply Consumption per Pop")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10000.0))]
    pub supply_per_pop: f32,
    #[name("Monthly Base Pop Growth")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub pop_growth: f32,
    pub resource: ResourceConfig,
}

/// Config for population jobs and production.
#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct ResourceConfig {
    pub efficiency: [f32; 9],
}

impl MakeUi for ResourceConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        ui.heading("Resource Efficiency");
        ui.end_row();
        SidebarSlider::new(ui, "Supply", &mut self.efficiency[0])
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Construction", &mut self.efficiency[1])
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Maintenance", &mut self.efficiency[2])
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Civilian Goods", &mut self.efficiency[3])
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Military Equipment", &mut self.efficiency[4])
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Research", &mut self.efficiency[5])
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Culture", &mut self.efficiency[6])
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Services", &mut self.efficiency[7])
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Treasure", &mut self.efficiency[8])
            .clamp_range(0.0..=1000.0)
            .show(None);
        ui.end_row();
    }
}
