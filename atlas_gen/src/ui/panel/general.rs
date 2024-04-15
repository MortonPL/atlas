use bevy_egui::egui::Ui;

use atlas_lib::ui::{button, sidebar::MakeUi, UiEditableEnum};

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
        add_section(ui, "World", |ui| {
            let old_world_model = config.general.world_model.self_as_index();
            config.general.make_ui(ui);
            if config.general.world_model.self_as_index() != old_world_model {
                events.world_model_changed = Some(config.general.world_model.clone());
            }
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
