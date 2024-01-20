use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::{config::GeneratorConfig, map::ViewedMapLayer};

use super::{
    internal::{MainPanel, UiState},
    panel_climate::MainPanelClimate,
    panel_continents::MainPanelContinents,
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl MainPanel for MainPanelTopography {
    fn show(&self, _ui: &mut Ui, _config: &mut ResMut<GeneratorConfig>, _ui_state: &mut UiState) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Topography"
    }

    fn transition(&self, prev: bool, next: bool) -> Box<dyn MainPanel + Sync + Send> {
        if prev {
            Box::<MainPanelContinents>::default()
        } else if next {
            Box::<MainPanelClimate>::default()
        } else {
            Box::new(*self)
        }
    }

    fn get_map_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Topograpy
    }
}
