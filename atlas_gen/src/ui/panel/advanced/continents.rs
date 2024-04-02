use bevy_egui::egui::Ui;

use atlas_lib::ui::{button, sidebar::MakeUi};

use crate::{
    config::{GeneratorConfig, GeneratorType},
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::UiState,
        panel::{MainPanel, MainPanelGeneral, MainPanelTransition},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelContinents;

impl MainPanel for MainPanelContinents {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut GeneratorConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        match &mut config.generator {
            GeneratorType::Advanced(advanced) => {
                advanced.continents.make_ui(ui);
            },
            _ => unreachable!(),
        };
        if button(ui, "Generate Layer") {
            events.generate_request = Some(self.get_layer());
        }
    }

    fn get_heading(&self) -> &'static str {
        "Continents"
    }

    fn get_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Continents
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelGeneral>::default(),
            MainPanelTransition::Next => Box::new(*self), //Box::<MainPanelTopography>::default(), // TODO
        }
    }
}
