use bevy_egui::egui::Ui;

use atlas_lib::ui::sidebar::MakeUi;

use crate::{
    config::{AtlasGenConfig, InfluenceShape},
    map::MapDataLayer,
    ui::panel::{MainPanelContinents, MainPanelTemperature, MainPanelTransition, SidebarPanel},
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

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelContinents>::default(),
            MainPanelTransition::Next => Box::<MainPanelTemperature>::default(),
        }
    }
}
