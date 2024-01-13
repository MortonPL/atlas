use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::config::GeneratorConfig;

use super::{
    internal::MainPanel, panel_climate::MainPanelClimate, panel_general::MainPanelGeneral,
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl MainPanel for MainPanelTopography {
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Topography"
    }

    fn transition(&self, prev: bool, next: bool) -> Box<dyn MainPanel + Sync + Send> {
        if prev {
            Box::new(MainPanelGeneral::default())
        } else if next {
            Box::new(MainPanelClimate::default())
        } else {
            Box::new(*self)
        }
    }
}
