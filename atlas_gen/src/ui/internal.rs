use std::path::Path;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_egui::egui::{self, Context, Ui};

use atlas_lib::{
    ui::{
        button, button_action,
        sidebar::{SidebarControl, SidebarEnumDropdown},
        UiEditableEnum,
    },
    update_enum,
};

use crate::{
    config::{load_config, load_image, save_config, GeneratorConfig},
    event::EventStruct,
    map::ViewedMapLayer,
    ui::panel::{MainPanel, MainPanelTransition},
};

/// Default sidebar width in points. Should be greater or equal to [SIDEBAR_MIN_WIDTH].
const SIDEBAR_WIDTH: f32 = 420.0;
/// Minimal sidebar width in points.
const SIDEBAR_MIN_WIDTH: f32 = 420.0;

/// Minimal camera zoom as Z in world space (bad idea?).
const MIN_CAMERA_ZOOM: f32 = 2.0;
/// Minimal camera zoom as Z in world space (bad idea?).
const MAX_CAMERA_ZOOM: f32 = 5.0;
/// Mutliplier to current Z.
const CAMERA_ZOOM_SPEED: f32 = 0.05;

#[derive(Clone, Copy, Default)]
enum FileDialogMode {
    /// Save generator configuration to TOML file.
    #[default]
    SaveConfig,
    /// Load generator configuration to TOML file.
    LoadConfig,
    /// Save generation layer output to PNG file.
    SaveImage(ViewedMapLayer),
    /// Load generation layer output from PNG file.
    LoadImage(ViewedMapLayer),
}

/// Struct that contains only the UI-related state (no logic).
#[derive(Default, Resource)]
pub struct UiState {
    pub viewport_size: bevy::prelude::Vec2,
    file_dialog: Option<egui_file::FileDialog>,
    file_dialog_mode: FileDialogMode,
    current_layer: ViewedMapLayer,
}

/// Currently viewed sidebar panel.
#[derive(Default, Resource)]
pub struct UiStatePanel {
    current_panel: Box<dyn MainPanel + Sync + Send>,
}

/// Add the entire UI.
pub fn create_ui(
    ctx: &Context,
    mut config: ResMut<GeneratorConfig>,
    ui_state: &mut UiState,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    // The UI is a resizeable sidebar fixed to the right window border.
    // __________________
    // | Sidebar Head   |  <-- Title, "Save"/"Load"/"Reset" buttons.
    // |----------------|
    // | Layer View     |  <-- Layer dropdown and other visibility settings.
    // |----------------|
    // | Panel-specific |  <-- Panel displaying current stage settings
    // |                |      and "Previous"/"Next" buttons.
    // |  Prev || Next  |
    // |________________|
    egui::SidePanel::right("ui_root")
        .min_width(SIDEBAR_MIN_WIDTH)
        .default_width(SIDEBAR_WIDTH)
        .show(ctx, |ui| {
            create_sidebar_head(ui, &mut config, ui_state, ui_panel, events);
            ui.separator(); // HACK: Do not delete. The panel won't resize without it. Known issue.
            create_layer_view_settings(ui, ui_state, events);
            ui.separator();
            create_current_panel(ui, &mut config, ui_state, ui_panel, events);
            adjust_viewport(ui, ui_state);
        });
    handle_file_dialog(ctx, &mut config, ui_state, ui_panel, events);
}

/// Handle camera movement/zoom inputs.
pub fn handle_camera(
    kb: Res<Input<KeyCode>>,
    mut mouse: EventReader<MouseWheel>,
    camera: &mut Mut<Transform>,
) {
    let scroll = if let Some(event) = mouse.read().next() {
        match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y * 2.0,
        }
    } else {
        0.0
    };
    let mut z = camera.translation.z;
    // Zoom in.
    if kb.pressed(KeyCode::Equals) || (scroll > 0.0) {
        z *= 1.0f32 - CAMERA_ZOOM_SPEED * (1.0 + scroll);
    // Zoom out.
    } else if kb.pressed(KeyCode::Minus) || (scroll < 0.0) {
        z *= 1.0f32 + CAMERA_ZOOM_SPEED * (1.0 - scroll);
    }
    camera.translation.z = z.clamp(MIN_CAMERA_ZOOM, MAX_CAMERA_ZOOM);
}

/// Add a top bar with configuration S/L.
fn create_sidebar_head(
    ui: &mut Ui,
    config: &mut ResMut<GeneratorConfig>,
    ui_state: &mut UiState,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    ui.vertical(|ui| {
        ui.heading(egui::RichText::new("Atlas Map Generator").size(24.0));
        ui.horizontal(|ui| {
            button_action(ui, "Save Config", || save_config_clicked(ui_state));
            button_action(ui, "Load Config", || load_config_clicked(ui_state));
            button_action(ui, "Reset Config", || {
                reset_config_clicked(config, ui_panel, events)
            });
        });
    });
}

/// Create sidebar settings for the layer display.
fn create_layer_view_settings(ui: &mut Ui, ui_state: &mut UiState, events: &mut EventStruct) {
    ui.horizontal(|ui| {
        let old = ui_state.current_layer;
        let selection = SidebarEnumDropdown::new(ui, "Viewed Layer", &mut ui_state.current_layer).show(None);
        update_enum!(ui_state.current_layer, selection);
        if old != ui_state.current_layer {
            events.viewed_layer_changed = Some(ui_state.current_layer);
        }
        if button(ui, "Load Layer") {
            let mut file_picker = egui_file::FileDialog::open_file(None);
            file_picker.open();
            ui_state.file_dialog = Some(file_picker);
            ui_state.file_dialog_mode = FileDialogMode::LoadImage(ui_state.current_layer);
        }
        if button(ui, "Save Layer") {
            let mut file_picker = egui_file::FileDialog::save_file(None);
            file_picker.open();
            ui_state.file_dialog = Some(file_picker);
            ui_state.file_dialog_mode = FileDialogMode::SaveImage(ui_state.current_layer);
        }
        if button(ui, "Reset Layer") {
            events.reset_layer_request = Some(ui_state.current_layer);
        }
    });
}

/// Create the current panel.
fn create_current_panel(
    ui: &mut Ui,
    config: &mut GeneratorConfig,
    ui_state: &mut UiState,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    // Panel heading and content.
    ui.heading(ui_panel.current_panel.get_heading());
    egui::ScrollArea::both().show(ui, |ui| ui_panel.current_panel.show(ui, config, ui_state, events));
    // Previous/Next buttons and panel transitioning.
    ui.separator();
    ui.horizontal(|ui| {
        let transition = match (button(ui, "Previous"), button(ui, "Next")) {
            (true, _) => MainPanelTransition::Previous,
            (false, true) => MainPanelTransition::Next,
            _ => MainPanelTransition::None,
        };
        ui_panel.current_panel = ui_panel.current_panel.transition(transition);
        if transition != MainPanelTransition::None {
            let layer = ui_panel.current_panel.get_layer();
            events.viewed_layer_changed = Some(layer);
            ui_state.current_layer = layer;
        }
    });
}

/// Adjust viewport size to not overlap the sidebar.
fn adjust_viewport(ui: &mut Ui, ui_state: &mut UiState) {
    let window_size = ui.clip_rect().size();
    let ui_size = ui.max_rect().size();
    ui_state.viewport_size = Vec2 {
        x: window_size.x - ui_size.x,
        y: window_size.y,
    };
}

/// Handle the file dialog window if it is open. Perform configuration S/L if the user selected a file.
fn handle_file_dialog(
    ctx: &Context,
    config: &mut ResMut<GeneratorConfig>,
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
                FileDialogMode::LoadConfig => file_dialog_load_config(path, config, ui_panel, events), // TODO error handling
                FileDialogMode::SaveConfig => file_dialog_save_config(path, config), // TODO error handling
                FileDialogMode::LoadImage(layer) => file_dialog_load_image(path, layer, config, events), // TODO error handling
                FileDialogMode::SaveImage(layer) => file_dialog_save_image(path, layer, events), // TODO error handling
            };
        }
        ui_state.file_dialog = None;
    }
}

/// Set context for the file dialog to "saving" and show it.
fn save_config_clicked(ui_state: &mut UiState) {
    let mut file_picker = egui_file::FileDialog::save_file(None);
    file_picker.open();
    ui_state.file_dialog = Some(file_picker);
    ui_state.file_dialog_mode = FileDialogMode::SaveConfig;
}

/// Set context for the file dialog to "loading" and show it.
fn load_config_clicked(ui_state: &mut UiState) {
    let mut file_picker = egui_file::FileDialog::open_file(None);
    file_picker.open();
    ui_state.file_dialog = Some(file_picker);
    ui_state.file_dialog_mode = FileDialogMode::LoadConfig;
}

/// Reset generator config to defaults.
fn reset_config_clicked(
    config: &mut ResMut<GeneratorConfig>,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    **config = GeneratorConfig::default();
    ui_panel.current_panel = default();
    events.world_model_changed = Some(config.general.world_model.clone());
}

/// Load and overwrite configuration from a TOML file.
fn file_dialog_load_config(
    path: &Path,
    config: &mut ResMut<GeneratorConfig>,
    ui_panel: &mut UiStatePanel,
    events: &mut EventStruct,
) {
    if let Ok(res) = load_config(path) {
        **config = res;
        events.world_model_changed = Some(config.general.world_model.clone());
        ui_panel.current_panel = default();
    }
}

/// Save current configuration to a TOML file.
fn file_dialog_save_config(path: &Path, config: &mut ResMut<GeneratorConfig>) {
    save_config(config, path).unwrap()
}

/// Load a layer image and send an event.
fn file_dialog_load_image(
    path: &Path,
    layer: ViewedMapLayer,
    config: &mut ResMut<GeneratorConfig>,
    events: &mut EventStruct,
) {
    let (width, height) = config.general.world_model.get_dimensions();
    let data = load_image(path, width, height).unwrap();
    events.load_layer_request = Some((layer, data));
}

/// Send and event to save a layer image.
fn file_dialog_save_image(path: &Path, layer: ViewedMapLayer, events: &mut EventStruct) {
    events.save_layer_request = Some((layer, path.into()));
}
