use bevy_egui::egui::Ui;

use atlas_lib::{
    ui::{button, sidebar::MakeUi, UiEditableEnum},
    update_enum,
};

use crate::{
    config::{
        CircleSamplerConfig, FbmConfig, InfluenceFbmConfig, InfluenceShape, SessionConfig, StripSamplerConfig,
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
        config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        add_section(ui, "Common", |ui| {
            let ui_results = config.continents.make_ui(ui);
            update_enum!(config.continents.algorithm, ui_results[0]);
            update_enum!(config.continents.influence_map_type, ui_results[2]);
        });
        make_algorithm_ui(ui, &mut config.continents.config);
        let generate_influence = make_influence_map_ui(ui, &mut config.continents.influence_map_type);
        if generate_influence {
            events.generate_request = Some(ViewedMapLayer::ContinentsInfluence);
        }
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

fn make_influence_map_ui(ui: &mut Ui, config: &mut InfluenceShape) -> bool {
    match config {
        InfluenceShape::None(_) => {
            return false;
        }
        InfluenceShape::FromImage(_) => {
            return false;
        }
        InfluenceShape::Circle(x) => make_influence_circle_ui(ui, x),
        InfluenceShape::Strip(x) => make_influence_strip_ui(ui, x),
        InfluenceShape::Fbm(x) => make_influence_fbm_ui(ui, x),
    }
    button(ui, "Generate Influence Map")
}

fn make_influence_circle_ui(ui: &mut Ui, config: &mut CircleSamplerConfig) {
    add_section(ui, "Influence Map Settings", |ui| {
        config.make_ui(ui);
    });
}

fn make_influence_strip_ui(ui: &mut Ui, config: &mut StripSamplerConfig) {
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
