use bevy::prelude::*;
use bevy_egui::egui::Ui;

use atlas_lib::{
    ui::{MakeUi, UiConfigurableEnum, UiControl, UiEnumDropdown},
    update_enum,
};

use crate::{
    config::{FlatWorldModel, GeneratorConfig, GeneratorType, GlobeWorldModel, WorldModel},
    ui::{
        advanced::MainPanelContinents,
        internal::{MainPanel, MainPanelTransition, UiState},
        simple::MainPanelTopography,
        utils::add_section,
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral {
    use_advanced: bool,
}

impl MainPanel for MainPanelGeneral {
    fn show(&mut self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        self.use_advanced = match &config.generator {
            GeneratorType::Simple(_) => false,
            GeneratorType::Advanced(_) => true,
        };

        let mut ui_results = vec![];
        add_section(ui, "World", |ui| {
            ui_results = config.general.make_ui(ui);
        });
        // TODO: Bit hacky with raw indices, oh well
        if ui_results[2] == 1 {
            config.general.seed = rand::random();
        }

        if config.general.world_model.self_as_index() != ui_results[0] {
            config.general.world_model = config.general.world_model.index_as_self(ui_results[0]);
            ui_state.just_changed_model = true;
        }

        add_section(
            ui,
            format!(
                "{} World Settings",
                config.general.world_model.self_as_str()
            ),
            |ui| {
                match &mut config.general.world_model {
                    WorldModel::Flat(x) => create_general_flat_settings(ui, ui_state, x),
                    WorldModel::Globe(x) => create_general_globe_settings(ui, ui_state, x),
                };
            },
        );

        add_section(ui, "Generator", |ui| {
            let generator = UiEnumDropdown::new(ui, "Type", &mut config.generator).show(None);
            update_enum!(config.generator, generator);
        });
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Next => match self.use_advanced {
                false => Box::<MainPanelTopography>::default(),
                true => Box::<MainPanelContinents>::default(),
            },
            _ => Box::new(*self),
        }
    }
}

fn create_general_flat_settings(ui: &mut Ui, ui_state: &mut UiState, config: &mut FlatWorldModel) {
    let old = config.world_size;
    config.make_ui(ui);
    ui_state.just_changed_size = old != config.world_size;
}

fn create_general_globe_settings(
    _ui: &mut Ui,
    _ui_state: &mut UiState,
    _config: &mut GlobeWorldModel,
) {
    // TODO
}
