use atlas_lib::{bevy_egui::egui::Ui, domain::map::MapDataLayer};

use crate::{config::AtlasSimConfig, ui::panel::SidebarPanel};

/// Panel with general simulation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral;

impl SidebarPanel for MainPanelGeneral {
    fn make_ui(&mut self, _ui: &mut Ui, _config: &mut AtlasSimConfig) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}
