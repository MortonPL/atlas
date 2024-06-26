use atlas_lib::{bevy_egui::egui::Ui, domain::map::MapDataLayer, ui::sidebar::MakeUi};

use crate::{
    config::{AtlasGenConfig, InfluenceShape},
    ui::panel::SidebarPanel,
};

/// Panel with topography generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelTopography;

impl SidebarPanel for MainPanelTopography {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.topography.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Topography"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Topography
    }

    fn get_influence_shape<'b>(&self, config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &config.topography.influence_shape
    }
}
