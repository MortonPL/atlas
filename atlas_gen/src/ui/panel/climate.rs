use bevy_egui::egui::{self, Ui};

use crate::{
    config::AtlasGenConfig,
    event::EventStruct,
    map::MapDataLayer,
    ui::{internal::UiState, panel::SidebarPanel},
};

/// Panel with climate generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl SidebarPanel for MainPanelClimate {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        show_biomes_readonly(ui, config, events);
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Climate
    }
}

// TODO Make biomes work with MakeUi nicely?
fn show_biomes_readonly(ui: &mut Ui, config: &mut AtlasGenConfig, events: &mut EventStruct) {
    if ui.button("Reload \"climatemap.png\"").clicked() {
        events.load_climatemap_request = Some(());
    }
    egui::CollapsingHeader::new(egui::RichText::new("Climate list").heading())
        .default_open(true)
        .show(ui, |ui| {
            for biome in &config.climate.biomes {
                let color = egui::Color32::from_rgb(biome.color[0], biome.color[1], biome.color[2]);
                ui.heading(egui::RichText::new(&biome.name).color(color));
                egui::Grid::new(format!("{}_climate_grid", biome.name)).show(ui, |ui| {
                    ui.label("Productivity");
                    ui.label(biome.productivity.to_string());
                    ui.end_row();
                });
            }
        });
}
