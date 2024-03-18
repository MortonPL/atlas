use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::{config::GeneratorConfig, map::ViewedMapLayer};

use super::{
    internal::{make_layer_save_load, ImageLayer, MainPanel, MainPanelTransition, UiState},
    panel_topography::MainPanelTopography,
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl MainPanel for MainPanelClimate {
    fn show(&self, ui: &mut Ui, _config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        make_layer_save_load(ui, ui_state, ImageLayer::Climate);
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelTopography>::default(),
            _ => Box::new(*self),
        }
    }

    fn get_map_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Climate
    }
}
