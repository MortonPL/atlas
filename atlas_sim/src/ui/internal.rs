use std::path::Path;

use atlas_lib::{
    base::{events::EventStruct, ui::HandleFileDialog},
    config::{load_config, save_config, sim::AtlasSimConfig},
    domain::map::MapDataLayer,
};

use crate::ui::{panel_init::MainPanelGeneral, AtlasSimUi};

/// Reset generator config to defaults.
pub fn reset_config_clicked(
    config: &mut AtlasSimConfig,
    ui_state: &mut AtlasSimUi,
    events: &mut EventStruct,
) {
    *config = AtlasSimConfig::default();
    ui_state.current_panel = Box::<MainPanelGeneral>::default();
    events.world_model_changed = Some(());
}

/// Reset a config from one panel to defaults, and reset relevant logic layers.
pub fn reset_panel_clicked(config: &mut AtlasSimConfig, ui_state: &AtlasSimUi, _events: &mut EventStruct) {
    macro_rules! reset_panel {
        ($field:ident) => {{
            config.$field = Default::default();
        }};
        ($field:ident, $subfield:ident) => {{
            config.$field.$subfield = Default::default();
        }};
    }
    match ui_state.current_panel.get_heading() {
        "General" => reset_panel!(general),
        "Scenario" => reset_panel!(scenario),
        "Rules (Misc)" => reset_panel!(rules, misc),
        "Rules (Economy)" => reset_panel!(rules, economy),
        "Rules (Tech)" => reset_panel!(rules, tech),
        "Rules (Culture)" => reset_panel!(rules, culture),
        "Rules (City)" => reset_panel!(rules, city),
        "Climate" => reset_panel!(climate),
        _ => {}
    }
}

/// A handler implementation for the egui file dialog.
pub struct FileDialogHandler<'a> {
    pub events: &'a mut EventStruct,
    pub config: &'a mut AtlasSimConfig,
}

impl<'a> FileDialogHandler<'a> {
    pub fn new(events: &'a mut EventStruct, config: &'a mut AtlasSimConfig) -> Self {
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

    fn export(&mut self, path: &Path) {
        self.events.export_world_request = Some(path.into());
    }

    fn import(&mut self, path: &Path) {
        self.events.import_world_request = Some(path.into());
    }

    fn import_special(&mut self, path: &Path) {
        self.events.import_start_request = Some(path.into());
    }

    fn load_layer_data(&mut self, _path: &Path, _layer: MapDataLayer) {
        unreachable!()
    }

    fn save_layer_data(&mut self, _path: &Path, _layer: MapDataLayer) {
        unreachable!()
    }

    fn render_image(&mut self, _path: &Path, _layer: MapDataLayer) {
        unreachable!()
    }
}
