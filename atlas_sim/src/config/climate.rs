use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui,
    serde_derive::{Deserialize, Serialize},
    ui::sidebar::*,
    MakeUi,
};

/// A single climate biome.
#[derive(Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct BiomeConfig {
    #[name("Name")]
    #[control(SidebarHeader)]
    pub name: String,
    #[name("Color")]
    #[control(SidebarColor)]
    pub color: [u8; 3],
    #[name("Color (Simplified View)")]
    #[control(SidebarColor)]
    pub simple_color: [u8; 3],
    #[name("Habitability")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    #[add(speed(0.1))]
    pub habitability: f32,
}

/// Create a list of default biomes for general use.
pub fn make_default_biomes() -> Vec<BiomeConfig> {
    vec![
        BiomeConfig {
            // 0
            name: "Water".to_string(),
            color: [80, 200, 255],
            simple_color: [80, 200, 255],
            habitability: 1.0,
        },
        BiomeConfig {
            // 1
            name: "Polar Desert".to_string(),
            color: [225, 245, 250],
            simple_color: [225, 245, 250],
            habitability: 0.0,
        },
        BiomeConfig {
            // 2
            name: "Arctic Desert".to_string(),
            color: [170, 185, 190],
            simple_color: [225, 245, 250],
            habitability: 0.1,
        },
        BiomeConfig {
            // 3
            name: "Arctic Desert (Wet)".to_string(),
            color: [145, 160, 160],
            simple_color: [225, 245, 250],
            habitability: 0.1,
        },
        BiomeConfig {
            // 4
            name: "Tundra".to_string(),
            color: [140, 195, 175],
            simple_color: [140, 195, 175],
            habitability: 0.3,
        },
        BiomeConfig {
            // 5
            name: "Tundra (Wet)".to_string(),
            color: [90, 195, 155],
            simple_color: [140, 195, 175],
            habitability: 0.3,
        },
        BiomeConfig {
            // 6
            name: "Boreal Forest".to_string(),
            color: [90, 170, 140],
            simple_color: [90, 170, 140],
            habitability: 0.5,
        },
        BiomeConfig {
            // 7
            name: "Boreal Forest (Wet)".to_string(),
            color: [40, 140, 100],
            simple_color: [90, 170, 140],
            habitability: 0.5,
        },
        BiomeConfig {
            // 8
            name: "Cold Desert (Arid)".to_string(),
            color: [160, 155, 140],
            simple_color: [185, 175, 140],
            habitability: 0.1,
        },
        BiomeConfig {
            // 9
            name: "Cold Desert".to_string(),
            color: [185, 175, 140],
            simple_color: [185, 175, 140],
            habitability: 0.3,
        },
        BiomeConfig {
            // 10
            name: "Temperate Grassland".to_string(),
            color: [180, 190, 130],
            simple_color: [90, 200, 75],
            habitability: 0.8,
        },
        BiomeConfig {
            // 11
            name: "Temperate Shrubland".to_string(),
            color: [150, 190, 130],
            simple_color: [90, 200, 75],
            habitability: 1.0,
        },
        BiomeConfig {
            // 12
            name: "Temperate Woodland".to_string(),
            color: [90, 200, 75],
            simple_color: [90, 200, 75],
            habitability: 1.0,
        },
        BiomeConfig {
            // 13
            name: "Temperate Forest".to_string(),
            color: [50, 185, 65],
            simple_color: [90, 200, 75],
            habitability: 1.0,
        },
        BiomeConfig {
            // 14
            name: "Temperate Rainforest".to_string(),
            color: [0, 180, 50],
            simple_color: [90, 200, 75],
            habitability: 0.9,
        },
        BiomeConfig {
            // 15
            name: "Temperate Rainforest (Wet)".to_string(),
            color: [90, 230, 45],
            simple_color: [90, 200, 75],
            habitability: 0.9,
        },
        BiomeConfig {
            // 16
            name: "Tropical Rainforest (Wet)".to_string(),
            color: [30, 255, 0],
            simple_color: [25, 210, 0],
            habitability: 0.9,
        },
        BiomeConfig {
            // 17
            name: "Tropical Rainforest".to_string(),
            color: [25, 210, 0],
            simple_color: [25, 210, 0],
            habitability: 0.9,
        },
        BiomeConfig {
            // 18
            name: "Tropical Forest".to_string(),
            color: [130, 210, 0],
            simple_color: [130, 210, 0],
            habitability: 1.0,
        },
        BiomeConfig {
            // 19
            name: "Savanna".to_string(),
            color: [180, 210, 45],
            simple_color: [130, 210, 0],
            habitability: 0.8,
        },
        BiomeConfig {
            // 20
            name: "Hot Desert".to_string(),
            color: [225, 220, 55],
            simple_color: [225, 220, 55],
            habitability: 0.2,
        },
        BiomeConfig {
            // 21
            name: "Hot Desert (Arid)".to_string(),
            color: [220, 195, 80],
            simple_color: [225, 220, 55],
            habitability: 0.1,
        },
        BiomeConfig {
            // 22
            name: "Tropical Rainforest (Extreme)".to_string(),
            color: [155, 180, 0],
            simple_color: [25, 210, 0],
            habitability: 0.5,
        },
        BiomeConfig {
            // 23
            name: "Scorched Desert".to_string(),
            color: [255, 215, 0],
            simple_color: [225, 220, 55],
            habitability: 0.0,
        },
        BiomeConfig {
            // 24
            name: "Mountains".to_string(),
            color: [80, 70, 60],
            simple_color: [80, 70, 60],
            habitability: 0.3,
        },
    ]
}
