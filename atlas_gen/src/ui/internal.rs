use std::path::Path;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_egui::egui::{self, Context, Ui};

use crate::{
    config::{self, load_config, load_image, save_config, save_image, GeneratorConfig},
    map::ViewedMapLayer,
};

use super::panel_general::MainPanelGeneral;

/// Default sidebar width in points. Should be greater or equal to [SIDEBAR_MIN_WIDTH].
const SIDEBAR_WIDTH: f32 = 400.0;
/// Minimal sidebar width in points.
const SIDEBAR_MIN_WIDTH: f32 = 300.0;

/// Minimal camera zoom as Z in world space (bad idea?).
const MIN_CAMERA_ZOOM: f32 = 2.0;
/// Minimal camera zoom as Z in world space (bad idea?).
const MAX_CAMERA_ZOOM: f32 = 5.0;
/// Mutliplier to current Z.
const CAMERA_ZOOM_SPEED: f32 = 0.05;

#[derive(Clone, Copy, Default)]
pub enum FileDialogMode {
    /// Save generator configuration to TOML file.
    #[default]
    SaveConfig,
    /// Load generator configuration to TOML file.
    LoadConfig,
    /// Save generation layer output to PNG file.
    SaveImage(ImageLayer),
    /// Load generation layer output from PNG file.
    LoadImage(ImageLayer),
}

#[derive(Clone, Copy)]
pub enum ImageLayer {
    Continental,
    Topographical,
    Climate,
}

/// Camera tag.
#[derive(Component)]
pub struct MainCamera;

/// Struct that contains only the UI-related state (no logic).
#[derive(Default, Resource)]
pub struct UiState {
    pub viewport_size: bevy::prelude::Vec2,
    pub file_dialog: Option<egui_file::FileDialog>,
    pub file_dialog_mode: FileDialogMode,
    pub just_loaded_layer: bool,
    pub just_changed_dimensions: bool,
}

#[derive(Default, Resource)]
pub struct UiStatePanel {
    pub current_panel: Box<dyn MainPanel + Sync + Send>,
}

/// A sidebar page.
pub trait MainPanel {
    /// Get panel heading.
    fn get_heading(&self) -> &'static str;

    /// Create UI for this panel.
    fn show(&self, ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState);

    /// Handle transitioning to the previous or next panel.
    fn transition(&self, prev: bool, next: bool) -> Box<dyn MainPanel + Sync + Send>;

    /// Map self to [ViewedMapLayer].
    fn get_map_layer(&self) -> ViewedMapLayer;
}

impl Default for Box<dyn MainPanel + Sync + Send> {
    fn default() -> Self {
        Box::<MainPanelGeneral>::default()
    }
}

/// Add the entire UI.
pub fn create_ui(
    ctx: &Context,
    mut config: ResMut<GeneratorConfig>,
    mut ui_state: ResMut<UiState>,
    mut ui_panel: ResMut<UiStatePanel>,
) {
    // The UI is a resizeable sidebar fixed to the right window border.
    // __________________
    // | Sidebar Head   |  <-- Title, "Save"/"Load"/"Reset" buttons.
    // |----------------|
    // | Panel-specific |  <-- Panel displaying current stage settings
    // |                |      and "Previous"/"Next" buttons.
    // |  Prev || Next  |
    // |________________|
    egui::SidePanel::right("ui_root")
        .min_width(SIDEBAR_MIN_WIDTH)
        .default_width(SIDEBAR_WIDTH)
        .show(ctx, |ui| {
            create_sidebar_head(ui, &mut config, &mut ui_state);
            create_current_panel(ui, &mut config, &mut ui_state, &mut ui_panel);
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
fn create_sidebar_head(ui: &mut Ui, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
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
    ui_panel: &mut ResMut<UiStatePanel>,
) {
    ui.heading(ui_panel.current_panel.get_heading());
    egui::ScrollArea::both().show(ui, |ui| ui_panel.current_panel.show(ui, config, ui_state));
    ui.separator();
    ui.horizontal(|ui| {
        ui_panel.current_panel = ui_panel
            .current_panel
            .transition(ui.button("Previous").clicked(), ui.button("Next").clicked());
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
fn handle_file_dialog(ctx: &Context, config: &mut ResMut<GeneratorConfig>, ui_state: &mut UiState) {
    let mode = ui_state.file_dialog_mode;
    if let Some(file_dialog) = &mut ui_state.file_dialog {
        if file_dialog.show(ctx).selected() {
            if let Some(file) = file_dialog.path() {
                match mode {
                    FileDialogMode::LoadConfig => **config = load_config(file).unwrap(), // TODO error handling
                    FileDialogMode::SaveConfig => save_config(config, file).unwrap(), // TODO error handling
                    FileDialogMode::LoadImage(layer) => {
                        handle_load_file(config, file, layer).unwrap(); // TODO error handling
                        ui_state.just_loaded_layer = true;
                    }
                    FileDialogMode::SaveImage(layer) => {
                        handle_save_file(config, file, layer).unwrap(); // TODO error handling
                    }
                };
            }
            ui_state.file_dialog = None;
        }
    }
}

fn handle_load_file(
    config: &mut ResMut<GeneratorConfig>,
    file: &Path,
    layer: ImageLayer,
) -> Result<(), config::Error> {
    let (width, height) = config.general.world_model.get_dimensions();
    let data = load_image(file, width, height)?;
    match layer {
        ImageLayer::Continental => config.continents.data = data,
        _ => {}
    };
    Ok(())
}

fn handle_save_file(
    config: &mut ResMut<GeneratorConfig>,
    file: &Path,
    layer: ImageLayer,
) -> Result<(), config::Error> {
    let (width, height) = config.general.world_model.get_dimensions();
    let empty = vec![];
    let data = match layer {
        ImageLayer::Continental => &config.continents.data,
        _ => &empty,
    };
    save_image(file, data, width, height)?;
    Ok(())
}
