use bevy_egui::egui::Ui;

use crate::{
    config::GeneratorConfig,
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::{make_layer_save_load, UiState},
        panel::{MainPanel, MainPanelTransition, simple::MainPanelClimate},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelResources;

impl MainPanel for MainPanelResources {
    fn show(
        &mut self,
        ui: &mut Ui,
        _config: &mut GeneratorConfig,
        ui_state: &mut UiState,
        _events: &mut EventStruct,
    ) {
        make_layer_save_load(ui, ui_state, ViewedMapLayer::Resource);
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Resources"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelClimate>::default(),
            _ => Box::new(*self),
        }
    }
}
