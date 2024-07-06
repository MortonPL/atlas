use atlas_lib::{
    bevy_egui::egui::Ui,
    domain::map::MapDataLayer,
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::{config::AtlasSimConfig, ui::AtlasSimUi};

/// Panel with simulation rules.
#[derive(Default, Clone, Copy)]
pub struct MainPanelRules;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelRules {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.rules.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Rules"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}
