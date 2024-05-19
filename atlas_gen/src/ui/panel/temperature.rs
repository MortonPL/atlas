use bevy_egui::egui::Ui;

use atlas_lib::ui::sidebar::MakeUi;

use crate::{
    config::{AtlasGenConfig, InfluenceShape},
    map::MapDataLayer,
    ui::panel::{MainPanelPrecipitation, MainPanelTopography, MainPanelTransition, SidebarPanel},
};

/// Panel with temperature generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelTemperature;

impl SidebarPanel for MainPanelTemperature {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.temperature.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Temperature"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Temperature
    }

    fn get_influence_shape<'b>(&self, config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &config.temperature.influence_shape
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Previous => Box::<MainPanelTopography>::default(),
            MainPanelTransition::Next => Box::<MainPanelPrecipitation>::default(),
        }
    }
}
