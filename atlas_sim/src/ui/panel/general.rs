use atlas_lib::{bevy_egui::egui::Ui, domain::map::MapDataLayer, ui::sidebar::SidebarPanel};

use crate::{config::AtlasSimConfig, ui::AtlasSimUi};

/// Panel with general simulation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelGeneral {
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
