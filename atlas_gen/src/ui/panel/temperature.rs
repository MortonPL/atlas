use atlas_lib::{
    bevy_egui::egui::Ui,
    domain::map::MapDataLayer,
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::{
    config::{AtlasGenConfig, InfluenceShape},
    event::EventStruct,
    ui::{panel::SidebarPanelGen, AtlasGenUi},
};

/// Panel with temperature generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelTemperature;

impl SidebarPanel<AtlasGenConfig, EventStruct, AtlasGenUi> for MainPanelTemperature {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.temperature.make_ui(ui);
    }

    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut AtlasGenUi,
        events: &mut EventStruct,
    ) {
        self.button_influence(ui, events, self.get_influence_shape(config));
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Temperature"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Temperature
    }
}

impl SidebarPanelGen for MainPanelTemperature {
    fn get_influence_shape<'b>(&self, config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &config.temperature.influence_shape
    }
}
