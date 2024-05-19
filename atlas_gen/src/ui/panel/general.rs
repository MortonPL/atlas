use bevy_egui::egui::Ui;

use atlas_lib::ui::{button, sidebar::MakeUi, UiEditableEnum};

use crate::{
    config::{AtlasGenConfig, WorldModel},
    event::EventStruct,
    map::MapDataLayer,
    ui::{
        internal::UiState,
        panel::{MainPanelContinents, MainPanelTransition, SidebarPanel},
    },
};

/// Panel with general world gen and preview settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral {}

impl SidebarPanel for MainPanelGeneral {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        let old_world_model = config.general.world_model.self_as_index();
        let old = match &config.general.world_model {
            WorldModel::Flat(x) => x.world_size,
            WorldModel::Globe(_) => [0, 0],
        };

        config.general.make_ui(ui);

        if config.general.world_model.self_as_index() != old_world_model {
            events.world_model_changed = Some(config.general.world_model.clone());
        }

        match &config.general.world_model {
            WorldModel::Flat(x) => {
                if old != x.world_size {
                    events.world_model_changed = Some(WorldModel::Flat(x.clone()));
                }
            }
            WorldModel::Globe(_) => {}
        }

        if button(ui, "Generate Preview") {
            events.generate_request = Some((self.get_layer(), false));
        }
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Next => Box::<MainPanelContinents>::default(),
            _ => Box::new(*self),
        }
    }
}
