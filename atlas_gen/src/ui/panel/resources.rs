use bevy_egui::egui::Ui;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::MapDataLayer,
    ui::{
        internal::UiState,
        panel::{MainPanelClimate, MainPanelTransition, SidebarPanel},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelResources;

impl SidebarPanel for MainPanelResources {
    fn show(
        &mut self,
        ui: &mut Ui,
        _config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        // TODO
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Resources"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Resource
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelClimate>::default(),
            _ => Box::new(*self),
        }
    }
}
