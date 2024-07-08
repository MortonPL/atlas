use atlas_lib::{
    bevy_egui::egui::Ui,
    domain::map::{MapDataLayer, MapDataOverlay},
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::{config::AtlasSimConfig, ui::AtlasSimUi};

/// Panel with selected object info.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelMisc;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for InfoPanelMisc {
    fn make_ui(&mut self, _ui: &mut Ui, _config: &mut AtlasSimConfig) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Selection"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }

    fn get_overlay(&self) -> MapDataOverlay {
        MapDataOverlay::None
    }
}
