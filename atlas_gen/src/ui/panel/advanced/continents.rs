use bevy_egui::egui::Ui;

use atlas_lib::ui::sidebar::MakeUi;

use crate::{
    config::{GeneratorConfig, GeneratorType},
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::{make_layer_save_load, UiState},
        panel::{MainPanelGeneral, MainPanel, MainPanelTransition, add_section},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelContinents;

impl MainPanel for MainPanelContinents {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut GeneratorConfig,
        ui_state: &mut UiState,
        _events: &mut EventStruct,
    ) {
        make_layer_save_load(ui, ui_state, ViewedMapLayer::Continents);

        add_section(ui, "Continents Generator", |ui| {
            match &mut config.generator {
                GeneratorType::Advanced(advanced) => advanced.continents.make_ui(ui),
                _ => unreachable!(),
            };
        });
    }

    fn get_heading(&self) -> &'static str {
        "Continents"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelGeneral>::default(),
            MainPanelTransition::Next => Box::new(*self), //Box::<MainPanelTopography>::default(), // TODO
        }
    }
}