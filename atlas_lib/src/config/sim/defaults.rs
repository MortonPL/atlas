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
                1.0, // 3 Civilian Goods
                1.0, // 4 Military Equipment
                1.0, // 5 Research
                1.0, // 6 Culture
                1.0, // 7 Treasure
                1.0, // 8 Services
            ],
        }
    }
}

impl Default for TechnologiesConfig {
    fn default() -> Self {
        Self {
            base_speed: 0.001,
            max_level: 100.0,
            techs: [
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
                TechConfig { strength: 1.0, cost: 1.0 },
            ],
        }
    }
}
