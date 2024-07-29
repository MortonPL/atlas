use std::collections::BTreeMap;

use crate::{
    bevy::prelude::*,
    bevy_egui,
    serde_derive::{Deserialize, Serialize},
    serde_with::{serde_as, DisplayFromStr},
    ui::sidebar::*,
    MakeUi,
};

/// Config for the resource deposits generation.
#[derive(Debug, Clone, Deserialize, Resource, Serialize, MakeUi)]
pub struct DepositsConfig {
    #[name("Chunk Size")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1..=255))]
    pub chunk_size: u8,
    #[name("Deposit Types")]
    #[control(SidebarStructList)]
    pub types: Vec<DepositType>,
    #[name("Deposit Chunks")]
    #[control(SidebarStructList)]
    pub chunks: Vec<DepositChunk>,
}

impl Default for DepositsConfig {
    fn default() -> Self {
        Self {
            chunk_size: 18,
            chunks: Default::default(),
            types: make_default_deposits(),
        }
    }
}

/// A resource chunk.
#[serde_as(crate = "crate::serde_with")]
#[derive(Clone, Debug, Default, Deserialize, Resource, Serialize)]
pub struct DepositChunk {
    pub tile_count: u16,
    #[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
    pub deposits: BTreeMap<u32, f32>,
}

impl MakeUi for DepositChunk {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Land Tile Count", &mut self.tile_count).show(None);
        ui.label("Deposits");
        ui.end_row();
        for (i, val) in self.deposits.iter_mut() {
            SidebarSlider::new(ui, format!("Deposit type {}", i).as_str(), val).show(None);
        }
    }
}

/// A resource type.
#[derive(Debug, Clone, Default, Deserialize, Resource, Serialize, MakeUi)]
pub struct DepositType {
    #[name("Name")]
    #[control(SidebarTextbox)]
    pub name: String,
    #[name("Random Deposit Chance")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub gen_chance: f32,
    #[name("Deposit Averge Size")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub gen_average: f32,
    #[name("Deposit Size Deviation")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub gen_deviation: f32,
    #[name("Supply Points")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub supply: f32,
    #[name("Industry Points")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub industry: f32,
    #[name("Wealth Points")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub wealth: f32,
}

/// Create a list of default deposit types for general use.
pub fn make_default_deposits() -> Vec<DepositType> {
    vec![
        // 0
        DepositType {
            name: "Arable Land".into(),
            gen_chance: 0.0,
            gen_average: 1.0,
            gen_deviation: 0.5,
            supply: 0.8,
            industry: 0.2,
            wealth: 0.0,
        },
        // 1
        DepositType {
            name: "Luxury Arable Land".into(),
            gen_chance: 0.0,
            gen_average: 1.0,
            gen_deviation: 0.5,
            supply: 0.0,
            industry: 0.0,
            wealth: 1.0,
        },
        // 2
        DepositType {
            name: "Grazing Land".into(),
            gen_chance: 0.0,
            gen_average: 1.0,
            gen_deviation: 0.5,
            supply: 0.7,
            industry: 0.2,
            wealth: 0.1,
        },
        // 3
        DepositType {
            name: "Wild Game".into(),
            gen_chance: 0.0,
            gen_average: 1.0,
            gen_deviation: 0.5,
            supply: 0.7,
            industry: 0.1,
            wealth: 0.2,
        },
        // 4
        DepositType {
            name: "Luxury Wild Game".into(),
            gen_chance: 0.0,
            gen_average: 1.0,
            gen_deviation: 0.5,
            supply: 0.3,
            industry: 0.0,
            wealth: 0.7,
        },
        // 5
        DepositType {
            name: "Trees".into(),
            gen_chance: 0.0,
            gen_average: 1.0,
            gen_deviation: 0.5,
            supply: 0.1,
            industry: 0.8,
            wealth: 0.1,
        },
        // 6
        DepositType {
            name: "Fishing Water".into(),
            gen_chance: 0.0,
            gen_average: 0.5,
            gen_deviation: 1.0,
            supply: 0.7,
            industry: 0.1,
            wealth: 0.2,
        },
        // 7
        DepositType {
            name: "Stone & Clay Deposit".into(),
            gen_chance: 0.5,
            gen_average: 0.5,
            gen_deviation: 0.4,
            supply: 0.0,
            industry: 0.8,
            wealth: 0.2,
        },
        // 8
        DepositType {
            name: "Metal Deposit".into(),
            gen_chance: 0.25,
            gen_average: 0.5,
            gen_deviation: 0.4,
            supply: 0.0,
            industry: 0.9,
            wealth: 0.1,
        },
        // 9
        DepositType {
            name: "Coal Deposit".into(),
            gen_chance: 0.35,
            gen_average: 0.5,
            gen_deviation: 0.4,
            supply: 0.1,
            industry: 0.9,
            wealth: 0.0,
        },
        // 10
        DepositType {
            name: "Precious Metal Deposit".into(),
            gen_chance: 0.1,
            gen_average: 0.2,
            gen_deviation: 0.2,
            supply: 0.0,
            industry: 0.0,
            wealth: 1.0,
        },
    ]
}
