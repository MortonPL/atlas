use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::{
    config::GeneratorConfig,
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::{make_layer_save_load, MainPanel, MainPanelTransition, UiState},
        simple::{MainPanelResources, MainPanelTopography},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl MainPanel for MainPanelClimate {
    fn show(
        &mut self,
        ui: &mut Ui,
        _config: &mut ResMut<GeneratorConfig>,
        ui_state: &mut UiState,
        _events: &mut EventStruct,
    ) {
        make_layer_save_load(ui, ui_state, ViewedMapLayer::Climate);
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelTopography>::default(),
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Next => Box::<MainPanelResources>::default(),
        }
    }
}
