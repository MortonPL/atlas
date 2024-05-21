use std::path::Path;

use bevy::{
    app::AppExit, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*
};
use bevy_egui::egui::{self, Align2, Context, Ui};

use atlas_lib::ui::{
    button_action,
    sidebar::{SidebarControl, SidebarEnumDropdown},
};

use crate::{
    config::{load_config, load_image, load_image_grey, save_config, AtlasGenConfig},
    event::EventStruct,
    map::MapDataLayer,
    ui::panel::SidebarPanel,
};

use super::panel::{
    MainPanelClimate, MainPanelContinents, MainPanelGeneral, MainPanelPrecipitation, MainPanelResources,
    MainPanelTemperature, MainPanelTopography,
};

/// Default sidebar width in points. Should be greater or equal to [`SIDEBAR_MIN_WIDTH`].
const SIDEBAR_WIDTH: f32 = 360.0;
/// Minimum sidebar width in points.
const SIDEBAR_MIN_WIDTH: f32 = 360.0;

/// Minimum camera zoom as Z in world space (bad idea?).
const MIN_CAMERA_ZOOM: f32 = 1.0;
/// Maximum camera zoom as Z in world space (bad idea?).
const MAX_CAMERA_ZOOM: f32 = 15.0;
/// Mutliplier to scroll value.
const CAMERA_ZOOM_SPEED: f32 = 0.05;

/// Mode of operation for the generic file dialog.
#[derive(Clone, Copy, Default)]
pub enum FileDialogMode {
    /// Save generator configuration to TOML file.
    #[default]
    SaveConfig,
    /// Load generator configuration to TOML file.
    LoadConfig,
    /// Save this layer data to PNG file.
    SaveData(MapDataLayer),
    /// Load this layer data from PNG file.
    LoadData(MapDataLayer),
    /// Render this layer to a PNG file.
    RenderImage(MapDataLayer),
    /// Export all layers.
    Export,
}

/// Struct that contains only the UI-related state (no logic).
#[derive(Default, Resource)]
pub struct UiState {
    /// Size (in pixels) of the viewport, AKA window size - sidebar size.
    pub viewport_size: bevy::prelude::Vec2,
    /// All purpose file dialog. Some if open, None if closed.
    file_dialog: Option<egui_file::FileDialog>,
    /// File dialog mode of operation. See [`FileDialogMode`].
    file_dialog_mode: FileDialogMode,
    /// Currently viewed map layer.
    current_layer: MapDataLayer,
    /// Is the error popup window open?
    error_window_open: bool,
    /// Current error message.
    error_message: String,
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
    mut config: ResMut<AtlasGenConfig>,
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
            create_sidebar_head(ui, &mut config, ui_state, ui_panel, events, exit);
            ui.separator(); // HACK: Do not delete. The panel won't resize without it. Known issue.
            create_layer_view_settings(ui, ui_state, events);
            ui.separator();
            create_panel_tabs(ui, ui_state, ui_panel, events);
            ui.separator();
            create_current_panel(ui, &mut config, ui_state, ui_panel, events);
            // We've finished drawing the sidebar. Its size is now established
            // and we can calculate the viewport size.
            adjust_viewport(ui, ui_state);
        });
    handle_file_dialog(ctx, &mut config, ui_state, ui_panel, events);
    handle_error_window(ctx, ui_state, events);
}

/// Handle camera movement/zoom inputs.
pub fn handle_camera(
    kb: Res<Input<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    window: &Window,
    camera: &mut Mut<Transform>,
    ui_state: &UiState,
) {
    let mut scroll = 0.0;
    // Don't scroll with mouse if it's not inside the viewport.
    if let Some(event) = mouse_wheel.read().next() {
        if let Some(cursor) = window.cursor_position() {
            if (cursor[0] <= ui_state.viewport_size[0]) && (cursor[1] <= ui_state.viewport_size[1]) {
                match event.unit {
                    MouseScrollUnit::Line => scroll = event.y,
                    MouseScrollUnit::Pixel => scroll = event.y * 2.0,
                }
            }
        }
    }
    let mut z = camera.translation.z;
    // Zoom in.
    if kb.pressed(KeyCode::Equals) || (scroll > 0.0) {
        z *= 1.0f32 - CAMERA_ZOOM_SPEED * (1.0 + scroll);
    // Zoom out.
    } else if kb.pressed(KeyCode::Minus) || (scroll < 0.0) {
        z *= 1.0f32 + CAMERA_ZOOM_SPEED * (1.0 - scroll);
    }
    // Apply new Z to the camera.
    camera.translation.z = z.clamp(MIN_CAMERA_ZOOM, MAX_CAMERA_ZOOM);
}

/// Create the top part of the sidebar with configuration S/L.
fn create_sidebar_head(
    ui: &mut Ui,
    config: &mut ResMut<AtlasGenConfig>,
    ui_state: &mut UiState,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
    exit: &mut EventWriter<AppExit>,
) {
    ui.vertical(|ui| {
        ui.heading(egui::RichText::new("Atlas Map Generator").size(24.0));
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                button_action(ui, "Import World", || { /* TODO */ });
                button_action(ui, "Export World", || export_world_clicked(ui_state));
                button_action(ui, "Exit", || exit.send(AppExit));
            });
            ui.menu_button("Edit", |ui| {
                button_action(ui, "Reset Current Panel", || {
                    reset_panel_clicked(config, ui_panel, events)
                });
            });
            ui.menu_button("Config", |ui| {
                button_action(ui, "Save Configuration", || save_config_clicked(ui_state));
                button_action(ui, "Load Configuration", || load_config_clicked(ui_state));
                button_action(ui, "Reset Configuration", || {
                    reset_config_clicked(config, ui_panel, events)
                });
            });
            ui.menu_button("Layer", |ui| {
                button_action(ui, "Load Layer Data", || load_layer_clicked(ui_state));
                button_action(ui, "Save Layer Data", || save_layer_clicked(ui_state));
                button_action(ui, "Clear Layer Data", || clear_layer_clicked(ui_state, events));
                button_action(ui, "Render Layer Image", || render_layer_clicked(ui_state));
            });
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
                ui_panel.current_panel = Box::new(MainPanelGeneral::default());
                true
            });
            changed |= button_action(ui, "Continents", || {
                ui_panel.current_panel = Box::new(MainPanelContinents::default());
                true
            });
            changed |= button_action(ui, "Topography", || {
                ui_panel.current_panel = Box::new(MainPanelTopography::default());
                true
            });
            changed |= button_action(ui, "Temperature", || {
                ui_panel.current_panel = Box::new(MainPanelTemperature::default());
                true
            });
        });
        egui::menu::bar(ui, |ui| {
            changed |= button_action(ui, "Precipitation", || {
                ui_panel.current_panel = Box::new(MainPanelPrecipitation::default());
                true
            });
            changed |= button_action(ui, "Climate", || {
                ui_panel.current_panel = Box::new(MainPanelClimate::default());
                true
            });
            changed |= button_action(ui, "Resources", || {
                ui_panel.current_panel = Box::new(MainPanelResources::default());
                true
            });
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

/// Adjust viewport size to not overlap the sidebar.
fn adjust_viewport(ui: &mut Ui, ui_state: &mut UiState) {
    let window_size = ui.clip_rect().size();
    let ui_size = ui.max_rect().size();
    ui_state.viewport_size = Vec2 {
        x: (window_size.x - ui_size.x).max(1.0),
        y: window_size.y.max(1.0),
    };
}

/// Handle the file dialog window if it is open. Perform configuration S/L if the user selected a file.
fn handle_file_dialog(
    ctx: &Context,
    config: &mut ResMut<AtlasGenConfig>,
    ui_state: &mut UiState,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    let mode = ui_state.file_dialog_mode;
    if let Some(file_dialog) = &mut ui_state.file_dialog {
        if !file_dialog.show(ctx).selected() {
            return;
        }
        if let Some(path) = file_dialog.path() {
            match mode {
                FileDialogMode::LoadConfig => file_dialog_load_config(path, config, ui_panel, events),
                FileDialogMode::SaveConfig => file_dialog_save_config(path, config, events),
                FileDialogMode::LoadData(layer) => file_dialog_load_data(path, layer, config, events),
                FileDialogMode::SaveData(layer) => file_dialog_save_data(path, layer, events),
                FileDialogMode::RenderImage(layer) => file_dialog_render_image(path, layer, events),
                FileDialogMode::Export => file_dialog_export(path, events),
            };
        }
        ui_state.file_dialog = None;
    }
}

/// Show the error window if there's an error.
fn handle_error_window(ctx: &Context, ui_state: &mut UiState, events: &mut EventStruct) {
    if let Some(error) = events.error_window.take() {
        ui_state.error_message = error;
        ui_state.error_window_open = true;
    }
    egui::Window::new("An error has occured")
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .open(&mut ui_state.error_window_open)
        .collapsible(false)
        .movable(false)
        .show(ctx, |ui| {
            ui.label(&ui_state.error_message);
        });
}

/// Set context for the file dialog to "exporting world" and show it.
fn export_world_clicked(ui_state: &mut UiState) {
    let mut file_picker = egui_file::FileDialog::select_folder(None);
    file_picker.open();
    ui_state.file_dialog = Some(file_picker);
    ui_state.file_dialog_mode = FileDialogMode::Export;
}

/// Set context for the file dialog to "saving config" and show it.
fn save_config_clicked(ui_state: &mut UiState) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_state.file_dialog = Some(file_picker);
    ui_state.file_dialog_mode = FileDialogMode::SaveConfig;
}

/// Set context for the file dialog to "loading config" and show it.
fn load_config_clicked(ui_state: &mut UiState) {
    let mut file_picker = egui_file::FileDialog::open_file(None);
    file_picker.open();
    ui_state.file_dialog = Some(file_picker);
    ui_state.file_dialog_mode = FileDialogMode::LoadConfig;
}

/// Reset generator config to defaults.
fn reset_config_clicked(
    config: &mut ResMut<AtlasGenConfig>,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    **config = AtlasGenConfig::default();
    ui_panel.current_panel = default();
    events.world_model_changed = Some(config.general.world_model.clone());
}

/// Reset a config from one panel to defaults, and reset relevant logic layers.
fn reset_panel_clicked(
    config: &mut ResMut<AtlasGenConfig>,
    ui_panel: &UiStatePanel,
    events: &mut EventStruct,
) {
    match ui_panel.current_panel.get_layer() {
        MapDataLayer::Preview => {
            config.general = default();
            events.world_model_changed = Some(config.general.world_model.clone());
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
fn load_layer_clicked(ui_state: &mut UiState) {
    let mut file_picker = egui_file::FileDialog::open_file(None);
    file_picker.open();
    ui_state.file_dialog = Some(file_picker);
    ui_state.file_dialog_mode = FileDialogMode::LoadData(ui_state.current_layer);
}

// Set context for the file dialog to "saving layer" and show it.
fn save_layer_clicked(ui_state: &mut UiState) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_state.file_dialog = Some(file_picker);
    ui_state.file_dialog_mode = FileDialogMode::SaveData(ui_state.current_layer);
}

// Set context for the file dialog to "rendering layer" and show it.
fn render_layer_clicked(ui_state: &mut UiState) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_state.file_dialog = Some(file_picker);
    ui_state.file_dialog_mode = FileDialogMode::RenderImage(ui_state.current_layer);
}

// Clear layer data.
fn clear_layer_clicked(ui_state: &mut UiState, events: &mut EventStruct) {
    events.clear_layer_request = Some(ui_state.current_layer);
}

/// Load and overwrite configuration from a TOML file.
fn file_dialog_load_config(
    path: &Path,
    config: &mut ResMut<AtlasGenConfig>,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    match load_config(path) {
        Ok(data) => {
            **config = data;
            events.world_model_changed = Some(config.general.world_model.clone());
            ui_panel.current_panel = default();
        },
        Err(err) => events.error_window = Some(err.to_string()),
    }
}

/// Save current configuration to a TOML file.
fn file_dialog_save_config(path: &Path, config: &mut ResMut<AtlasGenConfig>, events: &mut EventStruct) {
    if let Err(err) = save_config(config, path) {
        events.error_window = Some(err.to_string());
    }
}

/// Load a layer image and send an event.
fn file_dialog_load_data(
    path: &Path,
    layer: MapDataLayer,
    config: &mut ResMut<AtlasGenConfig>,
    events: &mut EventStruct,
) {
    let (width, height) = config.general.world_model.get_dimensions();
    let result = match layer {
        MapDataLayer::Preview => load_image(path, width, height),
        _ => load_image_grey(path, width, height),
    };
    match result {
        Ok(data) => events.load_layer_request = Some((layer, data)),
        Err(err) => events.error_window = Some(err.to_string()),
    };
}

/// Send an event to save a layer image.
fn file_dialog_save_data(path: &Path, layer: MapDataLayer, events: &mut EventStruct) {
    events.save_layer_request = Some((layer, path.into()));
}

/// Send an event to render a layer image.
fn file_dialog_render_image(path: &Path, layer: MapDataLayer, events: &mut EventStruct) {
    events.render_layer_request = Some((layer, path.into()));
}

/// Send an event to export all layers.
fn file_dialog_export(path: &Path, events: &mut EventStruct) {
    events.export_world_request = Some(path.into());
}
