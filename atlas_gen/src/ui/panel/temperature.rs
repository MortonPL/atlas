use bevy_egui::egui::Ui;

use atlas_lib::ui::sidebar::MakeUi;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::MapDataLayer,
    ui::{
        internal::UiState,
        panel::{MainPanelHumidity, MainPanelTopography, MainPanelTransition, SidebarPanel},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTemperature;

impl SidebarPanel for MainPanelTemperature {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        config.temperature.make_ui(ui);
        self.button_influence(ui, events, &config.temperature.influence_shape);
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Temperature"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Temperature
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelTopography>::default(),
            MainPanelTransition::Next => Box::<MainPanelHumidity>::default(),
        }
    }
}
