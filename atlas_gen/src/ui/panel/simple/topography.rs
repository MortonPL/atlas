use bevy_egui::egui::Ui;

use crate::{
    config::GeneratorConfig,
    event::EventStruct,
    ui::{
        internal::UiState,
        panel::{simple::MainPanelClimate, MainPanel, MainPanelGeneral, MainPanelTransition},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl MainPanel for MainPanelTopography {
    fn show(
        &mut self,
        _ui: &mut Ui,
        _config: &mut GeneratorConfig,
        _ui_state: &mut UiState,
        _events: &mut EventStruct,
    ) {
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
