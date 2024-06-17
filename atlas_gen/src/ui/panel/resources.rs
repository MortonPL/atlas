use crate::{
    config::AtlasGenConfig,
    event::EventStruct,
    ui::{panel::SidebarPanelGen, AtlasGenUi},
};
use atlas_lib::{
    bevy_egui::egui::Ui,
    domain::map::MapDataLayer,
    ui::sidebar::{MakeUi, SidebarPanel},
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelResources;

impl SidebarPanel<AtlasGenConfig, EventStruct, AtlasGenUi> for MainPanelResources {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.resources.make_ui(ui);
    }

    fn extra_ui(
        &mut self,
        ui: &mut Ui,
        _config: &mut AtlasGenConfig,
        _ui_state: &mut AtlasGenUi,
        events: &mut EventStruct,
    ) {
        self.button_layer(ui, events);
    }

    fn get_heading(&self) -> &'static str {
        "Resources"
    }

    fn get_layer(&self) -> MapDataLayer {
        MapDataLayer::Resources
    }
}

impl SidebarPanelGen for MainPanelResources {}
