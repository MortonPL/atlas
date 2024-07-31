use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::{Grid, Ui},
    config::sim::AtlasSimConfig,
    domain::map::MapDataLayer,
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::ui::AtlasSimUi;

macro_rules! make_panel {
    ($panel:ty, $name:literal, $fun:ident) => {
        impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for $panel {
            fn extra_ui(
                &mut self,
                ui: &mut Ui,
                _config: &mut AtlasSimConfig,
                ui_state: &mut AtlasSimUi,
                _events: &mut EventStruct,
            ) {
                if let Some(selection) = &mut ui_state.selection {
                    if let Some(polity) = &mut selection.polity {
                        ui.add_enabled_ui(true, |ui| {
                            Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
                                polity.$fun(ui);
                            });
                        });
                        return;
                    }
                }
                ui.label("No object selected.");
            }

            fn get_heading(&self) -> &'static str {
                $name
            }

            fn get_layer(&self) -> MapDataLayer {
                MapDataLayer::Preview
            }
        }
    };
}

/// Panel with selected object info.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelMisc;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for InfoPanelMisc {
    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasSimConfig,
        ui_state: &mut AtlasSimUi,
        _events: &mut EventStruct,
    ) {
        if let Some(selection) = &mut ui_state.selection {
            if let Some(city) = &mut selection.city {
                ui.add_enabled_ui(true, |ui| {
                    Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
                        city.make_ui(ui);
                    });
                });
                return;
            } else {
                // TODO other selectables
            }
        }
        ui.label("No object selected.");
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

/// Panel with polity economy summary.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelEconomy;

/// Panel with polity research summary.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelScience;

/// Panel with polity culture summary.
#[derive(Default, Clone, Copy)]
pub struct InfoPanelCulture;

make_panel!(InfoPanelPolity, "General", make_ui);
make_panel!(InfoPanelEconomy, "Economy", make_ui_economy);
make_panel!(InfoPanelScience, "Science", make_ui_science);
make_panel!(InfoPanelCulture, "Culture", make_ui_culture);
