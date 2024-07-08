use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*, utils::HashMap},
    bevy_egui,
    serde_derive::{Deserialize, Serialize},
    ui::sidebar::*,
    MakeUi,
};

/// A resource chunk.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct ResourceChunk {
    pub tile_count: u16,
    pub harvest_resources: HashMap<u8, f32>,
    pub extract_resources: HashMap<u8, f32>,
}

impl MakeUi for ResourceChunk {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "Land Tile Count", &mut self.tile_count).show(None);
        ui.label("Harvestable Resources");
        ui.end_row();
        for (i, val) in self.harvest_resources.iter_mut() {
            SidebarSlider::new(ui, format!("Resource type {}", i).as_str(), val).show(None);
        }
        ui.label("Extractable Resources");
        ui.end_row();
        for (i, val) in self.extract_resources.iter_mut() {
            SidebarSlider::new(ui, format!("Resource type {}", i).as_str(), val).show(None);
        }
    }
}

/// A resource type.
#[derive(Debug, Default, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct ResourceType {
    pub name: String,
    #[name("Harvestable")]
    #[control(SidebarCheckbox)]
    pub harvestable: bool,
    #[name("Extractable")]
    #[control(SidebarCheckbox)]
    pub extractable: bool,
}

/// Create a list of default resource types for general use.
pub fn make_default_resources() -> Vec<ResourceType> {
    vec![
        ResourceType {
            name: "Food".into(),
            harvestable: true,
            extractable: false,
        }
    ]
}
