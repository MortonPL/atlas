use atlas_lib::{
    base::events::EventStruct,
    bevy_egui::egui::Ui,
    config::gen::{AtlasGenConfig, InfluenceShape},
    ui::{button, sidebar::SidebarPanel},
};

use crate::ui::AtlasGenUi;

/// A sidebar page/panel.
pub trait SidebarPanelGen: SidebarPanel<AtlasGenConfig, AtlasGenUi> {
    /// Get influence shape from this panel's config. [`InfluenceShape::None`] by default.
    fn get_influence_shape<'b>(&self, _config: &'b AtlasGenConfig) -> &'b InfluenceShape {
        &InfluenceShape::None
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
