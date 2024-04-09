use bevy_egui::egui::Ui;

use atlas_lib::{
    ui::{button, sidebar::MakeUi, UiEditableEnum},
    update_enum,
};

use crate::{
    config::{
        FbmConfig, GeneratorConfig, GeneratorType, InfluenceCircleConfig, InfluenceFbmConfig,
        InfluenceMapType, InfluenceStripConfig,
    },
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::UiState,
        panel::{add_section, simple::MainPanelTopography, MainPanel, MainPanelGeneral, MainPanelTransition},
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
            GeneratorType::Simple(simple) => {
                add_section(ui, "Common", |ui| {
                    let ui_results = simple.continents.make_ui(ui);
                    update_enum!(simple.continents.algorithm, ui_results[0]);
                    update_enum!(simple.continents.influence_map_type, ui_results[2]);
                });
                make_algorithm_ui(ui, &mut simple.continents.config);
                let generate_influence = make_influence_map_ui(ui, &mut simple.continents.influence_map_type);
                if generate_influence {
                    events.generate_request = Some(ViewedMapLayer::ContinentsInfluence);
                }
            }
            GeneratorType::Advanced(_) => unreachable!(),
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
            MainPanelTransition::Next => Box::<MainPanelTopography>::default(),
        }
    }
}

fn make_algorithm_ui(ui: &mut Ui, config: &mut FbmConfig) {
    add_section(ui, "Algorithm Settings", |ui| {
        let ui_results = config.make_ui(ui);
        // TODO Same hack/problem as in crate::ui::panel::general
        if ui_results[0] == 1 {
            config.seed = rand::random();
        }
    });
}

fn make_influence_map_ui(ui: &mut Ui, config: &mut InfluenceMapType) -> bool {
    match config {
        InfluenceMapType::None(_) => {
            return false;
        }
        InfluenceMapType::FromImage(_) => {
            return false;
        }
        InfluenceMapType::Circle(x) => make_influence_circle_ui(ui, x),
        InfluenceMapType::Strip(x) => make_influence_strip_ui(ui, x),
        InfluenceMapType::Fbm(x) => make_influence_fbm_ui(ui, x),
    }
    button(ui, "Generate Influence Map")
}

fn make_influence_circle_ui(ui: &mut Ui, config: &mut InfluenceCircleConfig) {
    add_section(ui, "Influence Map Settings", |ui| {
        config.make_ui(ui);
    });
}

fn make_influence_strip_ui(ui: &mut Ui, config: &mut InfluenceStripConfig) {
    add_section(ui, "Influence Map Settings", |ui| {
        config.make_ui(ui);
    });
}

fn make_influence_fbm_ui(ui: &mut Ui, config: &mut InfluenceFbmConfig) {
    add_section(ui, "Influence Map Settings", |ui| {
        config.make_ui(ui);
        let config = &mut config.config;
        let ui_results = config.make_ui(ui);
        // TODO Same hack/problem as in crate::ui::panel::general
        if ui_results[0] == 1 {
            config.seed = rand::random();
        }
    });
}
