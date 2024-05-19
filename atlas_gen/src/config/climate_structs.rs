use crate::config::BiomeConfig;

/// Create a list of default biomes for general use.
pub fn make_default_biomes() -> Vec<BiomeConfig> {
    vec![
        BiomeConfig {
            // 0
            name: "Default Biome".to_string(),
            color: [255, 0, 255],
            simple_color: [255, 0, 255],
            productivity: 1.0,
        },
        BiomeConfig {
            // 1
            name: "Polar Desert".to_string(),
            color: [225, 245, 250],
            simple_color: [225, 245, 250],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 2
            name: "Arctic Desert".to_string(),
            color: [170, 185, 190],
            simple_color: [225, 245, 250],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 3
            name: "Arctic Desert (Wet)".to_string(),
            color: [145, 160, 160],
            simple_color: [225, 245, 250],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 4
            name: "Tundra".to_string(),
            color: [140, 195, 175],
            simple_color: [140, 195, 175],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 5
            name: "Tundra (Wet)".to_string(),
            color: [90, 195, 155],
            simple_color: [140, 195, 175],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 6
            name: "Boreal Forest".to_string(),
            color: [90, 170, 140],
            simple_color: [90, 170, 140],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 7
            name: "Boreal Forest (Wet)".to_string(),
            color: [40, 140, 100],
            simple_color: [90, 170, 140],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 8
            name: "Cold Desert (Arid)".to_string(),
            color: [160, 155, 140],
            simple_color: [185, 175, 140],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 9
            name: "Cold Desert".to_string(),
            color: [185, 175, 140],
            simple_color: [185, 175, 140],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 10
            name: "Temperate Grassland".to_string(),
            color: [180, 190, 130],
            simple_color: [90, 200, 75],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 11
            name: "Temperate Shrubland".to_string(),
            color: [150, 190, 130],
            simple_color: [90, 200, 75],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 12
            name: "Temperate Woodland".to_string(),
            color: [90, 200, 75],
            simple_color: [90, 200, 75],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 13
            name: "Temperate Forest".to_string(),
            color: [50, 185, 65],
            simple_color: [90, 200, 75],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 14
            name: "Temperate Rainforest".to_string(),
            color: [0, 180, 50],
            simple_color: [90, 200, 75],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 15
            name: "Temperate Rainforest (Wet)".to_string(),
            color: [90, 230, 45],
            simple_color: [90, 200, 75],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 16
            name: "Tropical Rainforest (Wet)".to_string(),
            color: [30, 255, 0],
            simple_color: [25, 210, 0],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 17
            name: "Tropical Rainforest".to_string(),
            color: [25, 210, 0],
            simple_color: [25, 210, 0],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 18
            name: "Tropical Forest".to_string(),
            color: [130, 210, 0],
            simple_color: [130, 210, 0],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 19
            name: "Savanna".to_string(),
            color: [180, 210, 45],
            simple_color: [130, 210, 0],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 20
            name: "Hot Desert".to_string(),
            color: [225, 220, 55],
            simple_color: [225, 220, 55],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 21
            name: "Hot Desert (Arid)".to_string(),
            color: [220, 195, 80],
            simple_color: [225, 220, 55],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 22
            name: "Tropical Rainforest (Extreme)".to_string(),
            color: [155, 180, 0],
            simple_color: [25, 210, 0],
            productivity: 0.1, // TODO find good value
        },
        BiomeConfig {
            // 23
            name: "Scorched Desert".to_string(),
            color: [255, 215, 0],
            simple_color: [225, 220, 55],
            productivity: 0.1, // TODO find good value
        },
    ]
}
