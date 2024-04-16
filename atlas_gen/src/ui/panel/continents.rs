use bevy_egui::egui::Ui;

use atlas_lib::ui::sidebar::MakeUi;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::MapDataLayer,
    ui::{
        internal::UiState,
        panel::{MainPanelGeneral, MainPanelTopography, MainPanelTransition, SidebarPanel},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelContinents;

impl SidebarPanel for MainPanelContinents {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        config.continents.make_ui(ui);
        self.button_influence(ui, events, &config.continents.influence_shape);
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Continents"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Continents
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelGeneral>::default(),
            MainPanelTransition::Next => Box::<MainPanelTopography>::default(),
        }
    }
}
