use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::{
    config::GeneratorConfig,
    ui::{
        general::MainPanelGeneral,
        internal::{make_layer_save_load, ImageLayer, MainPanel, MainPanelTransition, UiState},
        simple::MainPanelClimate,
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl MainPanel for MainPanelTopography {
    fn show(&mut self, ui: &mut Ui, _config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        make_layer_save_load(ui, ui_state, ImageLayer::Topographical);
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Topography"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelGeneral>::default(),
            MainPanelTransition::Next => Box::<MainPanelClimate>::default(),
        }
    }
}
