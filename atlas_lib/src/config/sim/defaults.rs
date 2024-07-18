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
        }
    }
}

/// Create a list of default job types for general use.
pub fn make_default_jobs() -> [JobType; 3] {
    [
        // 0
        JobType {
            name: "Agriculture Worker".to_string(),
            efficiency: 1.1,
        },
        // 1
        JobType {
            name: "Industry Worker".to_string(),
            efficiency: 1.0,
        },
        // 2
        JobType {
            name: "Craftsman".to_string(),
            efficiency: 1.0,
        },
    ]
}
