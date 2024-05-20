use atlas_lib::ui::button;
use bevy_egui::egui::Ui;

use crate::{
    config::{AtlasGenConfig, InfluenceShape},
    event::EventStruct,
    map::MapDataLayer,
    ui::{internal::UiState, panel::MainPanelGeneral},
};

/// A sidebar page/panel.
pub trait SidebarPanel {
    /// Get panel heading.
    fn get_heading(&self) -> &'static str;

    /// Get layer that should be displayed with this panel.
    fn get_layer(&self) -> MapDataLayer;

    /// Create a config UI for this panel. Nothing shown by default.
    fn make_ui(&mut self, _ui: &mut Ui, _config: &mut AtlasGenConfig) {}

    /// Get influence shape from this panel's config. [`InfluenceShape::None`] by default.
    fn get_influence_shape<'b>(&self, _config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &InfluenceShape::None
    }

    /// Create UI for this panel.
    fn show(
        &mut self,
        ui: &mut Ui,
        config: &mut AtlasGenConfig,
        _ui_state: &mut UiState,
        events: &mut EventStruct,
    ) {
        self.make_ui(ui, config);
        self.button_influence(ui, events, self.get_influence_shape(config));
        self.button_layer(ui, events);
    }

    /// Create a "Generate Layer" button.
    fn button_layer(&self, ui: &mut Ui, events: &mut EventStruct) {
        if button(ui, "Generate Layer") {
            events.generate_request = Some((self.get_layer(), false));
        }
    }

    /// Create a "Generate Influence Map" button.
    fn button_influence(&self, ui: &mut Ui, events: &mut EventStruct, influence: &InfluenceShape) {
        if !matches!(influence, InfluenceShape::None) && button(ui, "Generate Influence Map") {
            if let Some(layer) = self.get_layer().get_influence_layer() {
                events.generate_request = Some((layer, false));
            }
        }
    }
}

impl Default for Box<dyn SidebarPanel + Sync + Send> {
    fn default() -> Self {
        Box::<MainPanelGeneral>::default()
    }
}
