use bevy_egui::egui::{self, Ui};

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::MapDataLayer,
    ui::{
        internal::UiState,
        panel::{MainPanelPrecipitation, MainPanelResources, MainPanelTransition, SidebarPanel},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl SidebarPanel for MainPanelClimate {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        show_climates_readonly(ui, config);
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Climate
    }

    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send> {
        match transition {
            MainPanelTransition::Previous => Box::<MainPanelPrecipitation>::default(),
            MainPanelTransition::None => Box::new(*self),
            MainPanelTransition::Next => Box::<MainPanelResources>::default(),
        }
    }
}

fn show_climates_readonly(ui: &mut Ui, config: &mut SessionConfig) {
    for climate in &config.climate.climates {
        let color = egui::Color32::from_rgb(climate.color[0], climate.color[1], climate.color[2]);
        egui::CollapsingHeader::new(egui::RichText::new(&climate.name).heading().color(color))
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new(format!("{}_climate_grid", climate.name)).show(ui, |ui| {
                    ui.label("Minimum Temperature");
                    ui.label(climate.temperature[0].to_string());
                    ui.label("Maximum Temperature");
                    ui.label(climate.temperature[1].to_string());
                    ui.end_row();
                    ui.label("Minimum Precipitation (x40mm)");
                    ui.label(climate.precipitation[0].to_string());
                    ui.label("Maximum Precipitation (x40mm)");
                    ui.label(climate.precipitation[1].to_string());
                    ui.end_row();
                })
            });
    }
}
