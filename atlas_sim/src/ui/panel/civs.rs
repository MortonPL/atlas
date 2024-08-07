use atlas_lib::{
    bevy_egui::egui::Ui, config::sim::AtlasSimConfig, domain::map::MapDataLayer, ui::sidebar::SidebarPanel,
};

use crate::ui::AtlasSimUi;

/// Panel with civilization summary.
#[derive(Default, Clone, Copy)]
pub struct MainPanelCiv;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelCiv {
    fn make_ui(&mut self, _ui: &mut Ui, _config: &mut AtlasSimConfig) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Civilizations"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}
