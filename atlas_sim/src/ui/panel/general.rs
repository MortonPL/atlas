use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::Ui,
    domain::map::MapDataLayer,
    ui::{
        button_action,
        sidebar::{MakeUi, SidebarPanel},
    },
};

use crate::{
    config::AtlasSimConfig,
    ui::{panel_sim::InfoPanelPolity, AtlasSimUi},
};

/// Panel with general simulation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelGeneral {
    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasSimConfig,
        ui_state: &mut AtlasSimUi,
        events: &mut EventStruct,
    ) {
        button_action(ui, "Begin Simulation", || {
            events.simulation_start_request = Some(());
            ui_state.setup_mode = false;
            ui_state.current_panel = Box::<InfoPanelPolity>::default();
            ui_state.force_changed = true;
        });
    }

    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.general.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}
