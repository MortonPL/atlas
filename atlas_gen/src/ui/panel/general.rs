use atlas_lib::{
    bevy_egui::egui::{Grid, Ui},
    domain::map::MapDataLayer,
    ui::{
        button,
        sidebar::{MakeUi, SidebarPanel},
        UiEditableEnum,
    },
};

use crate::{
    config::AtlasGenConfig,
    event::EventStruct,
    ui::{panel::SidebarPanelGen, AtlasGenUi},
};

/// Panel with general world gen and preview settings.
#[derive(Default, Clone, Copy)]
pub struct MainPanelGeneral {}

impl SidebarPanel<AtlasGenConfig, EventStruct, AtlasGenUi> for MainPanelGeneral {
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut AtlasGenUi,
        events: &mut EventStruct,
    ) {
        let old_world_model = config.general.preview_model.self_as_index();
        let old = config.general.world_size;

        Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
            config.general.make_ui(ui);
        });

        if config.general.preview_model.self_as_index() != old_world_model {
            events.world_model_changed = Some(());
        }

        if old != config.general.world_size {
            events.world_model_changed = Some(());
        }

        if button(ui, "Generate Preview") {
            events.generate_request = Some((self.get_layer(), false));
        }
    }

    fn get_heading(&self) -> &'static str {
        "General"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Preview
    }
}

impl SidebarPanelGen for MainPanelGeneral {}
