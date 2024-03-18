use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::{
    config::GeneratorConfig,
    ui::{
        internal::{make_layer_save_load, ImageLayer, MainPanel, MainPanelTransition, UiState},
        simple::MainPanelClimate,
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelResources;

impl MainPanel for MainPanelResources {
    fn show(&mut self, ui: &mut Ui, _config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        make_layer_save_load(ui, ui_state, ImageLayer::Climate);
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Resources"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelClimate>::default(),
            _ => Box::new(*self),
        }
    }
}
