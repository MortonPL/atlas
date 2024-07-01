use atlas_lib::{bevy_egui::egui::Ui, domain::map::MapDataLayer, ui::sidebar::{MakeUi, SidebarPanel}};

use crate::{config::AtlasSimConfig, ui::AtlasSimUi};

/// Panel with general simulation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelGeneral {
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
