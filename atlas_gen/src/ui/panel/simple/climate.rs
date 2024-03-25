use bevy_egui::egui::Ui;

use crate::{
    config::GeneratorConfig,
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::UiState,
        panel::{
            simple::{MainPanelResources, MainPanelTopography},
            MainPanel, MainPanelTransition,
        },
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl MainPanel for MainPanelClimate {
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
        "Climate"
    }

    fn get_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Climate
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelTopography>::default(),
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Next => Box::<MainPanelResources>::default(),
        }
    }
}
