use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::Ui,
    config::sim::{AtlasSimConfig, PolityConfig, StartingPoint},
    domain::map::MapDataLayer,
    ui::{
        button, button_action_enabled,
        sidebar::{MakeUi, SidebarPanel},
    },
};

use crate::ui::{panel_sim::InfoPanelPolity, AtlasSimUi};

macro_rules! make_panel {
    ($panel:ident, $name:literal, $field:ident) => {
        /// Panel with simulation rules.
        #[derive(Default, Clone, Copy)]
        pub struct $panel;

        impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for $panel {
            fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
                config.rules.$field.make_ui(ui);
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

/// Panel with initial scenario settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelScenario;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelScenario {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.scenario.make_ui(ui);
        Self::ensure_starting_points(config);
    }

    fn extra_ui_pre(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasSimConfig,
        ui_state: &mut AtlasSimUi,
        events: &mut EventStruct,
    ) {
        button_action_enabled(ui, "Begin Simulation", ui_state.world_loaded, || {
            events.simulation_start_request = Some(());
            ui_state.setup_mode = false;
            ui_state.current_panel = Box::<InfoPanelPolity>::default();
            ui_state.force_changed = true;
        });
        if button(ui, "Randomize Starting Points") {
            events.randomize_starts_request = Some(());
        }
    }

    fn get_heading(&self) -> &'static str {
        "Scenario"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

impl MainPanelScenario {
    fn ensure_starting_points(config: &mut AtlasSimConfig) {
        let diff = config.scenario.num_starts as i32 - config.scenario.start_points.len() as i32;
        if diff >= 0 {
            for _ in 0..diff as usize {
                config.scenario.start_points.push(StartingPoint {
                    polity: PolityConfig {
                        population: config.scenario.start_pop,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        } else {
            for _ in 0..(-diff as usize) {
                config.scenario.start_points.pop();
            }
        }
        for point in &mut config.scenario.start_points {
            point.position[0] = point.position[0].clamp(0, config.general.world_size[0] - 1);
            point.position[1] = point.position[1].clamp(0, config.general.world_size[1] - 1);
        }
    }
}

/// Panel with climate generation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelClimate;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelClimate {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.climate.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Climate"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Climate
    }
}

make_panel!(MainPanelRulesDiplo, "Rules - Diplomacy", diplomacy);
make_panel!(MainPanelRulesEco, "Rules - Economy", economy);
make_panel!(MainPanelRulesTech, "Rules - Science", science);
make_panel!(MainPanelRulesCulture, "Rules - Culture", culture);
make_panel!(MainPanelRulesRegion, "Rules - Region", region);
make_panel!(MainPanelRulesCombat, "Rules - Combat", combat);
