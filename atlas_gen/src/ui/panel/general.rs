use bevy_egui::egui::Ui;

use atlas_lib::{
    ui::{
        sidebar::{MakeUi, SidebarControl, SidebarEnumDropdown},
        UiEditableEnum,
    },
    update_enum,
};

use crate::{
    config::{FlatWorldModel, GeneratorConfig, GeneratorType, GlobeWorldModel, WorldModel},
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{
        internal::UiState,
        panel::{
            add_section, advanced::MainPanelContinents, simple::MainPanelTopography, MainPanel,
            MainPanelTransition,
        },
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral {
    use_advanced: bool,
}

impl MainPanel for MainPanelGeneral {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut GeneratorConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        self.use_advanced = match &config.generator {
            GeneratorType::Simple(_) => false,
            GeneratorType::Advanced(_) => true,
        };

        let mut ui_results = vec![];
        add_section(ui, "World", |ui| {
            ui_results = config.general.make_ui(ui);
        });
        // TODO: Bit hacky with raw indices, oh well
        if ui_results[0] == 1 {
            config.general.seed = rand::random();
        }

        if config.general.world_model.self_as_index() != ui_results[2] {
            config.general.world_model = config.general.world_model.index_as_self(ui_results[2]);
            events.world_model_changed = Some(config.general.world_model.clone());
        }

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

        add_section(ui, "Generator", |ui| {
            let generator = SidebarEnumDropdown::new(ui, "Type", &mut config.generator).show(None);
            update_enum!(config.generator, generator);
        });
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn get_layer(&self) -> ViewedMapLayer {
        ViewedMapLayer::Preview
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
