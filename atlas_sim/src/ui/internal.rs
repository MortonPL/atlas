use std::path::Path;

use atlas_lib::{
    base::ui::{FileDialogMode, HandleFileDialog, UiStateBase},
    config::{load_config, save_config},
    domain::map::MapDataLayer,
    egui_file,
};

use crate::{config::AtlasSimConfig, event::EventStruct, ui::AtlasSimUi};

use super::panel::MainPanelGeneral;

/// Set context for the file dialog to "importing world" and show it.
pub fn import_world_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::ImportSpecial;
}

/// Set context for the file dialog to "importing world state" and show it.
pub fn import_world_state_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::Import;
}

/// Set context for the file dialog to "exporting world state" and show it.
pub fn export_world_state_clicked(ui_base: &mut UiStateBase) {
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
pub fn reset_config_clicked(config: &mut AtlasSimConfig, ui_state: &mut AtlasSimUi) {
    *config = AtlasSimConfig::default();
    ui_state.current_panel = Box::<MainPanelGeneral>::default();
}

/// Reset a config from one panel to defaults, and reset relevant logic layers.
pub fn reset_panel_clicked(_config: &mut AtlasSimConfig, _ui_state: &AtlasSimUi, _events: &mut EventStruct) {
    // TODO
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
            Ok(data) => *self.config = data,
            Err(err) => self.events.error_window = Some(err.to_string()),
        }
    }

    fn save_config(&mut self, path: &Path) {
        if let Err(err) = save_config(self.config, path) {
            self.events.error_window = Some(err.to_string());
        }
    }

    fn export(&mut self, path: &Path) {
        self.events.export_state_request = Some(path.into());
    }

    fn import(&mut self, path: &Path) {
        self.events.import_state_request = Some(path.into());
    }

    fn import_special(&mut self, path: &Path) {
        self.events.import_world_request = Some(path.into());
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
