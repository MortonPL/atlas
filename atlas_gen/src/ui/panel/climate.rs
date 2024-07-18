use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::{Grid, Ui},
    config::gen::AtlasGenConfig,
    domain::map::MapDataLayer,
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::ui::{panel::SidebarPanelGen, AtlasGenUi};

/// Panel with climate generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl SidebarPanel<AtlasGenConfig, AtlasGenUi> for MainPanelClimate {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut AtlasGenUi,
        events: &mut EventStruct,
    ) {
        if ui.button("Reload \"climatemap.png\"").clicked() {
            events.load_climatemap_request = Some(());
        }

        Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
            self.make_ui(ui, config);
        });

        self.button_layer(ui, events);
    }

    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.climate.make_ui(ui);
        if config.climate.mountains_biome as usize >= config.climate.biomes.len() {
            config.climate.mountains_biome = 0
        }
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Climate
    }
}

impl SidebarPanelGen for MainPanelClimate {}
