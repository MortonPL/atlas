use atlas_macro::UiEditableEnum;

use crate::{
    bevy::prelude::*,
    bevy_egui,
    config::IntoSimConfig,
    serde_derive::{Deserialize, Serialize},
    ui::{sidebar::*, UiEditableEnum},
    MakeUi,
};

/// Config for the climate generation.
#[derive(Debug, Clone, Deserialize, Resource, Serialize, MakeUi)]
pub struct ClimateConfig {
    #[serde(skip)]
    #[name("Preview Mode")]
    #[control(SidebarEnumDropdown)]
    pub preview_mode: ClimatePreviewMode,
    #[name("Mountains Biome Index")]
    #[control(SidebarSlider)]
    pub mountains_biome: u8,
    #[name("")]
    #[control(SidebarStructList)]
    pub biomes: Vec<BiomeConfig>,
    #[serde(skip)]
    pub default_biome: BiomeConfig,
}

impl Default for ClimateConfig {
    fn default() -> Self {
        Self {
            preview_mode: ClimatePreviewMode::DetailedColor,
            mountains_biome: 24,
            default_biome: BiomeConfig {
                name: "Default Biome".to_string(),
                color: [255, 0, 255],
                simple_color: [255, 0, 255],
                deposits: vec![],
                habitability: 0.0,
            },
            biomes: make_default_biomes(),
        }
    }
}

impl IntoSimConfig for ClimateConfig {
    fn into_sim_config(&self) -> Self {
        Self {
            preview_mode: self.preview_mode.clone(),
            mountains_biome: self.mountains_biome.clone(),
            default_biome: self.default_biome.clone(),
            biomes: self.biomes.iter().map(|x| x.into_sim_config()).collect(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum ClimatePreviewMode {
    SimplifiedColor,
    #[default]
    DetailedColor,
}

/// A single climate biome.
#[derive(Debug, Clone, Default, Deserialize, Resource, Serialize, MakeUi)]
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
    #[name("Resources")]
    #[control(SidebarStructList)]
    pub deposits: Vec<BiomeDeposit>,
    #[name("Habitability")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub habitability: f32,
}

impl IntoSimConfig for BiomeConfig {
    fn into_sim_config(&self) -> Self {
        Self {
            name: self.name.clone(),
            color: self.color.clone(),
            simple_color: self.simple_color.clone(),
            deposits: vec![],
            habitability: self.habitability.clone(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Resource, Serialize, MakeUi)]
pub struct BiomeDeposit {
    #[name("Resource ID")]
    #[control(SidebarSlider)]
    pub id: u32,
    #[name("Chance to Generate")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub chance: f32,
}

impl BiomeDeposit {
    pub fn new(id: u32, chance: f32) -> Self {
        Self { id, chance }
    }
}

/// Create a list of default biomes for general use.
pub fn make_default_biomes() -> Vec<BiomeConfig> {
    vec![
        BiomeConfig {
            // 0
            name: "Water".to_string(),
            color: [80, 200, 255],
            simple_color: [80, 200, 255],
            deposits: vec![BiomeDeposit::new(6, 0.8)],
            habitability: 1.0,
        },
        BiomeConfig {
            // 1
            name: "Polar Desert".to_string(),
            color: [225, 245, 250],
            simple_color: [225, 245, 250],
            deposits: vec![],
            habitability: 0.0,
        },
        BiomeConfig {
            // 2
            name: "Arctic Desert".to_string(),
            color: [170, 185, 190],
            simple_color: [225, 245, 250],
            deposits: vec![BiomeDeposit::new(3, 0.15), BiomeDeposit::new(4, 0.15)],
            habitability: 0.1,
        },
        BiomeConfig {
            // 3
            name: "Arctic Desert (Wet)".to_string(),
            color: [145, 160, 160],
            simple_color: [225, 245, 250],
            deposits: vec![BiomeDeposit::new(3, 0.15), BiomeDeposit::new(4, 0.15)],
            habitability: 0.1,
        },
        BiomeConfig {
            // 4
            name: "Tundra".to_string(),
            color: [140, 195, 175],
            simple_color: [140, 195, 175],
            deposits: vec![
                BiomeDeposit::new(0, 0.1),
                BiomeDeposit::new(2, 0.1),
                BiomeDeposit::new(3, 0.3),
                BiomeDeposit::new(4, 0.15),
                BiomeDeposit::new(5, 0.1),
            ],
            habitability: 0.3,
        },
        BiomeConfig {
            // 5
            name: "Tundra (Wet)".to_string(),
            color: [90, 195, 155],
            simple_color: [140, 195, 175],
            deposits: vec![
                BiomeDeposit::new(0, 0.2),
                BiomeDeposit::new(2, 0.2),
                BiomeDeposit::new(3, 0.4),
                BiomeDeposit::new(4, 0.2),
                BiomeDeposit::new(5, 0.2),
            ],
            habitability: 0.3,
        },
        BiomeConfig {
            // 6
            name: "Boreal Forest".to_string(),
            color: [90, 170, 140],
            simple_color: [90, 170, 140],
            deposits: vec![
                BiomeDeposit::new(0, 0.3),
                BiomeDeposit::new(2, 0.3),
                BiomeDeposit::new(3, 0.6),
                BiomeDeposit::new(4, 0.3),
                BiomeDeposit::new(5, 0.9),
            ],
            habitability: 0.5,
        },
        BiomeConfig {
            // 7
            name: "Boreal Forest (Wet)".to_string(),
            color: [40, 140, 100],
            simple_color: [90, 170, 140],
            deposits: vec![
                BiomeDeposit::new(0, 0.4),
                BiomeDeposit::new(2, 0.4),
                BiomeDeposit::new(3, 0.7),
                BiomeDeposit::new(4, 0.4),
                BiomeDeposit::new(5, 1.0),
            ],
            habitability: 0.5,
        },
        BiomeConfig {
            // 8
            name: "Cold Desert (Arid)".to_string(),
            color: [160, 155, 140],
            simple_color: [185, 175, 140],
            deposits: vec![
                BiomeDeposit::new(0, 0.1),
                BiomeDeposit::new(2, 0.1),
                BiomeDeposit::new(3, 0.15),
                BiomeDeposit::new(5, 0.1),
            ],
            habitability: 0.1,
        },
        BiomeConfig {
            // 9
            name: "Cold Desert".to_string(),
            color: [185, 175, 140],
            simple_color: [185, 175, 140],
            deposits: vec![
                BiomeDeposit::new(0, 0.2),
                BiomeDeposit::new(2, 0.4),
                BiomeDeposit::new(3, 0.1),
                BiomeDeposit::new(5, 0.1),
            ],
            habitability: 0.3,
        },
        BiomeConfig {
            // 10
            name: "Temperate Grassland".to_string(),
            color: [180, 190, 130],
            simple_color: [90, 200, 75],
            deposits: vec![
                BiomeDeposit::new(0, 0.3),
                BiomeDeposit::new(2, 0.9),
                BiomeDeposit::new(3, 0.3),
                BiomeDeposit::new(4, 0.15),
                BiomeDeposit::new(5, 0.1),
            ],
            habitability: 0.8,
        },
        BiomeConfig {
            // 11
            name: "Temperate Shrubland".to_string(),
            color: [150, 190, 130],
            simple_color: [90, 200, 75],
            deposits: vec![
                BiomeDeposit::new(0, 0.6),
                BiomeDeposit::new(1, 0.1),
                BiomeDeposit::new(2, 0.7),
                BiomeDeposit::new(3, 0.3),
                BiomeDeposit::new(4, 0.15),
                BiomeDeposit::new(5, 0.2),
            ],
            habitability: 1.0,
        },
        BiomeConfig {
            // 12
            name: "Temperate Woodland".to_string(),
            color: [90, 200, 75],
            simple_color: [90, 200, 75],
            deposits: vec![
                BiomeDeposit::new(0, 0.9),
                BiomeDeposit::new(1, 0.2),
                BiomeDeposit::new(2, 0.5),
                BiomeDeposit::new(3, 0.5),
                BiomeDeposit::new(4, 0.25),
                BiomeDeposit::new(5, 0.5),
            ],
            habitability: 1.0,
        },
        BiomeConfig {
            // 13
            name: "Temperate Forest".to_string(),
            color: [50, 185, 65],
            simple_color: [90, 200, 75],
            deposits: vec![
                BiomeDeposit::new(0, 0.5),
                BiomeDeposit::new(2, 0.2),
                BiomeDeposit::new(3, 0.7),
                BiomeDeposit::new(4, 0.3),
                BiomeDeposit::new(5, 0.9),
            ],
            habitability: 1.0,
        },
        BiomeConfig {
            // 14
            name: "Temperate Rainforest".to_string(),
            color: [0, 180, 50],
            simple_color: [90, 200, 75],
            deposits: vec![
                BiomeDeposit::new(0, 0.6),
                BiomeDeposit::new(2, 0.1),
                BiomeDeposit::new(3, 0.8),
                BiomeDeposit::new(4, 0.2),
                BiomeDeposit::new(5, 1.0),
            ],
            habitability: 0.9,
        },
        BiomeConfig {
            // 15
            name: "Temperate Rainforest (Wet)".to_string(),
            color: [90, 230, 45],
            simple_color: [90, 200, 75],
            deposits: vec![
                BiomeDeposit::new(0, 0.6),
                BiomeDeposit::new(3, 0.8),
                BiomeDeposit::new(4, 0.2),
                BiomeDeposit::new(5, 1.0),
            ],
            habitability: 0.9,
        },
        BiomeConfig {
            // 16
            name: "Tropical Rainforest (Wet)".to_string(),
            color: [30, 255, 0],
            simple_color: [25, 210, 0],
            deposits: vec![
                BiomeDeposit::new(0, 0.6),
                BiomeDeposit::new(3, 0.8),
                BiomeDeposit::new(4, 0.2),
                BiomeDeposit::new(5, 1.0),
            ],
            habitability: 0.9,
        },
        BiomeConfig {
            // 17
            name: "Tropical Rainforest".to_string(),
            color: [25, 210, 0],
            simple_color: [25, 210, 0],
            deposits: vec![
                BiomeDeposit::new(0, 0.6),
                BiomeDeposit::new(2, 0.1),
                BiomeDeposit::new(3, 0.8),
                BiomeDeposit::new(4, 0.2),
                BiomeDeposit::new(5, 1.0),
            ],
            habitability: 0.9,
        },
        BiomeConfig {
            // 18
            name: "Tropical Forest".to_string(),
            color: [130, 210, 0],
            simple_color: [130, 210, 0],
            deposits: vec![
                BiomeDeposit::new(0, 0.5),
                BiomeDeposit::new(2, 0.2),
                BiomeDeposit::new(3, 0.7),
                BiomeDeposit::new(4, 0.3),
                BiomeDeposit::new(5, 0.9),
            ],
            habitability: 1.0,
        },
        BiomeConfig {
            // 19
            name: "Savanna".to_string(),
            color: [180, 210, 45],
            simple_color: [130, 210, 0],
            deposits: vec![
                BiomeDeposit::new(0, 0.9),
                BiomeDeposit::new(1, 0.2),
                BiomeDeposit::new(2, 0.5),
                BiomeDeposit::new(3, 0.5),
                BiomeDeposit::new(4, 0.25),
                BiomeDeposit::new(5, 0.5),
            ],
            habitability: 0.8,
        },
        BiomeConfig {
            // 20
            name: "Hot Desert".to_string(),
            color: [225, 220, 55],
            simple_color: [225, 220, 55],
            deposits: vec![BiomeDeposit::new(3, 0.15), BiomeDeposit::new(4, 0.15)],
            habitability: 0.2,
        },
        BiomeConfig {
            // 21
            name: "Hot Desert (Arid)".to_string(),
            color: [220, 195, 80],
            simple_color: [225, 220, 55],
            deposits: vec![BiomeDeposit::new(3, 0.15), BiomeDeposit::new(4, 0.15)],
            habitability: 0.1,
        },
        BiomeConfig {
            // 22
            name: "Tropical Rainforest (Extreme)".to_string(),
            color: [155, 180, 0],
            simple_color: [25, 210, 0],
            deposits: vec![
                BiomeDeposit::new(0, 0.3),
                BiomeDeposit::new(3, 0.4),
                BiomeDeposit::new(4, 0.1),
                BiomeDeposit::new(5, 0.8),
            ],
            habitability: 0.5,
        },
        BiomeConfig {
            // 23
            name: "Scorched Desert".to_string(),
            color: [255, 215, 0],
            simple_color: [225, 220, 55],
            deposits: vec![],
            habitability: 0.0,
        },
        BiomeConfig {
            // 24
            name: "Mountains".to_string(),
            color: [80, 70, 60],
            simple_color: [80, 70, 60],
            deposits: vec![
                BiomeDeposit::new(0, 0.5),
                BiomeDeposit::new(1, 0.4),
                BiomeDeposit::new(2, 0.3),
                BiomeDeposit::new(3, 0.4),
            ],
            habitability: 0.4,
        },
    ]
}

// Temperature Unit (Meter).
// Byte representation is 1 [u] = 40 [m].

pub const ALTITUDE_MIN: f32 = 0.0;
pub const ALTITUDE_MAX: f32 = 10200.0;
pub const ALTITUDE_STEP: f32 = 40.0;

// Temperature Unit (Celsius).
// Byte representation is +1 [u] = +0.5 [C], 100 [u] = 0 [C].

pub const CELSIUS_MIN: f32 = -50.0;
pub const CELSIUS_MAX: f32 = 77.5;

#[allow(unused)]
pub fn celsius_clamp(x: f32) -> f32 {
    x.clamp(CELSIUS_MIN, CELSIUS_MAX)
}

#[allow(unused)]
pub fn celsius_to_byte(x: f32) -> u8 {
    (x * 2.0 + 100.0) as u8
}

#[allow(unused)]
pub fn celsius_from_byte(x: u8) -> f32 {
    (x - 100) as f32 / 2.0
}

pub fn celsius_to_fraction(x: f32) -> f32 {
    (x - CELSIUS_MIN) / 127.5
}

#[allow(unused)]
pub fn celsius_from_fraction(x: f32) -> f32 {
    x * (CELSIUS_MAX - CELSIUS_MIN) + CELSIUS_MIN
}

// Precipitation Unit (mm).
// Byte representation is 1 [u] = 20 [mm].

pub const PRECIP_MIN: f32 = 0.0;
pub const PRECIP_MAX: f32 = 5100.0;

pub fn precip_clamp(x: f32) -> f32 {
    x.clamp(PRECIP_MIN, PRECIP_MAX)
}

pub fn precip_to_byte(x: f32) -> u8 {
    (x / 20.0) as u8
}

#[allow(unused)]
pub fn precip_from_byte(x: u8) -> f32 {
    x as f32 * 20.0
}

pub fn precip_to_fraction(x: f32) -> f32 {
    x / PRECIP_MAX
}

#[allow(unused)]
pub fn precip_from_fraction(x: f32) -> f32 {
    x * PRECIP_MAX
}
