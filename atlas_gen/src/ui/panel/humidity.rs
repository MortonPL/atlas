use bevy_egui::egui::Ui;

use atlas_lib::ui::sidebar::MakeUi;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::MapDataLayer,
    ui::{
        internal::UiState,
        panel::{MainPanelClimate, MainPanelTemperature, MainPanelTransition, SidebarPanel},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelHumidity;

impl SidebarPanel for MainPanelHumidity {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        config.humidity.make_ui(ui);
        self.button_influence(ui, events, &config.humidity.influence_shape);
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Humidity"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Humidity
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelTemperature>::default(),
            MainPanelTransition::Next => Box::<MainPanelClimate>::default(),
        }
    }
}
