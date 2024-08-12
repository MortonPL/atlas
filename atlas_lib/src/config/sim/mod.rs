mod defaults;

use weighted_rand::table::WalkerTable;

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
    #[add(clamp_range(1..=1000))]
    pub num_starts: u32,
    #[name("Random Start Point Algorithm")]
    #[control(SidebarEnumDropdown)]
    pub random_point_algorithm: StartPointAlgorithm,
    #[name("Starting Land Claim Points")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000000.0))]
    pub starting_land_claim_points: f32,
    #[name("Starting Population")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000000.0))]
    pub start_pop: f32,
    #[name("Policy Distribution Mean")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub policy_mean: f32,
    #[name("Policy Distribution Deviation")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub policy_deviation: f32,
    #[name("Starting Points")]
    #[control(SidebarStructList)]
    pub start_points: Vec<StartingPoint>,
}

#[derive(Default, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct StartingPoint {
    #[name("Locked Position")]
    #[control(SidebarCheckbox)]
    pub position_locked: bool,
    #[name("Position")]
    #[control(SidebarSliderN)]
    pub position: [u32; 2],
    #[name("Locked Polity Color")]
    #[control(SidebarCheckbox)]
    pub color_locked: bool,
    #[name("Locked Polity Policies")]
    #[control(SidebarCheckbox)]
    pub policy_locked: bool,
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

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct PolityConfig {
    pub color: [u8; 3],
    pub population: f32,
    pub policies: [f32; 6],
    pub next_policy: u32,
}

impl MakeUi for PolityConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarColor::new(ui, "Color", &mut self.color).show(None);
        SidebarSlider::new(ui, "Population", &mut self.population)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        ui.heading("Policies");
        ui.end_row();
        for (x, label) in self.policies.iter_mut().zip([
            "Expansionist",
            "Competitive",
            "Mercantile",
            "Militarist",
            "Progressive",
            "Legalist",
        ]) {
            SidebarSlider::new(ui, label, x).show(None);
        }
    }
}

/// Config for general world settings and preview.
#[derive(Default, Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct RulesConfig {
    #[name("Economy")]
    #[control(SidebarStructSection)]
    pub economy: EconomyConfig,
    #[name("Technology")]
    #[control(SidebarStructSection)]
    pub tech: TechnologiesConfig,
    #[name("Culture")]
    #[control(SidebarStructSection)]
    pub culture: CulturesConfig,
    #[name("Region")]
    #[control(SidebarStructSection)]
    pub region: RegionsConfig,
    #[name("Combat")]
    #[control(SidebarStructSection)]
    pub combat: CombatConfig,
    #[name("Diplomacy")]
    #[control(SidebarStructSection)]
    pub diplomacy: DiplomacyConfig,
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct EconomyConfig {
    pub pop_growth: f32,
    pub max_health_penalty: f32,
    pub min_pop: f32,
    pub base_supply_need: f32,
    pub base_industry_need: f32,
    pub base_wealth_need: f32,
    pub chaos_supply_loss: f32,
    pub chaos_industry_loss: f32,
    pub chaos_wealth_loss: f32,
    pub crime_rate: f32,
    pub resources: [ResConfig; 10],
}

impl MakeUi for EconomyConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Monthly Base Pop Growth", &mut self.pop_growth)
            .clamp_range(0.0..=1.0)
            .show(None);
        SidebarSlider::new(ui, "Maximum Healthcare Penalty", &mut self.max_health_penalty)
            .clamp_range(0.0..=1.0)
            .show(None);
        SidebarSlider::new(ui, "Supply Need per Pop", &mut self.base_supply_need)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Industry Need per Pop", &mut self.base_industry_need)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Wealth Need per Pop", &mut self.base_wealth_need)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Supply Loss to Chaos", &mut self.chaos_supply_loss)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Industry Loss to Chaos", &mut self.chaos_industry_loss)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Wealth Loss to Chaos", &mut self.chaos_wealth_loss)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Crime Rate", &mut self.crime_rate)
            .clamp_range(0.0..=1.0)
            .show(None);
        ui.heading("Resources");
        ui.end_row();
        for (x, label) in self.resources.iter_mut().zip([
            "Supply",
            "Industry (General)",
            "Civilian Industry",
            "Military Industry",
            "Wealth (General)",
            "Research",
            "Culture",
            "Loyalty",
            "Industry Tributes",
            "Wealth Tributes",
        ]) {
            ui.label(label);
            ui.end_row();
            x.make_ui(ui);
        }
    }
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct ResConfig {
    #[name("Efficiency")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000000.0))]
    pub efficiency: f32,
    #[name("Efficiency Over Capacity")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000000.0))]
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
    pub speed_major: f32,
    pub speed_minor: f32,
    pub max_level_major: f32,
    pub max_level_minor: f32,
    pub bonus_major: f32,
    pub bonus_minor: f32,
    pub base_decay: f32,
    pub level_decay: f32,
    pub level_difficulty: f32,
    pub techs: [TechConfig; 10],
}

impl MakeUi for TechnologiesConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Major Level Speed", &mut self.speed_major)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Minor Level Speed", &mut self.speed_minor)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Maximum Major Level", &mut self.max_level_major)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Maximum Minor Level", &mut self.max_level_minor)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Major Level Bonus", &mut self.bonus_major)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Minor Level Bonus", &mut self.bonus_minor)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Base Decay", &mut self.base_decay)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Major Level Decay", &mut self.level_decay)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Major Level Difficulty Increase", &mut self.level_difficulty)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        for (x, label) in self.techs.iter_mut().zip([
            "Geoscience",
            "Medicine",
            "Engineering",
            "Metallurgy",
            "Philosophy",
            "Mathematics",
            "Management",
            "Law",
            "Linguistics",
            "Military Tech",
        ]) {
            ui.label(label);
            ui.end_row();
            x.make_ui(ui);
        }
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
    pub great_event_chance_max: f32,
    pub great_work_bonus: f32,
    pub great_person_bonus: f32,
    pub great_person_chance: f32,
    pub great_person_duration: u32,
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
        SidebarSlider::new(ui, "Great Work Bonus", &mut self.great_work_bonus)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Great Person Bonus", &mut self.great_person_bonus)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Great Person Duration", &mut self.great_person_duration).show(None);

        for (x, label) in self.traditions.iter_mut().zip([
            "Pioneering",
            "Creative",
            "Inventive",
            "Artistic",
            "Industrious",
            "Honorable",
            "Diplomatic",
            "Militant",
        ]) {
            ui.label(label);
            ui.end_row();
            x.make_ui(ui);
        }
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
pub struct RegionsConfig {
    pub new_city_cost: f32,
    pub min_split_size: u32,
    pub land_claim_cost: f32,
    pub base_exp_speed: f32,
    pub base_dev_speed: f32,
    pub max_dev_level: f32,
    pub dev_level_cost: f32,
    pub dev_bonus: f32,
    pub base_capacity: f32,
    pub structures: [StructureConfig; 7],
}

impl MakeUi for RegionsConfig {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Minimum Size to Split", &mut self.min_split_size).show(None);
        SidebarSlider::new(ui, "New City Cost", &mut self.new_city_cost)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        SidebarSlider::new(ui, "Land Claim Cost Per Tile", &mut self.land_claim_cost)
            .clamp_range(0.0..=10000.0)
            .show(None);
        SidebarSlider::new(ui, "Base Expansion Speed", &mut self.base_exp_speed)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Base Development Speed", &mut self.base_dev_speed)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Development Level Cost", &mut self.dev_level_cost)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Maximum Development Level", &mut self.max_dev_level)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Development Bonus", &mut self.dev_bonus)
            .clamp_range(0.0..=1000.0)
            .show(None);
        SidebarSlider::new(ui, "Base Capacity", &mut self.base_capacity)
            .clamp_range(0.0..=1000000.0)
            .show(None);
        for (x, label) in self.structures.iter_mut().zip([
            "Hospital",
            "Manufacture",
            "Forge",
            "University",
            "Amphitheater",
            "Courthouse",
            "Fortress",
        ]) {
            ui.label(label);
            ui.end_row();
            x.make_ui(ui);
        }
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

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct CombatConfig {
    #[name("Action Weights (Attacker)")]
    #[control(SidebarSliderN)]
    pub action_weights_attacker: [u32; 8],
    #[name("Action Weights (Defender)")]
    #[control(SidebarSliderN)]
    pub action_weights_defender: [u32; 8],
    #[serde(skip)]
    pub action_table_attacker: WalkerTable,
    #[serde(skip)]
    pub action_table_defender: WalkerTable,
    #[name("Assault Bonus")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub assault_bonus: f32,
    #[name("Maneouver Bonus")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub maneouver_bonus: f32,
    #[name("Charge Bonus")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub charge_bonus: f32,
    #[name("Rally Bonus")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub rally_bonus: f32,
    #[name("Skirmish Bonus")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub skirmish_bonus: f32,
    #[name("Delay Bonus")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub delay_bonus: f32,
    #[name("Skirmish Penalty")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub skirmish_penalty: f32,
    #[name("Delay Penalty")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub delay_penalty: f32,
    #[name("Siege Bonus")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub siege_bonus: f32,
    #[name("Siege Penalty")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub siege_penalty: f32,
    #[name("Fortify Bonus")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub fortify_bonus: f32,
    #[name("Fortify Penalty")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub fortify_penalty: f32,
    #[name("Mobilization Speed")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub mobilization_speed: f32,
    #[name("Combat Randomness")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub randomness: f32,
    #[name("Material Damage Fatality")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub fatality: f32,
    #[name("Morale Damage Fragility")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub fragility: f32,
    #[name("Material Advantage Power")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub material_advantage: f32,
    #[name("Morale Advantage Power")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub morale_advantage: f32,
    #[name("Morale Breakdown Multiplier")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub breakdown: f32,
    #[name("Morale to Material Ratio Cap")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub morale_cap: f32,
    #[name("Equipment to Manpower Ratio")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub equipment_manpower_ratio: f32,
    #[name("Damage to Fort Ratio")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub fort_damage: f32,
    #[name("Monthly Defender Attrition")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub base_defender_attrition: f32,
    #[name("Monthly Attacker Attrition")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub base_attacker_attrition: f32,
    #[name("Attrition From Combat Damage")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub combat_attrition: f32,
    #[name("Attrition From Civilian Damage")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub civilian_attrition: f32,
    #[name("Civilian Damage From Unabsorbed Damage %")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub civilian_damage: f32,
    #[name("Number of Tribute Payments")]
    #[control(SidebarSlider)]
    pub tribute_time: u32,
    #[name("Economy % to Tribute")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub tribute_ratio: f32,
}

#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
pub struct DiplomacyConfig {
    #[name("Initial Peace Length")]
    #[control(SidebarSlider)]
    pub initial_peace_length: u32,
    #[name("Policy Change Time Mean")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub policy_time_mean: f32,
    #[name("Policy Change Time Deviation")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub policy_time_dev: f32,
    #[name("Relations Change Speed")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub relations_speed: f32,
    #[name("Ally threshold")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub ally_threshold: f32,
    #[name("Friend threshold")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub friend_threshold: f32,
    #[name("Rival threshold")]
    #[control(SidebarSlider)]
    #[add(clamp_range(-1.0..=0.0))]
    pub rival_threshold: f32,
    #[name("Enemy threshold")]
    #[control(SidebarSlider)]
    #[add(clamp_range(-1.0..=0.0))]
    pub enemy_threshold: f32,
}
