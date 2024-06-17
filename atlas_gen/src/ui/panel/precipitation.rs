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

/// Panel with precipitation generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelPrecipitation;

impl SidebarPanel<AtlasGenConfig, EventStruct, AtlasGenUi> for MainPanelPrecipitation {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.precipitation.make_ui(ui);
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
        "Precipitation"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Precipitation
    }
}

impl SidebarPanelGen for MainPanelPrecipitation {
    fn get_influence_shape<'b>(&self, config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &config.precipitation.influence_shape
    }
}
