use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::{config::GeneratorConfig, map::ViewedMapLayer};

use super::{internal::{MainPanel, UiState}, panel_topography::MainPanelTopography};

#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl MainPanel for MainPanelClimate {
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn transition(&self, prev: bool, _next: bool) -> Box<dyn MainPanel + Sync + Send> {
        if prev {
            Box::new(MainPanelTopography::default())
        } else {
            Box::new(*self)
        }
    }

    fn get_map_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Climate
    }
}
