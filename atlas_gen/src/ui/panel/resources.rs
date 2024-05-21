use crate::{config::AtlasGenConfig, ui::panel::SidebarPanel};
use atlas_lib::{bevy_egui::egui::Ui, domain::map::MapDataLayer, ui::sidebar::MakeUi};

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
