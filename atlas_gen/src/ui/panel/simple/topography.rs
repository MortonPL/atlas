use bevy_egui::egui::Ui;

use atlas_lib::{
    ui::{button, sidebar::MakeUi, UiEditableEnum},
    update_enum,
};

use crate::{
    config::{GeneratorConfig, GeneratorType, SimpleAlgorithm},
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::UiState,
        panel::{add_section, simple::MainPanelClimate, MainPanel, MainPanelGeneral, MainPanelTransition},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl MainPanel for MainPanelTopography {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut GeneratorConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        match &mut config.generator {
            GeneratorType::Simple(simple) => {
                let ui_results = simple.topography.make_ui(ui);
                update_enum!(simple.topography.algorithm, ui_results[0]);
                make_algorithm_ui(ui, &mut simple.topography.algorithm);
            }
            GeneratorType::Advanced(_) => unreachable!(),
        };
        if button(ui, "Generate Layer") {
            events.generate_request = Some(self.get_layer());
        }
    }

    fn get_heading(&self) -> &'static str {
        "Topography"
    }

    fn get_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Topography
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelGeneral>::default(),
            MainPanelTransition::Next => Box::<MainPanelClimate>::default(),
        }
    }
}

fn make_algorithm_ui(ui: &mut Ui, config: &mut SimpleAlgorithm) {
    match config {
        SimpleAlgorithm::Perlin(config) => add_section(ui, "Perlin Settings", |ui| {
            let ui_results = config.make_ui(ui);
            // TODO Same hack/problem as in crate::ui::panel::general
            if ui_results[0] == 1 {
                config.seed = rand::random();
            }
        }),
        SimpleAlgorithm::Simplex(config) => add_section(ui, "Simplex Settings", |ui| {
            let ui_results = config.make_ui(ui);
            // TODO Same hack/problem as in crate::ui::panel::general
            if ui_results[0] == 1 {
                config.seed = rand::random();
            }
        }),
        SimpleAlgorithm::DiamondSquare(_) => todo!(),
    };
}
