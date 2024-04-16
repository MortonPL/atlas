use bevy_egui::egui::Ui;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::UiState,
        panel::{MainPanel, MainPanelClimate, MainPanelTransition},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelResources;

impl MainPanel for MainPanelResources {
    fn show(
        &mut self,
        _ui: &mut Ui,
        _config: &mut SessionConfig,
        _ui_state: &mut UiState,
        _events: &mut EventStruct,
    ) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Resources"
    }

    fn get_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Resource
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelClimate>::default(),
            _ => Box::new(*self),
        }
    }
}
