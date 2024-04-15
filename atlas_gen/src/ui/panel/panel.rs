use bevy_egui::egui::Ui;

use crate::{
    config::SessionConfig,
    event::EventStruct,
    map::ViewedMapLayer,
    ui::{internal::UiState, panel::general::MainPanelGeneral},
};

/// Transition between sidebar panels.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum MainPanelTransition {
    #[default]
    None,
    Previous,
    Next,
}

/// A sidebar page.
pub trait MainPanel {
    /// Get panel heading.
    fn get_heading(&self) -> &'static str;

    /// Get layer that should be displayed with this panel.
    fn get_layer(&self) -> ViewedMapLayer;

    /// Create UI for this panel.
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        ui_state: &mut UiState,
        events: &mut EventStruct,
    );

    /// Handle transitioning to the previous or next panel.
    fn transition(&self, transition: MainPanelTransition) -> Box<dyn MainPanel + Sync + Send>;
}

impl Default for Box<dyn MainPanel + Sync + Send> {
    fn default() -> Self {
        Box::<MainPanelGeneral>::default()
    }
}
