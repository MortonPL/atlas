use atlas_lib::{bevy_egui::egui::Ui, domain::map::MapDataLayer, ui::sidebar::MakeUi};

use crate::{
    config::{AtlasGenConfig, InfluenceShape},
    ui::panel::SidebarPanel,
};

/// Panel with continents generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelContinents;

impl SidebarPanel for MainPanelContinents {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.continents.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Continents"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Continents
    }

    fn get_influence_shape<'b>(&self, config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &config.continents.influence_shape
    }
}
