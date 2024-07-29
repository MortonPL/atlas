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
    #[name("Misc")]
    #[control(SidebarStructSection)]
    pub misc: MiscConfig,
    #[name("Economy")]
    #[control(SidebarStructSection)]
    pub economy: EconomyConfig,
    #[name("Technology")]
    #[control(SidebarStructSection)]
    pub tech: TechnologiesConfig,
    #[name("Culture")]
    #[control(SidebarStructSection)]
    pub culture: CulturesConfig,
    #[name("City")]
    #[control(SidebarStructSection)]
    pub city: CitiesConfig,
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct MiscConfig {
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
    #[name("Default Manpower Split")]
    #[control(SidebarSliderN)]
    pub default_manpower_split: [f32; 3],
    #[name("Default Industry Split")]
    #[control(SidebarSliderN)]
    pub default_industry_split: [f32; 3],
    #[name("Default Wealth Split")]
    #[control(SidebarSliderN)]
    pub default_wealth_split: [f32; 4],
    #[name("Default Technology Split")]
    #[control(SidebarSliderN)]
    pub default_tech_split: [f32; 14],
    #[name("Default Tradition Split")]
    #[control(SidebarSliderN)]
    pub default_tradition_split: [f32; 8],
    #[name("Default Structures Split")]
    #[control(SidebarSliderN)]
    pub default_structure_split: [f32; 7],
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct EconomyConfig {
    pub pop_growth: f32,
    pub base_supply_need: f32,
    pub base_industry_need: f32,
    pub base_wealth_need: f32,
    pub resources: [ResConfig; 10],
}

impl MakeUi for EconomyConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Monthly Base Pop Growth", &mut self.pop_growth)
            .clamp_range(0.0..=1.0)
            .show(None);
        SidebarSlider::new(ui, "Supply Need per Pop", &mut self.base_supply_need)
            .clamp_range(0.0..=10000.0)
            .show(None);
        SidebarSlider::new(ui, "Industry Need per Pop", &mut self.base_industry_need)
            .clamp_range(0.0..=10000.0)
            .show(None);
        SidebarSlider::new(ui, "Wealth Need per Pop", &mut self.base_wealth_need)
            .clamp_range(0.0..=10000.0)
            .show(None);
        ui.heading("Resources");
        ui.end_row();
        ui.label("Supply");
        ui.end_row();
        self.resources[0].make_ui(ui);
        ui.label("Industry Consumption");
        ui.end_row();
        self.resources[1].make_ui(ui);
        ui.label("Construction");
        ui.end_row();
        self.resources[2].make_ui(ui);
        ui.label("Trade Goods");
        ui.end_row();
        self.resources[3].make_ui(ui);
        ui.label("Military Equipment");
        ui.end_row();
        self.resources[4].make_ui(ui);
        ui.label("Wealth Consumption");
        ui.end_row();
        self.resources[5].make_ui(ui);
        ui.label("Research");
        ui.end_row();
        self.resources[6].make_ui(ui);
        ui.label("Culture");
        ui.end_row();
        self.resources[7].make_ui(ui);
        ui.label("Administration");
        ui.end_row();
        self.resources[8].make_ui(ui);
        ui.label("Treasure");
        ui.end_row();
        self.resources[9].make_ui(ui);
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ResConfig {
    #[name("Efficiency")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub efficiency: f32,
    #[name("Efficiency Over Capacity")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub over_cap_efficiency: f32,
}

impl Default for ResConfig {
    fn default() -> Self {
        Self {
            efficiency: 1.0,
            over_cap_efficiency: 1.0,
        }
    }
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct TechnologiesConfig {
    pub base_speed: f32,
    pub base_decay: f32,
    pub max_level: f32,
    pub level_bonus: f32,
    pub level_decay: f32,
    pub techs: [TechConfig; 14],
}

impl MakeUi for TechnologiesConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Base Speed", &mut self.base_speed)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Base Decay", &mut self.base_decay)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Maximum Level", &mut self.max_level)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Level Bonus", &mut self.level_bonus)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Level Decay", &mut self.level_decay)
            .clamp_range(0.0..=1000.0)
            .show(None);
        ui.label("Agriculture");
        ui.end_row();
        self.techs[0].make_ui(ui);
        ui.label("Astronomy");
        ui.end_row();
        self.techs[1].make_ui(ui);
        ui.label("Forestry");
        ui.end_row();
        self.techs[2].make_ui(ui);
        ui.label("Geology");
        ui.end_row();
        self.techs[3].make_ui(ui);
        ui.label("Medicine");
        ui.end_row();
        self.techs[4].make_ui(ui);
        ui.label("Architecture");
        ui.end_row();
        self.techs[5].make_ui(ui);
        ui.label("Engineering");
        ui.end_row();
        self.techs[6].make_ui(ui);
        ui.label("Metallurgy");
        ui.end_row();
        self.techs[7].make_ui(ui);
        ui.label("Philosophy");
        ui.end_row();
        self.techs[8].make_ui(ui);
        ui.label("Mathematics");
        ui.end_row();
        self.techs[9].make_ui(ui);
        ui.label("Finances");
        ui.end_row();
        self.techs[10].make_ui(ui);
        ui.label("Law");
        ui.end_row();
        self.techs[11].make_ui(ui);
        ui.label("Linguistics");
        ui.end_row();
        self.techs[12].make_ui(ui);
        ui.label("Physics");
        ui.end_row();
        self.techs[13].make_ui(ui);
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct TechConfig {
    #[name("Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub strength: f32,
    #[name("Cost")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub cost: f32,
}

impl Default for TechConfig {
    fn default() -> Self {
        Self {
            strength: 1.0,
            cost: 1.0,
        }
    }
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct CulturesConfig {
    pub base_speed: f32,
    pub base_decay: f32,
    pub max_level: f32,
    pub level_bonus: f32,
    pub level_decay: f32,
    pub heritage_ratio: f32,
    pub great_event_heritage: f32,
    pub great_person_chance: f32,
    pub great_event_chance_max: f32,
    pub traditions: [TraditionConfig; 8],
}

impl MakeUi for CulturesConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Base Speed", &mut self.base_speed)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Base Decay", &mut self.base_decay)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Maximum Level", &mut self.max_level)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Level Bonus", &mut self.level_bonus)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Level Decay", &mut self.level_decay)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Overflow Culture to Heritage Ratio", &mut self.heritage_ratio)
            .clamp_range(0.0..=1.0)
            .show(None);
        SidebarSlider::new(ui, "Great Event Heritage Cost", &mut self.great_event_heritage)
            .clamp_range(0.0..=10000000.0)
            .show(None);
        SidebarSlider::new(ui, "Great Event Max Chance", &mut self.great_event_chance_max)
            .clamp_range(0.0..=1.0)
            .show(None);
        SidebarSlider::new(ui, "Great Person Chance", &mut self.great_person_chance)
            .clamp_range(0.0..=1.0)
            .show(None);
        ui.label("Agrarian");
        ui.end_row();
        self.traditions[0].make_ui(ui);
        ui.label("Industrious");
        ui.end_row();
        self.traditions[1].make_ui(ui);
        ui.label("Mercantile");
        ui.end_row();
        self.traditions[2].make_ui(ui);
        ui.label("Progressive");
        ui.end_row();
        self.traditions[3].make_ui(ui);
        ui.label("Traditional");
        ui.end_row();
        self.traditions[4].make_ui(ui);
        ui.label("Legalist");
        ui.end_row();
        self.traditions[5].make_ui(ui);
        ui.label("Cooperative");
        ui.end_row();
        self.traditions[6].make_ui(ui);
        ui.label("Militant");
        ui.end_row();
        self.traditions[7].make_ui(ui);
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct TraditionConfig {
    #[name("Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub strength: f32,
    #[name("Cost")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub cost: f32,
}

impl Default for TraditionConfig {
    fn default() -> Self {
        Self {
            strength: 1.0,
            cost: 1.0,
        }
    }
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct CitiesConfig {
    pub base_speed: f32,
    pub upgrade_speed: f32,
    pub max_level: f32,
    pub level_cost: f32,
    pub base_capacity: f32,
    pub structures: [StructureConfig; 7],
}

impl MakeUi for CitiesConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Base Speed", &mut self.base_speed)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Level Upgrade Speed", &mut self.upgrade_speed)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Level Cost", &mut self.level_cost)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Maximum Level", &mut self.max_level)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Base Capacity", &mut self.base_capacity)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        ui.label("Hospital");
        ui.end_row();
        self.structures[0].make_ui(ui);
        ui.label("Manufacture");
        ui.end_row();
        self.structures[1].make_ui(ui);
        ui.label("Forge");
        ui.end_row();
        self.structures[2].make_ui(ui);
        ui.label("University");
        ui.end_row();
        self.structures[3].make_ui(ui);
        ui.label("Amphitheater");
        ui.end_row();
        self.structures[4].make_ui(ui);
        ui.label("Courhouse");
        ui.end_row();
        self.structures[5].make_ui(ui);
        ui.label("Fortress");
        ui.end_row();
        self.structures[6].make_ui(ui);
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct StructureConfig {
    #[name("Strength")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub strength: f32,
    #[name("Cost")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub cost: f32,
}

impl Default for StructureConfig {
    fn default() -> Self {
        Self {
            strength: 1.0,
            cost: 1.0,
        }
    }
}
