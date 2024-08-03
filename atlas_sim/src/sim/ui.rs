use atlas_lib::{
    bevy_egui::{self, egui::Ui},
    ui::sidebar::*,
};

use crate::sim::{
    polity::*,
    time_to_string,
};

use super::polity::RES_TREASURE;

pub struct PolityUi {
    /// Ownership status.
    pub ownership: Ownership,
    /// Polity map color.
    pub color: [u8; 3],
    /// Number of cities.
    pub regions: u32,
    /// Total produced resources.
    pub resources: [f32; LEN_RES],
    /// Yearly accumulated resources.
    pub resources_acc: [f32; LEN_RES],
    /// Researched technology (major, minor level).
    pub tech: [[f32; 2]; LEN_SCI],
    /// Upkept traditions.
    pub traditions: [f32; LEN_TRAD],
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
}

impl PolityUi {
    pub fn make_ui_economy(&mut self, ui: &mut Ui) {
        ui.heading("Economy");
        ui.end_row();
        SidebarSlider::new(ui, "Accumulated Treasure", &mut self.resources_acc[RES_TREASURE]).show(None);
        SidebarSlider::new(ui, "Accumulated Civilian Industry", &mut self.resources_acc[RES_CIVILIAN]).show(None);
        for (x, label) in self.resources.iter_mut().zip(RES_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
        ui.heading("Population & Jobs");
        ui.end_row();
        SidebarSlider::new(ui, "Total Population", &mut self.population).show(None);
        SidebarStructSubsection::new(ui, "Sector Employment", &mut self.jobs).show(None);
    }

    pub fn make_ui_science(&mut self, ui: &mut Ui) {
        SidebarSlider::new(ui, "Accumulated Points", &mut self.resources_acc[RES_RESEARCH]).show(None);
        for (x, label) in self.tech.iter_mut().zip(SCI_LABELS) {
            SidebarSliderN::new(ui, label, x).show(None);
        }
    }

    pub fn make_ui_culture(&mut self, ui: &mut Ui) {
        ui.heading("Tradition");
        ui.end_row();
        SidebarSlider::new(ui, "Accumulated Points", &mut self.resources_acc[RES_CULTURE]).show(None);
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
        SidebarSlider::new(ui, "# of Regions", &mut self.regions).show(None);
        SidebarSlider::new(ui, "Total Population", &mut self.population).show(None);
        ui.heading("Policy");
        ui.end_row();
        for (x, label) in self.policies.iter_mut().zip(POL_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
    }
}