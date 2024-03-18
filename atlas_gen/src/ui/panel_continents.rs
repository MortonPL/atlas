use atlas_lib::ui::MakeUi;
use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::{config::GeneratorConfig, map::ViewedMapLayer};

use super::{
    internal::{make_layer_save_load, ImageLayer, MainPanel, MainPanelTransition, UiState},
    panel_general::MainPanelGeneral,
    panel_topography::MainPanelTopography,
    utils::add_section,
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelContinents;

impl MainPanel for MainPanelContinents {
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        make_layer_save_load(ui, ui_state, ImageLayer::Continental);
        
        add_section(ui, "Continents Generator", |ui| {
            config.continents.make_ui(ui);
        });
    }

    fn get_heading(&self) -> &'static str {
        "Continents"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelGeneral>::default(),
            MainPanelTransition::Next => Box::<MainPanelTopography>::default(),
        }
    }

    fn get_map_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Continental
    }
}
