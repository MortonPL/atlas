use atlas_lib::{bevy_egui::egui::Ui, domain::map::MapDataLayer, ui::sidebar::MakeUi};

use crate::{
    config::AtlasGenConfig,
    event::EventStruct,
    ui::{internal::UiState, panel::SidebarPanel},
};

/// Panel with climate generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl SidebarPanel for MainPanelClimate {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        if ui.button("Reload \"climatemap.png\"").clicked() {
            events.load_climatemap_request = Some(());
        }

        self.make_ui(ui, config);
        self.button_layer(ui, events);
    }

    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.climate.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Climate
    }
}
