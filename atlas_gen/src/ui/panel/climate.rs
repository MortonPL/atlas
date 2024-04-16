use bevy_egui::egui::Ui;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::MapDataLayer,
    ui::{
        internal::UiState,
        panel::{MainPanelHumidity, MainPanelResources, MainPanelTransition, SidebarPanel},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl SidebarPanel for MainPanelClimate {
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
        "Climate"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Climate
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelHumidity>::default(),
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Next => Box::<MainPanelResources>::default(),
        }
    }
}
