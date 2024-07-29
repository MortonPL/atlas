use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::Ui,
    config::sim::{AtlasSimConfig, StartingPoint},
    domain::map::MapDataLayer,
    ui::{
        button, button_action,
        sidebar::{MakeUi, SidebarPanel},
    },
};

use crate::ui::{panel_sim::InfoPanelPolity, AtlasSimUi};

/// Panel with general simulation settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelGeneral {
    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasSimConfig,
        ui_state: &mut AtlasSimUi,
        events: &mut EventStruct,
    ) {
        button_action(ui, "Begin Simulation", || {
            events.simulation_start_request = Some(());
            ui_state.setup_mode = false;
            ui_state.current_panel = Box::<InfoPanelPolity>::default();
            ui_state.force_changed = true;
        });
    }

    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.general.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
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
        _ui_state: &mut AtlasSimUi,
        events: &mut EventStruct,
    ) {
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
                config.scenario.start_points.push(StartingPoint::default());
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

/// Panel with simulation rules.
#[derive(Default, Clone, Copy)]
pub struct MainPanelRulesMisc;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelRulesMisc {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.rules.misc.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Rules (Misc)"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

/// Panel with simulation rules.
#[derive(Default, Clone, Copy)]
pub struct MainPanelRulesEco;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelRulesEco {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.rules.economy.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Rules (Economy)"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

/// Panel with simulation rules.
#[derive(Default, Clone, Copy)]
pub struct MainPanelRulesTech;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelRulesTech {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.rules.tech.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Rules (Tech)"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

/// Panel with simulation rules.
#[derive(Default, Clone, Copy)]
pub struct MainPanelRulesCult;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelRulesCult {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.rules.culture.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Rules (Culture)"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

/// Panel with simulation rules.
#[derive(Default, Clone, Copy)]
pub struct MainPanelRulesCity;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelRulesCity {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.rules.city.make_ui(ui);
    }

    fn get_heading(&self) -> &'static str {
        "Rules (City)"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}
