use bevy_egui::egui::Ui;

use atlas_lib::ui::sidebar::MakeUi;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::MapDataLayer,
    ui::{
        internal::UiState,
        panel::{MainPanelContinents, MainPanelTemperature, MainPanelTransition, SidebarPanel},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl SidebarPanel for MainPanelTopography {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        config.topography.make_ui(ui);
        self.button_influence(ui, events, &config.topography.influence_shape);
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Topography"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Topography
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelContinents>::default(),
            MainPanelTransition::Next => Box::<MainPanelTemperature>::default(),
        }
    }
}
