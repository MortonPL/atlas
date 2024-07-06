use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui,
    serde_derive::{Deserialize, Serialize},
    ui::{sidebar::SidebarControl, UiEditableEnum},
    MakeUi, UiEditableEnum,
};

/// Config for general world settings and preview.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct RulesConfig {
    #[name("Tile Resolution [km]")]
    #[control(SidebarSlider)]
    #[add(clamp_range(1.0..=100.0))]
    pub tile_resolution: f32,
    #[name("Starting Land Claim Points")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10000.0))]
    pub starting_land_claim_points: f32,
    #[name("Land Claim Cost Per Tile")]
    #[control(SidebarSlider)]
    #[add(clamp_range(0.0..=10000.0))]
    pub land_claim_cost: f32,
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            tile_resolution: 10.0,
            starting_land_claim_points: 1000.0,
            land_claim_cost: 100.0,
        }
    }
}
