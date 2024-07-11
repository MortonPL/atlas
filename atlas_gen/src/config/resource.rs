use std::collections::BTreeMap;

use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui,
    serde_derive::{Deserialize, Serialize},
    serde_with::{serde_as, DisplayFromStr},
    ui::sidebar::*,
    MakeUi,
};

/// A resource chunk.
#[serde_as(crate = "atlas_lib::serde_with")]
#[derive(Clone, Debug, Default, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct ResourceChunk {
    pub tile_count: u16,
    #[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
    pub resources: BTreeMap<u32, f32>,
}

impl MakeUi for ResourceChunk {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Land Tile Count", &mut self.tile_count).show(None);
        ui.label("Resources");
        ui.end_row();
        for (i, val) in self.resources.iter_mut() {
            SidebarSlider::new(ui, format!("Resource type {}", i).as_str(), val).show(None);
        }
    }
}

/// A resource type.
#[derive(Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct ResourceType {
    #[name("Name")]
    #[control(SidebarTextbox)]
    pub name: String,
    #[name("Can Be Traded")]
    #[control(SidebarCheckbox)]
    pub traded: bool,
    #[name("Random Deposits")]
    #[control(SidebarCheckbox)]
    pub random_deposits: bool,
    #[name("Random Deposit Chance")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1.0))]
    pub random_chance: f32,
    #[name("Random Deposit Averge Yield")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub random_average: f32,
    #[name("Random Deposit Yield Deviation")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub random_deviation: f32,
    #[name("Trade Value")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub trade_value: f32,
    #[name("Prestige Value")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=1000.0))]
    pub prestige_value: f32,
}

/// Create a list of default resource types for general use.
pub fn make_default_resources() -> Vec<ResourceType> {
    vec![
        // 0
        ResourceType {
            name: "Arable Land".into(),
            traded: false,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 1
        ResourceType {
            name: "Grazing Land".into(),
            traded: false,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 2
        ResourceType {
            name: "Hunting Land".into(),
            traded: false,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 3
        ResourceType {
            name: "Trees".into(),
            traded: false,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 4
        ResourceType {
            name: "Fishing Water".into(),
            traded: false,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 5
        ResourceType {
            name: "Stone & Clay Deposit".into(),
            traded: false,
            random_deposits: true,
            random_chance: 1.0,
            random_average: 1.0,
            random_deviation: 0.3,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 6
        ResourceType {
            name: "Metal Deposit".into(),
            traded: false,
            random_deposits: true,
            random_chance: 0.7,
            random_average: 1.2,
            random_deviation: 0.5,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 7
        ResourceType {
            name: "Coal Deposit".into(),
            traded: false,
            random_deposits: true,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 8
        ResourceType {
            name: "Precious Metal Deposit".into(),
            traded: false,
            random_deposits: true,
            random_chance: 0.3,
            random_average: 1.1,
            random_deviation: 0.7,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 9
        ResourceType {
            name: "Food".into(),
            traded: true,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 10.0,
            prestige_value: 1.0,
        },
        // 10
        ResourceType {
            name: "Cloth & Leather".into(),
            traded: true,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 15.0,
            prestige_value: 2.0,
        },
        // 11
        ResourceType {
            name: "Crafted Goods".into(),
            traded: true,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 30.0,
            prestige_value: 3.0,
        },
        // 12
        ResourceType {
            name: "Lumber".into(),
            traded: true,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 5.0,
            prestige_value: 1.0,
        },
        // 13
        ResourceType {
            name: "Stone & Clay".into(),
            traded: true,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 1.0,
            prestige_value: 1.0,
        },
        // 14
        ResourceType {
            name: "Metal".into(),
            traded: true,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 30.0,
            prestige_value: 2.0,
        },
        // 15
        ResourceType {
            name: "Precious Metal".into(),
            traded: true,
            random_deposits: false,
            random_chance: 0.0,
            random_average: 0.0,
            random_deviation: 0.0,
            trade_value: 50.0,
            prestige_value: 7.0,
        },
    ]
}
