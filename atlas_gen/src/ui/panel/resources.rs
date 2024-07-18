use crate::ui::{panel::SidebarPanelGen, AtlasGenUi};
use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::Ui,
    config::gen::AtlasGenConfig,
    domain::map::MapDataLayer,
    ui::{
        button,
        sidebar::{MakeUi, SidebarPanel},
    },
};

#[derive(Default, Clone, Copy)]
pub struct MainPanelResources;

impl SidebarPanel<AtlasGenConfig, AtlasGenUi> for MainPanelResources {
    fn make_ui(&mut self, ui: &mut Ui, config: &mut AtlasGenConfig) {
        config.deposits.make_ui(ui);
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
        MapDataLayer::Preview
    }
}

impl SidebarPanelGen for MainPanelResources {
    /// Create a "Generate Layer" button.
    fn button_layer(&self, ui: &mut Ui, events: &mut EventStruct) {
        if button(ui, "Generate Layer") {
            events.generate_request = Some((MapDataLayer::Resources, false));
        }
    }
}
