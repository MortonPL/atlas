use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::Ui,
    domain::map::MapDataLayer,
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::{
    config::{AtlasGenConfig, InfluenceShape},
    ui::{panel::SidebarPanelGen, AtlasGenUi},
};

/// Panel with continents generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelContinents;

impl SidebarPanel<AtlasGenConfig, AtlasGenUi> for MainPanelContinents {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.continents.make_ui(ui);
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
        "Continents"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Continents
    }
}

impl SidebarPanelGen for MainPanelContinents {
    fn get_influence_shape<'b>(&self, config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &config.continents.influence_shape
    }
}
