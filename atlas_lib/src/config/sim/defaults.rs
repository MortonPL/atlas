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
            start_pop: 1.0,
            random_point_algorithm: Default::default(),
            policy_mean: 0.5,
            policy_deviation: 0.2,
            start_points: vec![],
            starting_land_claim_points: 100.0,
        }
    }
}

impl Default for PolityConfig {
    fn default() -> Self {
        Self {
            color: Default::default(),
            population: 1.0,
            policies: [0.5; 6],
            next_policy: 0,
        }
    }
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            base_supply_need: 1.0,
            base_industry_need: 0.01,
            base_wealth_need: 0.01,
            chaos_supply_loss: 0.0,
            chaos_industry_loss: 0.03,
            chaos_wealth_loss: 0.03,
            pop_growth: 0.001,
            max_health_penalty: 0.9,
            min_pop: 1.0,
            crime_rate: 0.1,
            rebelion_speed: 0.1,
            resources: [
                ResConfig {
                    efficiency: 1.1,
                    over_cap_efficiency: 1.0,
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
                ResConfig::default(),
            ],
        }
    }
}

impl Default for TechnologiesConfig {
    fn default() -> Self {
        Self {
            speed_major: 0.001,
            speed_minor: 0.002,
            max_level_major: 10.0,
            max_level_minor: 10.0,
            bonus_major: 0.1,
            bonus_minor: 0.01,
            base_decay: 0.01,
            level_decay: 0.1,
            level_difficulty: 2.0,
            techs: Default::default(),
        }
    }
}

impl Default for CulturesConfig {
    fn default() -> Self {
        Self {
            base_speed: 0.01,
            base_decay: 0.05,
            max_level: 10.0,
            level_bonus: 0.025,
            level_decay: 0.1,
            heritage_ratio: 0.1,
            great_event_heritage: 100000.0,
            great_person_chance: 0.5,
            great_event_chance_max: 0.1,
            great_work_bonus: 1.0,
            great_person_bonus: 3.0,
            great_person_duration: 120,
            traditions: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                TraditionConfig {
                    strength: 10.0,
                    cost: 1.0,
                },
            ],
        }
    }
}

impl Default for RegionsConfig {
    fn default() -> Self {
        Self {
            min_split_size: 13,
            new_city_cost: 100.0,
            land_claim_cost: 10.0,
            base_exp_speed: 0.1,
            base_dev_speed: 0.05,
            max_dev_level: 10.0,
            dev_level_cost: 1.0,
            dev_bonus: 0.05,
            base_capacity: 5.0,
            structures: Default::default(),
        }
    }
}

impl Default for CombatConfig {
    fn default() -> Self {
        Self {
            action_weights_attacker: [5, 5, 3, 2, 2, 3, 4, 1],
            action_weights_defender: [5, 5, 2, 3, 3, 2, 1, 4],
            action_table_attacker: Default::default(),
            action_table_defender: Default::default(),
            assault_bonus: 0.3,
            maneouver_bonus: 0.3,
            rally_bonus: 0.3,
            charge_bonus: 0.3,
            skirmish_bonus: 0.3,
            delay_bonus: 0.3,
            skirmish_penalty: 0.5,
            delay_penalty: 0.5,
            siege_penalty: 0.5,
            siege_bonus: 0.3,
            fortify_bonus: 1.0,
            fortify_penalty: 0.3,
            mobilization_speed: 0.1,
            base_recruit_factor: 0.2,
            randomness: 0.2,
            fatality: 0.1,
            fragility: 0.1,
            material_advantage: 1.0,
            morale_advantage: 2.0,
            equipment_manpower_ratio: 3.0,
            breakdown: 2.0,
            morale_cap: 3.0,
            fort_damage: 0.005,
            base_defender_attrition: 0.005,
            base_attacker_attrition: 0.005,
            combat_attrition: 0.3,
            civilian_attrition: 0.1,
            civilian_damage: 0.01,
            civilian_damage_max: 0.02,
            claim_difficulty: 1.5,
            base_rebel_rate: 0.2,
            tribute_time: 60,
            tribute_ratio: 0.1,
        }
    }
}

impl Default for DiplomacyConfig {
    fn default() -> Self {
        Self {
            initial_peace_length: 30,
            truce_length: 5,
            policy_time_mean: 40.0,
            policy_time_dev: 10.0,
            relations_speed: 0.1,
            ally_threshold: 0.8,
            friend_threshold: 0.5,
            rival_threshold: -0.5,
            enemy_threshold: -0.8,
        }
    }
}
