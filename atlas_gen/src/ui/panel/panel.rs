use atlas_lib::ui::button;
use bevy_egui::egui::Ui;

use crate::{
    config::{InfluenceShape, SessionConfig},
    event::EventStruct,
    map::MapDataLayer,
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
pub trait SidebarPanel {
    /// Get panel heading.
    fn get_heading(&self) -> &'static str;

    /// Get layer that should be displayed with this panel.
    fn get_layer(&self) -> MapDataLayer;

    /// Create UI for this panel.
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut SessionConfig,
        ui_state: &mut UiState,
        events: &mut EventStruct,
    );

    /// Handle transitioning to the previous or next panel.
    fn transition(&self, transition: MainPanelTransition) -> Box<dyn SidebarPanel + Sync + Send>;

    /// Create a "Generate Layer" button.
    fn button_layer(&self, ui: &mut Ui, events: &mut EventStruct) {
        if button(ui, "Generate Layer") {
            events.generate_request = Some(self.get_layer());
        }
    }

    /// Create a "Generate Influence Map" button.
    fn button_influence(&self, ui: &mut Ui, events: &mut EventStruct, influence: &InfluenceShape) {
        if !matches!(influence, InfluenceShape::None(_)) && button(ui, "Generate Influence Map") {
            events.generate_request = self.get_layer().get_influence_layer();
        }
    }
}

impl Default for Box<dyn SidebarPanel + Sync + Send> {
    fn default() -> Self {
        Box::<MainPanelGeneral>::default()
    }
}
