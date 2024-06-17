use std::path::Path;

use atlas_lib::{
    base::ui::{FileDialogMode, HandleFileDialog, UiStateBase},
    bevy::prelude::*,
    config::{load_config, load_image, load_image_grey, save_config},
    domain::map::MapDataLayer,
    egui_file,
};

use crate::{config::AtlasGenConfig, event::EventStruct, ui::AtlasGenUi};

use super::panel::MainPanelGeneral;

/// Set context for the file dialog to "importing world" and show it.
pub fn import_world_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::Import;
}

/// Set context for the file dialog to "exporting world" and show it.
pub fn export_world_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::Export;
}

/// Set context for the file dialog to "saving config" and show it.
pub fn save_config_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::SaveConfig;
}

/// Set context for the file dialog to "loading config" and show it.
pub fn load_config_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::open_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::LoadConfig;
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

/// Reset a config from one panel to defaults, and reset relevant logic layers.
pub fn reset_panel_clicked(config: &mut AtlasGenConfig, ui_state: &mut AtlasGenUi, events: &mut EventStruct) {
    match ui_state.current_panel.get_layer() {
        MapDataLayer::Preview => {
            config.general = default();
            events.world_model_changed = Some(());
        }
        MapDataLayer::Continents => {
            config.continents = default();
            events.generate_request = Some((MapDataLayer::Continents, true));
        }
        MapDataLayer::Topography => {
            config.topography = default();
            events.generate_request = Some((MapDataLayer::Topography, true));
        }
        MapDataLayer::Temperature => {
            config.temperature = default();
            events.generate_request = Some((MapDataLayer::Temperature, true));
        }
        MapDataLayer::Precipitation => {
            config.precipitation = default();
            events.generate_request = Some((MapDataLayer::Precipitation, true));
        }
        MapDataLayer::Climate => {
            config.climate = default();
            events.generate_request = Some((MapDataLayer::Climate, false));
        }
        MapDataLayer::Resources => {
            config.resources = default();
            events.generate_request = Some((MapDataLayer::Resources, false));
        }
        _ => unreachable!(),
    }
}

// Set context for the file dialog to "loading layer" and show it.
pub fn load_layer_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::open_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::LoadData(ui_base.current_layer);
}

// Set context for the file dialog to "saving layer" and show it.
pub fn save_layer_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::SaveData(ui_base.current_layer);
}

// Set context for the file dialog to "rendering layer" and show it.
pub fn render_layer_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::RenderImage(ui_base.current_layer);
}

// Clear layer data.
pub fn clear_layer_clicked(ui_base: &mut UiStateBase, events: &mut EventStruct) {
    events.clear_layer_request = Some(ui_base.current_layer);
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
