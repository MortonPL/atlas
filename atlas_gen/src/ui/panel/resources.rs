use atlas_lib::ui::sidebar::MakeUi;
use bevy_egui::egui::Ui;

use crate::{config::AtlasGenConfig, map::MapDataLayer, ui::panel::SidebarPanel};

#[derive(Default, Clone, Copy)]
pub struct MainPanelResources;

impl SidebarPanel for MainPanelResources {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.resources.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Resources"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Resources
    }
}
