use bevy_egui::egui::{self, Response, Ui};

use crate::{
    config::GeneratorConfig,
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
        config: &mut GeneratorConfig,
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

/// Add a section consisting of a collapsible header and a grid.
pub fn add_section<BodyRet>(
    ui: &mut Ui,
    header: impl Into<String>,
    add_body: impl FnOnce(&mut Ui) -> BodyRet,
) -> Response {
    let header: String = header.into();
    egui::CollapsingHeader::new(egui::RichText::new(header.clone()).heading())
        .default_open(true)
        .show(ui, |ui| {
            egui::Grid::new(format!("{}_grid", header)).show(ui, add_body);
        })
        .header_response
}
