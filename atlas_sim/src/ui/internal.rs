use std::path::Path;

use atlas_lib::{
    bevy::{app::AppExit, ecs as bevy_ecs, prelude::*},
    bevy_egui::egui::{self, Context, RichText, Ui},
    config::{load_config, save_config},
    domain::map::MapDataLayer,
    egui_file,
    ui::{
        button_action, button_action_enabled,
        plugin_base::{
            adjust_viewport, ErrorWindowHandler, FileDialogMode, HandleErrorWindow, HandleFileDialog,
            UiStateBase, SIDEBAR_MIN_WIDTH, SIDEBAR_WIDTH,
        },
        window,
    },
};

use crate::{config::AtlasSimConfig, event::EventStruct, ui::panel::SidebarPanel};

use super::panel::MainPanelGeneral;

#[derive(Resource)]
pub struct UiState {
    /// Is the simulations not running yet? Can we still make changes to the configuration?
    is_not_running: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self { is_not_running: true }
    }
}

/// Extra struct (alongside [`UiState`]) that holds the current sidebar panel.
#[derive(Default, Resource)]
pub struct UiStatePanel {
    /// Currently viewed sidebar panel.
    current_panel: Box<dyn SidebarPanel + Sync + Send>,
}

pub fn create_ui(
    ctx: &mut Context,
    config: &mut AtlasSimConfig,
    ui_base: &mut UiStateBase,
    ui_state: &mut UiState,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
    exit: &mut EventWriter<AppExit>,
) {
    // The UI is a resizeable sidebar fixed to the right window border.
    // __________________
    // | Sidebar Head   |  <-- Title, menu bar.
    // |----------------|
    // | Layer View     |  <-- Layer dropdown.
    // |----------------|
    // | Panel Selection|  <-- Pseudo "tabs" for panels.
    // |----------------|
    // | Panel-specific |  <-- Panel displaying current stage settings.
    // |________________|
    egui::SidePanel::right("ui_root")
        .min_width(SIDEBAR_MIN_WIDTH)
        .default_width(SIDEBAR_WIDTH)
        .show(ctx, |ui| {
            create_sidebar_head(ui, config, ui_state, ui_base, ui_panel, events, exit);
            ui.separator(); // HACK: Do not delete. The panel won't resize without it. Known issue.
            // create_layer_view_settings(ui, ui_state, events);
            // ui.separator();
            create_panel_tabs(ui, ui_state, ui_panel, events);
            ui.separator();
            create_current_panel(ui, config, ui_state, ui_panel, events);
            // We've finished drawing the sidebar. Its size is now established
            // and we can calculate the viewport size.
            adjust_viewport(ui, ui_base);
        });
    // Handle file dialog.
    FileDialogHandler::new(events, config).handle(ctx, ui_base);
    // Handle error window.
    if let Some(error) = events.error_window.take() {
        ui_base.error_message = error;
        ui_base.error_window_open = true;
    }
    ErrorWindowHandler::new().handle(ctx, ui_base);
    // Handle about window.
    handle_about(ctx, "Atlas History Simulator", &mut ui_base.about_open);
}

/// Handle displaying the "About" window.
fn handle_about(ctx: &Context, name: impl Into<RichText>, open: &mut bool) {
    window(ctx, "About", open, |ui| {
        ui.heading(name);
        ui.label(env!("CARGO_PKG_DESCRIPTION"));
        ui.separator();
        ui.label(format!("Authors: {}", env!("CARGO_PKG_AUTHORS")));
        ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
        ui.label(format!("Home Page: {}", env!("CARGO_PKG_HOMEPAGE")));
    });
}

/// Create the top part of the sidebar with configuration S/L.
fn create_sidebar_head(
    ui: &mut Ui,
    config: &mut AtlasSimConfig,
    ui_state: &mut UiState,
    ui_base: &mut UiStateBase,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
    exit: &mut EventWriter<AppExit>,
) {
    ui.vertical(|ui| {
        ui.heading(egui::RichText::new("Atlas History Simulator").size(24.0));
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                button_action_enabled(ui, "Import Generated World", ui_state.is_not_running, || {
                    import_world_clicked(ui_base)
                });
                button_action(ui, "Export Current World", || export_world_clicked(ui_base));
                button_action(ui, "Exit", || exit.send(AppExit));
            });
            ui.menu_button("Edit", |ui| {
                button_action(ui, "Reset Current Panel", || {
                    reset_panel_clicked(config, ui_panel, events)
                });
            });
            ui.menu_button("Config", |ui| {
                button_action(ui, "Save Configuration", || save_config_clicked(ui_base));
                button_action_enabled(ui, "Load Configuration", ui_state.is_not_running, || {
                    load_config_clicked(ui_base)
                });
                button_action_enabled(ui, "Reset Configuration", ui_state.is_not_running, || {
                    reset_config_clicked(config, ui_panel)
                });
            });
            ui.menu_button("Help", |ui| {
                button_action(ui, "About", || ui_base.about_open = true);
            })
        });
    });
}

/// Create tabs for switching panels.
fn create_panel_tabs(
    ui: &mut Ui,
    ui_state: &mut UiState,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    ui.vertical(|ui| {
        let mut changed = false;
        egui::menu::bar(ui, |ui| {
            changed |= button_action(ui, "General", || {
                ui_panel.current_panel = Box::<MainPanelGeneral>::default();
                true
            });
            changed |= button_action(ui, "Tab 2", || {
                ui_panel.current_panel = Box::<MainPanelGeneral>::default();
                true
            });
            changed |= button_action(ui, "Tab 3", || {
                ui_panel.current_panel = Box::<MainPanelGeneral>::default();
                true
            });
            changed |= button_action(ui, "Tab 4", || {
                ui_panel.current_panel = Box::<MainPanelGeneral>::default();
                true
            });
        });
        egui::menu::bar(ui, |ui| {
            changed |= button_action(ui, "Tab 5", || {
                ui_panel.current_panel = Box::<MainPanelGeneral>::default();
                true
            });
            changed |= button_action(ui, "Tab 6", || {
                ui_panel.current_panel = Box::<MainPanelGeneral>::default();
                true
            });
            /* TODO
            changed |= button_action(ui, "Tab 7", || {
                ui_panel.current_panel = Box::<MainPanelGeneral>::default();
                true
            });
            */
        });
        if changed {
            // TODO
        }
    });
}

/// Create the current panel.
fn create_current_panel(
    ui: &mut Ui,
    config: &mut AtlasSimConfig,
    ui_state: &mut UiState,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    // Panel heading.
    ui.heading(ui_panel.current_panel.get_heading());
    // Panel inner.
    egui::ScrollArea::both().show(ui, |ui| {
        ui_panel.current_panel.show(ui, config, ui_state, events);
        ui.separator(); // HACK! Again! Without it the scroll area isn't greedy.
    });
}

/// Set context for the file dialog to "exporting world" and show it.
fn import_world_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::ImportGen;
}

/// Set context for the file dialog to "exporting world" and show it.
fn export_world_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::ExportGen;
}

/// Set context for the file dialog to "saving config" and show it.
fn save_config_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::SaveConfig;
}

/// Set context for the file dialog to "loading config" and show it.
fn load_config_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::open_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::LoadConfig;
}

/// Reset generator config to defaults.
fn reset_config_clicked(config: &mut AtlasSimConfig, ui_panel: &mut UiStatePanel) {
    *config = AtlasSimConfig::default();
    ui_panel.current_panel = default();
}

/// Reset a config from one panel to defaults, and reset relevant logic layers.
fn reset_panel_clicked(_config: &mut AtlasSimConfig, _ui_panel: &UiStatePanel, _events: &mut EventStruct) {
    // TODO
}

/// A handler implementation for the egui file dialog.
pub struct FileDialogHandler<'a> {
    pub events: &'a mut EventStruct,
    pub config: &'a mut AtlasSimConfig,
}

impl<'a> FileDialogHandler<'a> {
    fn new(events: &'a mut EventStruct, config: &'a mut AtlasSimConfig) -> Self {
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

    fn export_gen(&mut self, path: &Path) {
        self.events.export_world_request = Some(path.into());
    }

    fn import_gen(&mut self, path: &Path) {
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
