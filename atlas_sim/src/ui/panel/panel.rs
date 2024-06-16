use atlas_lib::{
    bevy_egui::egui::{Grid, Ui},
    domain::map::MapDataLayer,
};

use crate::{
    config::AtlasSimConfig,
    event::EventStruct,
    ui::{internal::UiState, panel::MainPanelGeneral},
};

/// A sidebar page/panel.
pub trait SidebarPanel {
    /// Get panel heading.
    /// NOTE: Must be a unique string!
    fn get_heading(&self) -> &'static str;

    /// Get layer that should be displayed with this panel.
    fn get_layer(&self) -> MapDataLayer;

    /// Create a config UI for this panel. Nothing shown by default.
    fn make_ui(&mut self, _ui: &mut Ui, _config: &mut AtlasSimConfig) {}

    /// Create UI for this panel.
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasSimConfig,
        _ui_state: &mut UiState,
        _events: &mut EventStruct,
    ) {
        Grid::new(format!("{}_panel", self.get_heading())).show(ui, |ui| {
            self.make_ui(ui, config);
        });
    }
}

impl Default for Box<dyn SidebarPanel + Sync + Send> {
    fn default() -> Self {
        Box::<MainPanelGeneral>::default()
    }
}
