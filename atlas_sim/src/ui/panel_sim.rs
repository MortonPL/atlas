use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::{Grid, Ui},
    config::sim::AtlasSimConfig,
    domain::map::MapDataLayer,
    ui::sidebar::{MakeUi, SidebarPanel},
};

use crate::ui::AtlasSimUi;

macro_rules! make_panel {
    ($panel:ident, $name:literal, $fun:ident) => {
        #[derive(Default, Clone, Copy)]
        pub struct $panel;

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
            if let Some(region) = &mut selection.region {
                ui.add_enabled_ui(true, |ui| {
                    Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
                        region.make_ui(ui);
                    });
                });
                return;
            } else if let Some(conflict) = &mut selection.conflict {
                ui.add_enabled_ui(true, |ui| {
                    Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
                        conflict.make_ui(ui);
                    });
                });
            } else {
                /* Other selectables */
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

make_panel!(InfoPanelPolity, "General", make_ui);
make_panel!(InfoPanelEconomy, "Economy", make_ui_economy);
make_panel!(InfoPanelScience, "Science", make_ui_science);
make_panel!(InfoPanelCulture, "Culture", make_ui_culture);
