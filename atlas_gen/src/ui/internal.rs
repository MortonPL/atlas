use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_egui::egui::{self, Context, Ui};

use crate::config::{load_config, save_config, GeneratorConfig};

use super::panel_general::MainPanelGeneral;

/// Default sidebar width in points. Should be greater or equal to [SIDEBAR_MIN_WIDTH].
const SIDEBAR_WIDTH: f32 = 400.0;
/// Minimal sidebar width in points.
const SIDEBAR_MIN_WIDTH: f32 = 400.0;

/// Minimal camera zoom as Z in world space (bad idea?).
const MIN_CAMERA_ZOOM: f32 = 2.5;
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
    SaveImage,
    /// Load generation layer output from PNG file.
    LoadImage,
}

/// Camera tag.
#[derive(Component)]
pub struct MainCamera;

/// Struct that contains only the UI-related state (no logic).
#[derive(Default, Resource)]
pub struct UiState {
    pub viewport_size: bevy::prelude::Vec2,
    current_panel: Box<dyn MainPanel + Sync + Send>,
    file_dialog: Option<egui_file::FileDialog>,
    file_dialog_mode: FileDialogMode,
}

/// A sidebar page.
pub trait MainPanel {
    /// Get panel heading.
    fn get_heading(&self) -> &'static str;

    /// Create UI for this panel.
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>);

    /// Handle transitioning to the previous or next panel.
    fn transition(&self, prev: bool, next: bool) -> Box<dyn MainPanel + Sync + Send>;
}

impl Default for Box<dyn MainPanel + Sync + Send> {
    fn default() -> Self {
        Box::new(MainPanelGeneral::default())
    }
}

/// Add the entire UI.
pub fn create_ui(
    ctx: &Context,
    mut config: ResMut<GeneratorConfig>,
    mut ui_state: ResMut<UiState>,
) {
    // The UI is a resizeable sidebar fixed to the right window border.
    // __________________
    // | Sidebar Head   |  <-- Title, "Save"/"Load"/"Reset" buttons.
    // |----------------|
    // | Panel-specific |  <-- Panel displaying current stage settings
    // |________________|      and "Previous"/"Next" buttons.
    egui::SidePanel::right("ui_root")
        .min_width(SIDEBAR_MIN_WIDTH)
        .default_width(SIDEBAR_WIDTH)
        .show(ctx, |ui| {
            create_sidebar_head(ui, &mut config, &mut ui_state);
            create_current_panel(ui, &mut config, &mut ui_state);
            adjust_viewport(ui, &mut ui_state);
        });

    handle_file_dialog(ctx, &mut config, &mut ui_state);
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
    ui_state: &mut ResMut<UiState>,
) {
    ui.vertical(|ui| {
        ui.heading(egui::RichText::new("Atlas Map Generator").size(24.0));
        ui.horizontal(|ui| {
            if ui
                .button(egui::RichText::new("Save Config").size(12.0))
                .clicked()
            {
                let mut file_picker = egui_file::FileDialog::save_file(None);
                file_picker.open();
                ui_state.file_dialog = Some(file_picker);
                ui_state.file_dialog_mode = FileDialogMode::SaveConfig;
            }
            if ui
                .button(egui::RichText::new("Load Config").size(12.0))
                .clicked()
            {
                let mut file_picker = egui_file::FileDialog::open_file(None);
                file_picker.open();
                ui_state.file_dialog = Some(file_picker);
                ui_state.file_dialog_mode = FileDialogMode::LoadConfig;
            }
            if ui
                .button(egui::RichText::new("Reset Config").size(12.0))
                .clicked()
            {
                **config = GeneratorConfig::default();
            }
        });
    });
    ui.separator(); // HACK: Do not delete. The panel won't resize without it. Known issue.
}

/// Create the current panel.
fn create_current_panel(
    ui: &mut Ui,
    config: &mut ResMut<GeneratorConfig>,
    ui_state: &mut ResMut<UiState>,
) {
    ui.heading(ui_state.current_panel.get_heading());
    egui::ScrollArea::both().show(ui, |ui| ui_state.current_panel.show(ui, config));
    ui.separator();
    ui.horizontal(|ui| {
        ui_state.current_panel = ui_state
            .current_panel
            .transition(ui.button("Previous").clicked(), ui.button("Next").clicked());
    });
}

/// Adjust viewport size to not overlap the sidebar.
fn adjust_viewport(ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
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
    ui_state: &mut ResMut<UiState>,
) {
    let mode = ui_state.file_dialog_mode;
    if let Some(file_dialog) = &mut ui_state.file_dialog {
        if file_dialog.show(ctx).selected() {
            if let Some(file) = file_dialog.path() {
                match mode {
                    FileDialogMode::LoadConfig => **config = load_config(file).unwrap(), // TODO error handling
                    FileDialogMode::SaveConfig => save_config(config, file).unwrap(), // TODO error handling
                    _ => {}
                }
            }
            ui_state.file_dialog = None;
        }
    }
}
