use bevy_egui::egui::Ui;

use atlas_lib::{ui::{button, sidebar::MakeUi, UiEditableEnum}, update_enum};

use crate::{
    config::{FlatWorldModel, GlobeWorldModel, SessionConfig, WorldModel},
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::UiState,
        panel::{add_section, simple::MainPanelContinents, MainPanel, MainPanelTransition},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral {}

impl MainPanel for MainPanelGeneral {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        let mut ui_results = vec![];
        add_section(ui, "World", |ui| {
            ui_results = config.general.make_ui(ui);
            // TODO: Bit hacky with raw indices, oh well
            if ui_results[0] == 1 {
                config.general.seed = rand::random();
            }
            if config.general.world_model.self_as_index() != ui_results[2] {
                config.general.world_model = config.general.world_model.index_as_self(ui_results[2]);
                events.world_model_changed = Some(config.general.world_model.clone());
            }
            update_enum!(config.general.topo_display, ui_results[3]);
        });

        add_section(
            ui,
            format!("{} World Settings", config.general.world_model.self_as_str()),
            |ui| {
                match &mut config.general.world_model {
                    WorldModel::Flat(x) => create_general_flat_settings(ui, events, x),
                    WorldModel::Globe(x) => create_general_globe_settings(ui, events, x),
                };
            },
        );

        if button(ui, "Generate Preview") {
            events.generate_request = Some(self.get_layer());
        }
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn get_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Preview
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Next => Box::<MainPanelContinents>::default(),
            _ => Box::new(*self),
        }
    }
}

fn create_general_flat_settings(ui: &mut Ui, events: &mut EventStruct, config: &mut FlatWorldModel) {
    let old = config.world_size;
    config.make_ui(ui);
    if old != config.world_size {
        events.world_model_changed = Some(WorldModel::Flat(config.clone()));
    }
}

fn create_general_globe_settings(_ui: &mut Ui, _events: &mut EventStruct, _config: &mut GlobeWorldModel) {
    // TODO
}
