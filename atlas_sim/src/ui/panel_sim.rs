use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::{Grid, Ui},
    config::sim::AtlasSimConfig,
    domain::map::MapDataLayer,
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::ui::AtlasSimUi;

/// Panel with selected object info.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelMisc;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for InfoPanelMisc {
    fn make_ui(&mut self, _ui: &mut Ui, _config: &mut AtlasSimConfig) {
        // TODO
    }

    fn get_heading(&self) -> &'static str {
        "Selection"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

/// Panel with polity summary.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelPolity;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for InfoPanelPolity {
    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasSimConfig,
        ui_state: &mut AtlasSimUi,
        _events: &mut EventStruct,
    ) {
        if let Some(selection) = &mut ui_state.selection {
            if let Some(polity) = &mut selection.polity {
                ui.add_enabled_ui(false, |ui| {
                    Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
                        polity.make_ui(ui);
                    });
                });
                return;
            }
        }
        ui.label("No object selected.");
    }

    fn get_heading(&self) -> &'static str {
        "Polity"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

/// Panel with city summary.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelCity;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for InfoPanelCity {
    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasSimConfig,
        ui_state: &mut AtlasSimUi,
        _events: &mut EventStruct,
    ) {
        if let Some(selection) = &mut ui_state.selection {
            if let Some(city) = &mut selection.city {
                ui.add_enabled_ui(false, |ui| {
                    Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
                        city.make_ui(ui);
                    });
                });
                return;
            }
        }
        ui.label("No object selected.");
    }

    fn get_heading(&self) -> &'static str {
        "City"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}
