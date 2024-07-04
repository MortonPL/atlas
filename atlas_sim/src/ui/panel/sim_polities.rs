use atlas_lib::{
    bevy_egui::egui::Ui,
    domain::map::{MapDataLayer, MapDataOverlay},
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::{config::AtlasSimConfig, ui::AtlasSimUi};

/// Panel with civilization summary.
#[derive(Default, Clone, Copy)]
pub struct SimPanelPolities;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for SimPanelPolities {
    fn make_ui(&mut self, _ui: &mut Ui, _config: &mut AtlasSimConfig) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Polities"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }

    fn get_overlay(&self) -> MapDataOverlay {
        MapDataOverlay::Polities
    }
}
