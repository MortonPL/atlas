use crate::config::gen::*;

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

impl Default for ContinentsConfig {
    fn default() -> Self {
        Self {
            sea_level: 0.4,
            algorithm: Default::default(),
            influence_shape: Default::default(),
        }
    }
}

impl Default for TopographyConfig {
    fn default() -> Self {
        Self {
            coastal_erosion: 1,
            algorithm: NoiseAlgorithm::Perlin(FbmConfig {
                midpoint: QuadPointLerp {
                    start: 0.1,
                    midpoint: 0.0,
                    midpoint2: 0.2,
                    end: 0.65,
                    midpoint_position: 0.50,
                    midpoint2_position: 0.75,
                    ..Default::default()
                },
                ..Default::default()
            }),
            influence_shape: Default::default(),
        }
    }
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

impl Default for LatitudinalTemperatureLerp {
    fn default() -> Self {
        Self {
            north_pole_value: -50.0,
            north_arctic_value: -15.0,
            north_temperate_value: 11.0,
            north_tropic_value: 23.0,
            equator_value: 30.0,
            south_tropic_value: 23.0,
            south_temperate_value: 11.0,
            south_arctic_value: -15.0,
            south_pole_value: -50.0,
            non_linear_tropics: false,
        }
    }
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

impl Default for LatitudinalPrecipitationLerp {
    fn default() -> Self {
        Self {
            south_pole_value: 0.0,
            south_arctic_value: 300.0,
            south_temperate_value: 1800.0,
            south_tropic_value: 100.0,
            equator_value: 4000.0,
            north_tropic_value: 100.0,
            north_temperate_value: 1800.0,
            north_arctic_value: 300.0,
            north_pole_value: 0.0,
            non_linear_tropics: false,
        }
    }
}

impl Default for FbmConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            detail: 6,
            frequency: 3.0,
            neatness: 2.0,
            roughness: 0.5,
            bias: 0.0,
            midpoint: QuadPointLerp {
                start: 0.0,
                midpoint: 0.3333,
                midpoint2: 0.6666,
                end: 1.0,
                midpoint_position: 0.3333,
                midpoint2_position: 0.6666,
                ..Default::default()
            },
            offset: Default::default(),
        }
    }
}

impl Default for NoiseAlgorithm {
    fn default() -> Self {
        Self::Perlin(Default::default())
    }
}

impl Default for QuadPointLerp {
    fn default() -> Self {
        Self {
            start: 1.0,
            midpoint: 0.6666,
            midpoint2: 0.3333,
            end: 0.0,
            midpoint_position: 0.3333,
            midpoint2_position: 0.6666,
            diff1: 0.0,
            diff2: 0.0,
        }
    }
}

impl Default for InfluenceShape {
    fn default() -> Self {
        Self::None
    }
}

impl Default for InfluenceCircleConfig {
    fn default() -> Self {
        Self {
            influence_mode: Default::default(),
            influence_strength: 1.0,
            radius: 100,
            offset: Default::default(),
            midpoint: Default::default(),
        }
    }
}

impl Default for InfluenceStripConfig {
    fn default() -> Self {
        Self {
            influence_mode: Default::default(),
            influence_strength: 1.0,
            thickness: 100,
            length: 100,
            angle: 0,
            flip: false,
            offset: Default::default(),
            midpoint: Default::default(),
        }
    }
}

impl Default for InfluenceFbmConfig {
    fn default() -> Self {
        Self {
            influence_mode: Default::default(),
            influence_strength: 1.0,
            algorithm: Default::default(),
        }
    }
}

impl Default for InfluenceImageConfig {
    fn default() -> Self {
        Self {
            influence_mode: Default::default(),
            influence_strength: 1.0,
        }
    }
}
