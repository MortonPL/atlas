use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};

use crate::config::{GeneratorConfig, WorldModel};

use super::{internal::MainPanel, panel_topography::MainPanelTopography, utils::add_section};

#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral;

impl MainPanel for MainPanelGeneral {
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>) {
        let old_model_globe = match &config.general.world_model {
            WorldModel::Flat(_) => false,
            WorldModel::Globe(_) => true,
        };
        let mut model_globe = old_model_globe;
        add_section(ui, "Stuff", |ui| {
            ui.label("World Model").on_hover_text_at_pointer("TODO");
            egui::ComboBox::from_label("")
                .selected_text(config.general.world_model.str())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut model_globe, false, "Flat");
                    ui.selectable_value(&mut model_globe, true, "Globe")
                })
                .response
                .on_hover_text_at_pointer("TODO");
            ui.end_row();
        });
        if old_model_globe != model_globe {
            config.general.world_model = match model_globe {
                false => WorldModel::Flat(Default::default()),
                true => WorldModel::Globe(Default::default()),
            };
        }
        add_section(
            ui,
            format!("{} World Settings", config.general.world_model.str()),
            |ui| {
                match model_globe {
                    false => create_general_flat_settings(),
                    true => create_general_globe_settings(),
                };
            },
        );
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn transition(&self, _prev: bool, next: bool) -> Box<dyn MainPanel + Sync + Send> {
        if next {
            Box::new(MainPanelTopography::default())
        } else {
            Box::new(*self)
        }
    }
}

fn create_general_flat_settings() {}

fn create_general_globe_settings() {}
