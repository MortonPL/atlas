use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_egui::egui::{self, Context, Response, Ui};

use crate::config::{load_config, save_config, GeneratorConfig, WorldModel};

type FnPanelCreator = fn(&mut Ui, &mut ResMut<GeneratorConfig>);

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

/// Type of UI panels with adjustable settings.
#[derive(Default)]
enum UiPanel {
    /// World model, world resolution, tile size...
    #[default]
    General,
    /// Height map...
    Topography,
    /// Temperature model, precipitation, currents...
    Climate,
}

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
    current_panel: UiPanel,
    file_dialog: Option<egui_file::FileDialog>,
    file_dialog_mode: FileDialogMode,
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
                config.set_if_neq(GeneratorConfig::default());
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
    let (head, panel_fun, prev, next): (&str, &FnPanelCreator, UiPanel, UiPanel) =
        match ui_state.current_panel {
            UiPanel::General => (
                "General",
                &(create_panel_general as FnPanelCreator),
                UiPanel::General,
                UiPanel::Topography,
            ),
            UiPanel::Topography => (
                "Topography",
                &(create_panel_topography as FnPanelCreator),
                UiPanel::General,
                UiPanel::Climate,
            ),
            UiPanel::Climate => (
                "Climate",
                &(create_panel_climate as FnPanelCreator),
                UiPanel::Topography,
                UiPanel::Climate,
            ),
        };
    ui.heading(head);
    egui::ScrollArea::both().show(ui, |ui| panel_fun(ui, config));
    ui.horizontal(|ui| {
        if ui.button("Previous").clicked() {
            ui_state.current_panel = prev;
        }
        if ui.button("Next").clicked() {
            ui_state.current_panel = next;
        }
    });
}

fn create_panel_general(ui: &mut Ui, config: &mut ResMut<GeneratorConfig>) {
    add_section(ui, "Stuff", |ui| {
        ui.label("World Model").on_hover_text_at_pointer("TODO");
        egui::ComboBox::from_label("")
            .selected_text(config.general.world_model.str())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut config.general.world_model,
                    WorldModel::Flat,
                    WorldModel::Flat.str(),
                );
                ui.selectable_value(
                    &mut config.general.world_model,
                    WorldModel::Globe,
                    WorldModel::Globe.str(),
                )
            })
            .response
            .on_hover_text_at_pointer("TODO");
        ui.end_row();
    });
}

fn create_panel_topography(_ui: &mut Ui, _config: &mut ResMut<GeneratorConfig>) {
    // TODO
}

fn create_panel_climate(_ui: &mut Ui, _config: &mut ResMut<GeneratorConfig>) {
    // TODO
}

/// Add a section consisting of a collapsible header and a grid.
fn add_section<BodyRet>(
    ui: &mut Ui,
    header: impl Into<String>,
    add_body: impl FnOnce(&mut Ui) -> BodyRet,
) -> Response {
    let header: String = header.into();
    egui::CollapsingHeader::new(egui::RichText::new(header.clone()).heading())
        .default_open(true)
        .show(ui, |ui| {
            egui::Grid::new(format!("{}_grid", header)).show(ui, add_body);
        })
        .header_response
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
    let Some(file_dialog) = &mut ui_state.file_dialog else {
        return;
    };
    if !file_dialog.show(ctx).selected() {
        return;
    }
    let Some(file) = file_dialog.path() else {
        return;
    };
    match mode {
        FileDialogMode::LoadConfig => _ = config.set_if_neq(load_config(file).unwrap()), // TODO error handling
        FileDialogMode::SaveConfig => save_config(config, file).unwrap(), // TODO error handling
        _ => {}
    }
    ui_state.file_dialog = None;
}
