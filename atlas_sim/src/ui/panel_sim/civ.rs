use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::{Grid, Ui},
    domain::map::{MapDataLayer, MapDataOverlay},
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::{config::AtlasSimConfig, ui::AtlasSimUi};

/// Panel with polity summary.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelCiv;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for InfoPanelCiv {
    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasSimConfig,
        ui_state: &mut AtlasSimUi,
        _events: &mut EventStruct,
    ) {
        if let Some(selection) = &mut ui_state.selection {
            if let Some(civ) = &mut selection.civ {
                ui.add_enabled_ui(false, |ui| {
                    Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
                        civ.make_ui(ui);
                    });
                });
                return;
            }
        }
        ui.label("No object selected.");
    }

    fn get_heading(&self) -> &'static str {
        "Civilization"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }

    fn get_overlay(&self) -> MapDataOverlay {
        MapDataOverlay::Civilizations
    }
}
