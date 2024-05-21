use std::path::Path;
use bevy::{
    app::{MainScheduleOrder, RunFixedUpdateLoop},
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    ecs::schedule::ScheduleLabel,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_egui::{egui::{self, Align2, Context}, EguiPlugin};

use crate::domain::map::MapDataLayer;

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
    SaveGenConfig,
    /// Load generator configuration to TOML file.
    LoadGenConfig,
    /// Save simulator configuration to TOML file.
    SaveSimConfig,
    /// Load simulator configuration to TOML file.
    LoadSimConfig,
    /// Save this layer data to PNG file.
    SaveData(MapDataLayer),
    /// Load this layer data from PNG file.
    LoadData(MapDataLayer),
    /// Render this layer to a PNG file.
    RenderImage(MapDataLayer),
    /// Export all layers.
    ExportGen,
}

/// Struct that contains only the UI-related state (no logic).
#[derive(Default, Resource)]
pub struct UiStateBase {
    /// Size (in pixels) of the viewport, AKA window size - sidebar size (if applicable).
    pub viewport_size: bevy::prelude::Vec2,
    /// All purpose file dialog. Some if open, None if closed.
    pub file_dialog: Option<egui_file::FileDialog>,
    /// File dialog mode of operation. See [`FileDialogMode`].
    pub file_dialog_mode: FileDialogMode,
    /// Is the error popup window open?
    pub error_window_open: bool,
    /// Current error message.
    pub error_message: String,
}

/// Plugin with base UI setup and input handling, but no contentts.
pub struct UiPluginBase;

impl Plugin for UiPluginBase {
    fn build(&self, app: &mut App) {
        let mut schedule_order = app.world.get_resource_mut::<MainScheduleOrder>().unwrap();
        schedule_order.insert_after(RunFixedUpdateLoop, UiUpdate);
        // NOTE: Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        app.init_resource::<UiStateBase>()
            .add_plugins(EguiPlugin)
            .init_schedule(UiUpdate)
            .add_systems(Startup, startup)
            .add_systems(UiUpdate, update_input);
    }
}

/// Schedule run before [`Update`], designed for UI handling.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UiUpdate;

/// Camera tag.
#[derive(Component)]
pub struct MainCamera;

/// Handler for the egui file dialog. All you need to do
/// is provide implementation for each file dialog mode.
pub trait HandleFileDialog {
    fn handle(&mut self, ctx: &Context, ui_state: &mut UiStateBase) {
        let mode = ui_state.file_dialog_mode;
        if let Some(file_dialog) = &mut ui_state.file_dialog {
            if !file_dialog.show(ctx).selected() {
                return;
            }
            if let Some(path) = file_dialog.path() {
                match mode {
                    FileDialogMode::LoadGenConfig => self.load_gen_config(path),
                    FileDialogMode::SaveGenConfig => self.save_gen_config(path),
                    FileDialogMode::LoadSimConfig => self.load_sim_config(path),
                    FileDialogMode::SaveSimConfig => self.save_sim_config(path),
                    FileDialogMode::LoadData(layer) => self.load_layer_data(path, layer),
                    FileDialogMode::SaveData(layer) => self.save_layer_data(path, layer),
                    FileDialogMode::RenderImage(layer) => self.render_image(path, layer),
                    FileDialogMode::ExportGen => self.export_gen(path),
                };
            }
            ui_state.file_dialog = None;
        }
    }

    fn load_gen_config(&mut self, path: &Path);
    fn save_gen_config(&mut self, path: &Path);
    fn load_sim_config(&mut self, path: &Path);
    fn save_sim_config(&mut self, path: &Path);
    fn load_layer_data(&mut self, path: &Path, layer: MapDataLayer);
    fn save_layer_data(&mut self, path: &Path, layer: MapDataLayer);
    fn render_image(&mut self, path: &Path, layer: MapDataLayer);
    fn export_gen(&mut self, path: &Path);
}

/// Handler for the egui error window.
pub trait HandleErrorWindow {
    /// Show the error window if there's an error.
    fn handle(&mut self, ctx: &Context, ui_state: &mut UiStateBase) {
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
}

/// Startup system
///
/// Spawn the main camera that the viewport will use.
fn startup(mut commands: Commands, mut light: ResMut<AmbientLight>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
            tonemapping: Tonemapping::None,
            dither: DebandDither::Disabled,
            ..default()
        },
        MainCamera,
    ));
    // More ambient light than default.
    light.brightness = 1.0;
}

/// Update system
///
/// Handle user input.
fn update_input(
    kb: Res<Input<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    window: Query<&Window>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    ui_state: Res<UiStateBase>,
) {
    let window = window.single();
    if !window.focused {
        return;
    }
    let mut camera = cameras.single_mut();

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
