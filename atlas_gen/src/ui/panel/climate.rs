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
        show_climates_readonly(ui, config, events);
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

fn show_climates_readonly(ui: &mut Ui, config: &mut SessionConfig, events: &mut EventStruct) {
    if ui.button("Reload \"climatemap.png\"").clicked() {
        events.load_climatemap_request = Some(());
    }
    egui::CollapsingHeader::new(egui::RichText::new("Climate list").heading()).default_open(true).show(ui, |ui| {
        for climate in &config.climate.climates {
            let color = egui::Color32::from_rgb(climate.color[0], climate.color[1], climate.color[2]);
            ui.heading(egui::RichText::new(&climate.name).color(color));
            egui::Grid::new(format!("{}_climate_grid", climate.name)).show(ui, |ui| {
                ui.label("Productivity");
                ui.label(climate.productivity.to_string());
                ui.end_row();
            });
        }
    });
}
