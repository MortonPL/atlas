use bevy_egui::egui::Ui;

use crate::{
    config::GeneratorConfig,
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::{make_layer_save_load, UiState},
        panel::{MainPanel, MainPanelTransition, MainPanelGeneral, simple::MainPanelClimate},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl MainPanel for MainPanelTopography {
    fn show(
        &mut self,
        ui: &mut Ui,
        _config: &mut GeneratorConfig,
        ui_state: &mut UiState,
        _events: &mut EventStruct,
    ) {
        make_layer_save_load(ui, ui_state, ViewedMapLayer::Topography);
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