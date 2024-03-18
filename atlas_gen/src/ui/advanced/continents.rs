use bevy::prelude::*;
use bevy_egui::egui::Ui;

use atlas_lib::ui::MakeUi;

use crate::{
    config::{GeneratorConfig, GeneratorType},
    ui::{
        general::MainPanelGeneral,
        internal::{make_layer_save_load, ImageLayer, MainPanel, MainPanelTransition, UiState},
        utils::add_section,
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelContinents;

impl MainPanel for MainPanelContinents {
    fn show(&mut self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        make_layer_save_load(ui, ui_state, ImageLayer::Continental);

        add_section(ui, "Continents Generator", |ui| {
            match &mut config.generator {
                GeneratorType::Advanced(advanced) => advanced.continents.make_ui(ui),
                _ => default(),
            };
        });
    }

    fn get_heading(&self) -> &'static str {
        "Continents"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelGeneral>::default(),
            MainPanelTransition::Next => Box::new(*self), //Box::<MainPanelTopography>::default(), // TODO
        }
    }
}
