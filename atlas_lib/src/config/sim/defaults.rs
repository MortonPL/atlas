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
const STR_SPLIT: f32 = 1.0 / 7.0;

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            tile_resolution: 10.0,
            starting_land_claim_points: 1000.0,
            land_claim_cost: 100.0,
            base_supply_need: 1.0,
            base_industry_need: 0.1,
            base_wealth_need: 0.1,
            pop_growth: 0.001,
            resource: Default::default(),
            tech: Default::default(),
            culture: Default::default(),
            city: Default::default(),
            default_manpower_split: [0.1, 0.45, 0.45],
            default_industry_split: [0.5, 0.4, 0.1],
            default_wealth_split: [0.1, 0.1, 0.2, 0.6],
            default_tech_split: [TECH_SPLIT; 14],
            default_tradition_split: [TRAD_SPLIT; 8],
            default_structure_split: [STR_SPLIT; 7],
        }
    }
}

impl Default for ResourcesConfig {
    fn default() -> Self {
        Self {
            resources: [
                ResConfig {
                    efficiency: 1.1,
                    over_cap_efficiency: 1.0,
                },
                ResConfig::default(),
                ResConfig::default(),
                ResConfig {
                    efficiency: 1.0,
                    over_cap_efficiency: 0.1,
                },
                ResConfig {
                    efficiency: 1.0,
                    over_cap_efficiency: 0.1,
                },
                ResConfig::default(),
                ResConfig {
                    efficiency: 1.0,
                    over_cap_efficiency: 0.1,
                },
                ResConfig {
                    efficiency: 1.0,
                    over_cap_efficiency: 0.1,
                },
                ResConfig {
                    efficiency: 1.0,
                    over_cap_efficiency: 0.1,
                },
                ResConfig::default(),
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

impl Default for CitiesConfig {
    fn default() -> Self {
        Self {
            base_speed: 0.01,
            upgrade_speed: 0.3,
            max_level: 10.0,
            level_cost: 0.5,
            base_capacity: 1.0,
            structures: [
                StructureConfig::default(),
                StructureConfig::default(),
                StructureConfig::default(),
                StructureConfig::default(),
                StructureConfig::default(),
                StructureConfig::default(),
                StructureConfig::default(),
            ],
        }
    }
}
