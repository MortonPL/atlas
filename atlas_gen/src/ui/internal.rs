use std::path::Path;

use atlas_lib::{
    base::{
        events::EventStruct,
        ui::{HandleFileDialog, UiStateBase},
    },
    bevy::prelude::*,
    bevy_egui::egui::Ui,
    config::{
        gen::{AtlasGenConfig, InfluenceShape},
        load_config, load_image, load_image_grey, save_config,
    },
    domain::map::MapDataLayer,
    ui::{button, sidebar::SidebarPanel},
};

use crate::ui::{panels::MainPanelGeneral, AtlasGenUi};

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

/// Reset generator config to defaults.
pub fn reset_config_clicked(
    config: &mut AtlasGenConfig,
    ui_state: &mut AtlasGenUi,
    events: &mut EventStruct,
) {
    *config = AtlasGenConfig::default();
    ui_state.current_panel = Box::<MainPanelGeneral>::default();
    events.world_model_changed = Some(());
}

// Clear layer data.
pub fn clear_layer_clicked(ui_base: &mut UiStateBase, events: &mut EventStruct) {
    events.clear_layer_request = Some(ui_base.current_layer);
}

/// Reset a config from one panel to defaults, and reset relevant logic layers.
pub fn reset_panel_clicked(config: &mut AtlasGenConfig, ui_state: &mut AtlasGenUi, events: &mut EventStruct) {
    macro_rules! reset_panel {
        ($field:ident, $layer:ident) => {{
            config.$field = default();
            events.generate_request = Some((MapDataLayer::$layer, true));
        }};
    }
    match ui_state.current_panel.get_layer() {
        MapDataLayer::Preview => {
            config.general = default();
            events.world_model_changed = Some(());
        }
        MapDataLayer::Continents => reset_panel!(continents, Continents),
        MapDataLayer::Topography => reset_panel!(topography, Topography),
        MapDataLayer::Temperature => reset_panel!(temperature, Temperature),
        MapDataLayer::Precipitation => reset_panel!(precipitation, Precipitation),
        MapDataLayer::Climate => reset_panel!(climate, Climate),
        MapDataLayer::Deposits => reset_panel!(deposits, Deposits),
        _ => unreachable!(),
    }
}

/// A handler implementation for the egui file dialog.
pub struct FileDialogHandler<'a> {
    pub events: &'a mut EventStruct,
    pub config: &'a mut AtlasGenConfig,
}

impl<'a> FileDialogHandler<'a> {
    pub fn new(events: &'a mut EventStruct, config: &'a mut AtlasGenConfig) -> Self {
        Self { events, config }
    }
}

impl<'a> HandleFileDialog for FileDialogHandler<'a> {
    fn load_config(&mut self, path: &Path) {
        match load_config(path) {
            Ok(data) => {
                *self.config = data;
                self.events.world_model_changed = Some(());
            }
            Err(err) => self.events.error_window = Some(err.to_string()),
        }
    }

    fn save_config(&mut self, path: &Path) {
        if let Err(err) = save_config(self.config, path) {
            self.events.error_window = Some(err.to_string());
        }
    }

    fn load_layer_data(&mut self, path: &Path, layer: MapDataLayer) {
        let (width, height) = (
            self.config.general.world_size[0],
            self.config.general.world_size[1],
        );
        let result = match layer {
            MapDataLayer::Preview => load_image(path, width, height),
            _ => load_image_grey(path, width, height),
        };
        match result {
            Ok(data) => self.events.load_layer_request = Some((layer, data)),
            Err(err) => self.events.error_window = Some(err.to_string()),
        };
    }

    fn save_layer_data(&mut self, path: &Path, layer: MapDataLayer) {
        self.events.save_layer_request = Some((layer, path.into()));
    }

    fn render_image(&mut self, path: &Path, layer: MapDataLayer) {
        self.events.render_layer_request = Some((layer, path.into()));
    }

    fn export(&mut self, path: &Path) {
        self.events.export_world_request = Some(path.into());
    }

    fn import(&mut self, path: &Path) {
        self.events.import_world_request = Some(path.into());
    }

    fn import_special(&mut self, _path: &Path) {
        unreachable!()
    }
}
