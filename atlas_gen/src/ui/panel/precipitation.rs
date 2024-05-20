use bevy_egui::egui::Ui;

use atlas_lib::ui::sidebar::MakeUi;

use crate::{
    config::{AtlasGenConfig, InfluenceShape},
    map::MapDataLayer,
    ui::panel::SidebarPanel,
};

/// Panel with precipitation generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelPrecipitation;

impl SidebarPanel for MainPanelPrecipitation {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.precipitation.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Precipitation"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Precipitation
    }

    fn get_influence_shape<'b>(&self, config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &config.precipitation.influence_shape
    }
}
