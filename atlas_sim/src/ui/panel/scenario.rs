use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::Ui,
    domain::map::MapDataLayer,
    ui::{
        button,
        sidebar::{MakeUi, SidebarPanel},
    },
};

use crate::{
    config::{AtlasSimConfig, StartingPoint},
    ui::AtlasSimUi,
};

/// Panel with initial scenario settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelScenario;

impl SidebarPanel<AtlasSimConfig, AtlasSimUi> for MainPanelScenario {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasSimConfig) {
        config.scenario.make_ui(ui);
        Self::ensure_starting_points(config);
    }

    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasSimConfig,
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
