use crate::config::sim::*;

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            world_size: [360, 180],
        }
    }
}

impl Default for ScenarioConfig {
    fn default() -> Self {
        Self {
            num_starts: 10,
            num_civs: 10,
            random_point_algorithm: Default::default(),
            random_civ_algorithm: Default::default(),
            start_points: vec![],
            start_civs: vec![],
        }
    }
}

impl Default for PolityConfig {
    fn default() -> Self {
        Self {
            color: Default::default(),
            population: 10.0,
        }
    }
}

const TECH_SPLIT: f32 = 1.0 / 14.0;
const TRAD_SPLIT: f32 = 1.0 / 8.0;

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            tile_resolution: 10.0,
            starting_land_claim_points: 1000.0,
            land_claim_cost: 100.0,
            supply_per_pop: 1.0,
            pop_growth: 0.001,
            resource: Default::default(),
            tech: Default::default(),
            culture: Default::default(),
            default_manpower_split: [0.1, 0.45, 0.45],
            default_industry_split: [0.5, 0.4, 0.1],
            default_wealth_split: [0.1, 0.1, 0.6, 0.2],
            default_tech_split: [TECH_SPLIT; 14],
            default_tradition_split: [TRAD_SPLIT; 8],
        }
    }
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            efficiency: [
                1.1, // 0 Supply
                1.0, // 1 Construction
                1.0, // 2 Maintenance
                1.0, // 3 Trade Goods
                1.0, // 4 Consumer Goods
                1.0, // 5 Military Equipment
                1.0, // 6 Research
                1.0, // 7 Culture
                1.0, // 8 Treasure
                1.0, // 9 Services
                1.0, // 10 Administration
            ],
        }
    }
}

impl Default for TechnologiesConfig {
    fn default() -> Self {
        Self {
            base_speed: 0.002,
            base_decay: 0.1,
            max_level: 100.0,
            level_bonus: 0.05,
            level_decay: 0.05,
            techs: [
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
                TechConfig::default(),
            ],
        }
    }
}

impl Default for CulturesConfig {
    fn default() -> Self {
        Self {
            base_speed: 0.01,
            base_decay: 0.1,
            max_level: 10.0,
            level_bonus: 0.015,
            level_decay: 0.1,
            heritage_ratio: 0.1,
            great_event_heritage: 1000000.0,
            great_person_chance: 0.5,
            great_event_chance_max: 0.1,
            traditions: [
                TraditionConfig::default(),
                TraditionConfig::default(),
                TraditionConfig::default(),
                TraditionConfig::default(),
                TraditionConfig::default(),
                TraditionConfig::default(),
                TraditionConfig::default(),
                TraditionConfig::default(),
            ],
        }
    }
}
