use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::{
    config::{FlatWorldModel, GeneratorConfig, GlobeWorldModel, WorldModel},
    map::ViewedMapLayer,
};

use atlas_lib::{ui::{MakeUi, UiConfigurableEnum}, update_enum};

use super::{
    internal::{MainPanel, UiState},
    panel_continents::MainPanelContinents,
    utils::add_section,
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral;

impl MainPanel for MainPanelGeneral {
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
        let mut ui_results = vec![];
        add_section(ui, "World", |ui| {
            ui_results = config.general.make_ui(ui);
        });
        // TODO: Bit hacky with raw indices, oh well
        if ui_results[2] == 1 {
            config.general.seed = rand::random();
        }
        
        update_enum!(config.general.world_model, ui_results[0]);
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
        
        update_enum!(config.general.generator, ui_results[3]);
        add_section(ui, format!("{} Generator Settings", config.general.generator.self_as_str()), |ui| {

        });
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn transition(&self, _prev: bool, next: bool) -> Box<dyn MainPanel + Sync + Send> {
        if next {
            Box::<MainPanelContinents>::default()
        } else {
            Box::new(*self)
        }
    }

    fn get_map_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::None
    }
}

fn create_general_flat_settings(ui: &mut Ui, ui_state: &mut UiState, config: &mut FlatWorldModel) {
    let old = config.world_size;
    config.make_ui(ui);
    ui_state.just_changed_dimensions = old != config.world_size;
}

fn create_general_globe_settings(
    _ui: &mut Ui,
    _ui_state: &mut UiState,
    _config: &mut GlobeWorldModel,
) {
    // TODO
}
