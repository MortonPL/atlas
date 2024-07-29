use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui::{self, egui::Ui},
    ui::sidebar::*,
};

use crate::sim::{
    polity::{
        GreatWork, JobStruct, Ownership, LEN_POL, LEN_RES, LEN_STR, LEN_TECH, LEN_TRAD, POL_LABELS,
        RES_LABELS, STR_LABELS, TECH_LABELS, TRAD_LABELS,
    },
    time_to_string,
};

pub struct PolityUi {
    /// Ownership status.
    pub ownership: Ownership,
    /// Polity map color.
    pub color: [u8; 3],
    /// The desire to claim border tiles.
    pub land_claim_points: f32,
    /// Number of cities.
    pub cities: u32,
    /// Map of available deposits.
    pub deposits: Vec<(String, f32)>,
    /// Total produced resources.
    pub resources: [f32; LEN_RES],
    /// Researched technology.
    pub tech: [f32; LEN_TECH],
    /// Tech points accumulated this year.
    pub tech_acc: f32,
    /// Upkept traditions.
    pub traditions: [f32; LEN_TRAD],
    /// Tradition points accumulated this year.
    pub tradition_acc: f32,
    /// Govt policies.
    pub policies: [f32; LEN_POL],
    /// Accumulated heritage.
    pub heritage: [f32; LEN_TRAD],
    /// Created great works.
    pub great_works: Vec<GreatWork>,
    /// Total polity population.
    pub population: f32,
    /// List of pop job groups.
    pub jobs: JobStruct,
    /// Accumulated polity currency.
    pub treasure_acc: f32,
    /// Construction points accumulated this year.
    pub const_acc: f32,
}

impl PolityUi {
    pub fn make_ui_economy(&mut self, ui: &mut Ui) {
        ui.heading("Economy");
        ui.end_row();
        SidebarSlider::new(ui, "Accumulated Treasure", &mut self.treasure_acc).show(None);
        SidebarSlider::new(ui, "Accumulated Construction", &mut self.const_acc).show(None);
        for (x, label) in self.resources.iter_mut().zip(RES_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
        ui.heading("Population & Jobs");
        ui.end_row();
        SidebarSlider::new(ui, "Population", &mut self.population).show(None);
        SidebarStructSubsection::new(ui, "Sector Employment", &mut self.jobs).show(None);
        ui.heading("Deposits");
        ui.end_row();
        for (k, v) in &mut self.deposits {
            SidebarSlider::new(ui, k.clone(), v).show(None);
        }
    }

    pub fn make_ui_science(&mut self, ui: &mut Ui) {
        SidebarSlider::new(ui, "Accumulated Points", &mut self.tech_acc).show(None);
        for (x, label) in self.tech.iter_mut().zip(TECH_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
    }

    pub fn make_ui_culture(&mut self, ui: &mut Ui) {
        ui.heading("Tradition");
        ui.end_row();
        SidebarSlider::new(ui, "Accumulated Points", &mut self.tradition_acc).show(None);
        for (x, label) in self.traditions.iter_mut().zip(TRAD_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
        ui.heading("Heritage");
        ui.end_row();
        for (x, label) in self.heritage.iter_mut().zip(TRAD_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
        ui.heading("Great Works");
        ui.end_row();
        for x in self.great_works.iter() {
            ui.label(TRAD_LABELS[x.tradition as usize]);
            ui.label(time_to_string(x.time));
            ui.end_row();
        }
    }
}

impl MakeUi for PolityUi {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarEnumDropdown::new(ui, "Ownership", &mut self.ownership).show(None);
        SidebarColor::new(ui, "Color", &mut self.color).show(None);
        SidebarSlider::new(ui, "Land Claim Points", &mut self.land_claim_points).show(None);
        SidebarSlider::new(ui, "# of Cities", &mut self.cities).show(None);
        ui.heading("Policy");
        ui.end_row();
        for (x, label) in self.policies.iter_mut().zip(POL_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
    }
}

#[derive(Component)]
pub struct CityUi {
    /// Urbanization level.
    pub level: f32,
    /// Level of special structures.
    pub structures: [f32; LEN_STR],
}

impl MakeUi for CityUi {
    fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        SidebarSlider::new(ui, "City Level", &mut self.level).show(None);
        ui.heading("Structures");
        ui.end_row();
        for (x, label) in self.structures.iter_mut().zip(STR_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
    }
}
