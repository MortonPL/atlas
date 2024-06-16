use std::path::Path;

use atlas_lib::{
    bevy::{app::AppExit, ecs as bevy_ecs, prelude::*},
    bevy_egui::egui::{self, Context, RichText, Ui},
    config::{load_config, load_image, load_image_grey, save_config},
    domain::map::MapDataLayer,
    egui_file,
    ui::{
        button_action,
        plugin_base::{
            adjust_viewport, ErrorWindowHandler, FileDialogMode, HandleErrorWindow, HandleFileDialog,
            UiStateBase, SIDEBAR_MIN_WIDTH, SIDEBAR_WIDTH,
        },
        sidebar::{SidebarControl, SidebarEnumDropdown},
        window,
    },
};

use crate::{
    config::AtlasGenConfig,
    event::EventStruct,
    ui::panel::{
        MainPanelClimate, MainPanelContinents, MainPanelGeneral, MainPanelPrecipitation, MainPanelResources,
        MainPanelTemperature, MainPanelTopography, SidebarPanel,
    },
};

/// Struct that contains only the UI-related state (no logic).
#[derive(Default, Resource)]
pub struct UiState {
    /// Currently viewed map layer.
    current_layer: MapDataLayer,
}

/// Extra struct (alongside [`UiState`]) that holds the current sidebar panel.
#[derive(Default, Resource)]
pub struct UiStatePanel {
    /// Currently viewed sidebar panel.
    current_panel: Box<dyn SidebarPanel + Sync + Send>,
}

/// Add the entire UI.
pub fn create_ui(
    ctx: &Context,
    config: &mut AtlasGenConfig,
    ui_state: &mut UiState,
    ui_base: &mut UiStateBase,
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
            create_layer_view_settings(ui, ui_state, events);
            ui.separator();
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
    handle_about(ctx, "Atlas Map Generator", &mut ui_base.about_open);
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
    config: &mut AtlasGenConfig,
    ui_state: &mut UiState,
    ui_base: &mut UiStateBase,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
    exit: &mut EventWriter<AppExit>,
) {
    ui.vertical(|ui| {
        ui.heading(egui::RichText::new("Atlas Map Generator").size(24.0));
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                button_action(ui, "Import World", || import_world_clicked(ui_base));
                button_action(ui, "Export World", || export_world_clicked(ui_base));
                button_action(ui, "Exit", || exit.send(AppExit));
            });
            ui.menu_button("Edit", |ui| {
                button_action(ui, "Reset Current Panel", || {
                    reset_panel_clicked(config, ui_panel, events)
                });
            });
            ui.menu_button("Config", |ui| {
                button_action(ui, "Save Configuration", || save_config_clicked(ui_base));
                button_action(ui, "Load Configuration", || load_config_clicked(ui_base));
                button_action(ui, "Reset Configuration", || {
                    reset_config_clicked(config, ui_panel, events)
                });
            });
            ui.menu_button("Layer", |ui| {
                button_action(ui, "Load Layer Data", || load_layer_clicked(ui_state, ui_base));
                button_action(ui, "Save Layer Data", || save_layer_clicked(ui_state, ui_base));
                button_action(ui, "Clear Layer Data", || clear_layer_clicked(ui_state, events));
                button_action(ui, "Render Layer Image", || {
                    render_layer_clicked(ui_state, ui_base)
                });
            });
            ui.menu_button("Help", |ui| {
                button_action(ui, "About", || ui_base.about_open = true);
            })
        });
    });
}

/// Create sidebar settings for the layer display.
fn create_layer_view_settings(ui: &mut Ui, ui_state: &mut UiState, events: &mut EventStruct) {
    ui.vertical(|ui| {
        let old = ui_state.current_layer;
        // Layer visibility dropdown.
        // NOTE: `ui.horizontal_wrapped()` respects `ui.end_row()` used internally by a `SidebarControl`.
        ui.horizontal_wrapped(|ui| {
            let selection =
                SidebarEnumDropdown::new(ui, "Viewed Layer", &mut ui_state.current_layer).show(None);
            SidebarEnumDropdown::post_show(selection, &mut ui_state.current_layer);
            // Trigger layer change event as needed.
            if old != ui_state.current_layer {
                events.viewed_layer_changed = Some(ui_state.current_layer);
            }
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
            changed |= button_action(ui, "Continents", || {
                ui_panel.current_panel = Box::<MainPanelContinents>::default();
                true
            });
            changed |= button_action(ui, "Topography", || {
                ui_panel.current_panel = Box::<MainPanelTopography>::default();
                true
            });
            changed |= button_action(ui, "Temperature", || {
                ui_panel.current_panel = Box::<MainPanelTemperature>::default();
                true
            });
        });
        egui::menu::bar(ui, |ui| {
            changed |= button_action(ui, "Precipitation", || {
                ui_panel.current_panel = Box::<MainPanelPrecipitation>::default();
                true
            });
            changed |= button_action(ui, "Climate", || {
                ui_panel.current_panel = Box::<MainPanelClimate>::default();
                true
            });
            /* TODO
            changed |= button_action(ui, "Resources", || {
                ui_panel.current_panel = Box::<MainPanelResources>::default();
                true
            });
            */
        });
        if changed {
            let layer = ui_panel.current_panel.get_layer();
            events.viewed_layer_changed = Some(layer);
            ui_state.current_layer = layer;
        }
    });
}

/// Create the current panel.
fn create_current_panel(
    ui: &mut Ui,
    config: &mut AtlasGenConfig,
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

/// Set context for the file dialog to "importing world" and show it.
fn import_world_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::Import;
}

/// Set context for the file dialog to "exporting world" and show it.
fn export_world_clicked(ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::Export;
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
fn reset_config_clicked(config: &mut AtlasGenConfig, ui_panel: &mut UiStatePanel, events: &mut EventStruct) {
    *config = AtlasGenConfig::default();
    ui_panel.current_panel = default();
    events.world_model_changed = Some(());
}

/// Reset a config from one panel to defaults, and reset relevant logic layers.
fn reset_panel_clicked(config: &mut AtlasGenConfig, ui_panel: &UiStatePanel, events: &mut EventStruct) {
    match ui_panel.current_panel.get_layer() {
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
fn load_layer_clicked(ui_state: &mut UiState, ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::open_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::LoadData(ui_state.current_layer);
}

// Set context for the file dialog to "saving layer" and show it.
fn save_layer_clicked(ui_state: &mut UiState, ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::SaveData(ui_state.current_layer);
}

// Set context for the file dialog to "rendering layer" and show it.
fn render_layer_clicked(ui_state: &mut UiState, ui_base: &mut UiStateBase) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = FileDialogMode::RenderImage(ui_state.current_layer);
}

// Clear layer data.
fn clear_layer_clicked(ui_state: &mut UiState, events: &mut EventStruct) {
    events.clear_layer_request = Some(ui_state.current_layer);
}

/// A handler implementation for the egui file dialog.
pub struct FileDialogHandler<'a> {
    pub events: &'a mut EventStruct,
    pub config: &'a mut AtlasGenConfig,
}

impl<'a> FileDialogHandler<'a> {
    fn new(events: &'a mut EventStruct, config: &'a mut AtlasGenConfig) -> Self {
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
