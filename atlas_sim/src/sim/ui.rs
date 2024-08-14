use atlas_lib::{
    bevy::prelude::*,
    bevy_egui::egui::{CollapsingHeader, RichText, Ui},
    ui::sidebar::*,
};

use crate::sim::{polity::*, time_to_string, time_to_string_plus};

use super::conflict::ConflictMember;

pub struct PolityUi {
    /// Polity.
    pub this: Entity,
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
    pub traditions: [[f32; 2]; LEN_TRAD],
    /// Govt policies.
    pub policies: [f32; LEN_POL],
    /// Accumulated heritage.
    pub heritage: [f32; LEN_TRAD],
    /// Created great works.
    pub great_works: Vec<GreatWork>,
    /// Created great people.
    pub great_people: Vec<GreatPerson>,
    /// Total polity population.
    pub population: f32,
    /// List of pop job groups.
    pub jobs: JobStruct,
    /// Average stability of all regions/pops.
    pub avg_stability: f32,
    /// Average health of all regions/pops.
    pub avg_health: f32,
    /// Tributes to pay.
    pub tributes: Vec<Tribute>,
    /// Neighbours and relations,
    pub neighbours: Vec<(Entity, f32)>,
    /// Next policy change date.
    pub next_policy: u32,
    /// Active conflicts.
    pub conflicts: Vec<ConflictUi>,
}

impl PolityUi {
    pub fn make_ui_economy(&mut self, ui: &mut Ui) {
        ui.heading("Economy");
        ui.end_row();
        SidebarSlider::new(
            ui,
            "Accumulated Civilian Industry",
            &mut self.resources_acc[RES_CIVILIAN],
        )
        .show(None);
        for (x, label) in self.resources.iter_mut().zip(RES_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
        ui.heading("Population & Jobs");
        ui.end_row();
        SidebarSlider::new(
            ui,
            "Total Population",
            &mut (self.population + self.jobs.military),
        )
        .show(None);
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
            SidebarSliderN::new(ui, label, x).show(None);
        }
        ui.heading("Heritage");
        ui.end_row();
        for (x, label) in self.heritage.iter_mut().zip(TRAD_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
        CollapsingHeader::new(RichText::new("Great Works").heading())
            .default_open(true)
            .show(ui, |ui| {
                for x in self.great_works.iter() {
                    ui.label(TRAD_LABELS[x.tradition as usize]);
                    ui.label(time_to_string(x.time));
                    ui.end_row();
                }
            });
        ui.end_row();
        CollapsingHeader::new(RichText::new("Great People").heading())
            .default_open(true)
            .show(ui, |ui| {
                for x in self.great_people.iter() {
                    ui.label(GRT_LABELS[x.tradition as usize]);
                    let active = if x.active { "(Active)" } else { "(Retired)" };
                    ui.label(time_to_string_plus(x.time, active));
                    ui.end_row();
                }
            });
        ui.end_row();
    }

    pub fn make_ui_combat(&mut self, ui: &mut Ui) {
        for x in self.conflicts.iter_mut() {
            x.make_ui(ui);
        }
    }
}

impl MakeUi for PolityUi {
    fn make_ui(&mut self, ui: &mut Ui) {
        SidebarEntityLink::new(ui, "Id", &mut self.this).show(None);
        SidebarColor::new(ui, "Color", &mut self.color).show(None);
        SidebarSlider::new(ui, "# of Regions", &mut self.regions).show(None);
        SidebarSlider::new(ui, "Civilian Population", &mut self.population).show(None);
        SidebarSlider::new(ui, "Average Stability", &mut self.avg_stability).show(None);
        SidebarSlider::new(ui, "Average Healthcare", &mut self.avg_health).show(None);
        ui.heading("Policy");
        ui.end_row();
        ui.label("Next Policy Change");
        ui.label(time_to_string(self.next_policy));
        ui.end_row();
        for (x, label) in self.policies.iter_mut().zip(POL_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
        CollapsingHeader::new(RichText::new("Neighbours").heading())
            .default_open(true)
            .show(ui, |ui| {
                for (entity, relation) in self.neighbours.iter_mut() {
                    SidebarSlider::new(ui, format!("{:?}", entity), relation).show(None);
                }
            });
        ui.end_row();
        CollapsingHeader::new(RichText::new("Tributes").heading())
            .default_open(true)
            .show(ui, |ui| {
                for x in self.tributes.iter_mut() {
                    x.make_ui(ui);
                }
            });
        ui.end_row();
    }
}

impl MakeUi for Tribute {
    fn make_ui(&mut self, ui: &mut Ui) {
        SidebarEntityLink::new(ui, "Receiver", &mut self.receiver).show(None);
        SidebarSlider::new(ui, "Economy Fraction", &mut self.fraction).show(None);
        SidebarSlider::new(ui, "Payments Left", &mut self.time_left).show(None);
    }
}

#[derive(Clone)]
pub struct RegionUi {
    /// Region population.
    pub population: f32,
    /// Map of available deposits.
    pub deposits: Vec<(String, f32)>,
    /// Number of tiles in the region.
    pub tiles: u32,
    /// Land claim points.
    pub land_claim: f32,
    /// City fund points.
    pub city_fund: f32,
    /// Development level.
    pub development: f32,
    /// Level of special structures.
    pub structures: [f32; LEN_STR],
    /// Stability level.
    pub stability: f32,
    /// Healthcare level.
    pub healthcare: f32,
    /// Security force power.
    pub security: f32,
    /// Public health power.
    pub health: f32,
}

impl MakeUi for RegionUi {
    fn make_ui(&mut self, ui: &mut Ui) {
        SidebarSlider::new(ui, "Population", &mut self.population).show(None);
        SidebarSlider::new(ui, "Public Security Power", &mut self.security).show(None);
        SidebarSlider::new(ui, "Public Health Power", &mut self.health).show(None);
        SidebarSlider::new(ui, "Avg. Stability", &mut self.stability).show(None);
        SidebarSlider::new(ui, "Avg. Healthcare", &mut self.healthcare).show(None);
        SidebarSlider::new(ui, "# of Tiles", &mut self.tiles).show(None);
        ui.heading("Expansion & Development");
        ui.end_row();
        SidebarSlider::new(ui, "Land Claim Points", &mut self.land_claim).show(None);
        SidebarSlider::new(ui, "New Region Points", &mut self.city_fund).show(None);
        SidebarSlider::new(ui, "Development Level", &mut self.development).show(None);
        ui.heading("Structures");
        ui.end_row();
        for (x, label) in self.structures.iter_mut().zip(STR_LABELS) {
            SidebarSlider::new(ui, label, x).show(None);
        }
        ui.heading("Deposits");
        ui.end_row();
        for (k, v) in &mut self.deposits {
            SidebarSlider::new(ui, k.clone(), v).show(None);
        }
    }
}

#[derive(Clone)]
pub struct ConflictUi {
    pub start_date: u32,
    pub primary_attacker: Entity,
    pub primary_defender: Entity,
    pub attackers: Vec<ConflictMember>,
    pub defenders: Vec<ConflictMember>,
}

impl MakeUi for ConflictUi {
    fn make_ui(&mut self, ui: &mut Ui) {
        ui.label("Start Date");
        ui.label(time_to_string(self.start_date));
        ui.end_row();
        SidebarEntityLink::new(ui, "Primary Attacker", &mut self.primary_attacker).show(None);
        SidebarEntityLink::new(ui, "Primary Defender", &mut self.primary_defender).show(None);
        CollapsingHeader::new(RichText::new("Attackers").heading())
            .default_open(true)
            .show(ui, |ui| {
                for x in self.attackers.iter_mut() {
                    x.make_ui(ui);
                }
            });
        ui.end_row();
        CollapsingHeader::new(RichText::new("Defenders").heading())
            .default_open(true)
            .show(ui, |ui| {
                for x in self.defenders.iter_mut() {
                    x.make_ui(ui);
                }
            });
        ui.end_row();
    }
}
