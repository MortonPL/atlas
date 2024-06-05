use bevy::{
    app::{MainScheduleOrder, RunFixedUpdateLoop},
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    ecs::schedule::ScheduleLabel,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_egui::{egui::Context, EguiPlugin};
use std::path::Path;

use crate::{domain::map::MapDataLayer, ui::window};

/// Minimum camera zoom as Z in world space (bad idea?).
const MIN_CAMERA_ZOOM: f32 = 1.0;
/// Maximum camera zoom as Z in world space (bad idea?).
const MAX_CAMERA_ZOOM: f32 = 15.0;
/// Mutliplier to scroll value.
const CAMERA_ZOOM_SPEED: f32 = 0.05;
/// Mutliplier to drag value.
const CAMERA_DRAG_SPEED: f32 = 12.0;
/// Multiplier to rotation value.
const CAMERA_ROTATE_SPEED: f32 = 0.2;

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
    /// Import all layers.
    ImportGen,
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
    pub camera: UiCameraData,
}

#[derive(Resource)]
pub struct UiCameraData {
    /// Saved screen space mouse position.
    pub vec: Vec2,
    /// Saved zoom value.
    pub zoom: f32,
    /// Saved camera translation.
    pub vec2: Vec3,
    /// Saved model rotation.
    pub saved_rotation: Quat,
    /// Target model rotation.
    pub rotation: Quat,
    /// Should the mouse motion mean rotation, or dragging?
    pub rotate_mode: bool,
}

impl Default for UiCameraData {
    fn default() -> Self {
        Self {
            vec: Default::default(),
            zoom: 5.0,
            vec2: Default::default(),
            saved_rotation: Default::default(),
            rotation: Default::default(),
            rotate_mode: false,
        }
    }
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
                    FileDialogMode::ImportGen => self.import_gen(path),
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
    fn import_gen(&mut self, path: &Path);
    fn export_gen(&mut self, path: &Path);
}

/// Handler for the egui error window.
pub trait HandleErrorWindow {
    /// Show the error window if there's an error.
    fn handle(&mut self, ctx: &Context, ui_state: &mut UiStateBase) {
        window(
            ctx,
            "An error has occured",
            &mut ui_state.error_window_open,
            |ui| {
                ui.label(&ui_state.error_message);
            },
        );
    }
}

/// Startup system
///
/// Spawn the main camera that the viewport will use.
fn startup(mut commands: Commands, mut light: ResMut<AmbientLight>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, -5.0).looking_to(-Vec3::Z, Vec3::Y),
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
    mouse_button: Res<Input<MouseButton>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    window: Query<&Window>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    mut ui_state: ResMut<UiStateBase>,
) {
    let window = window.single();
    if !window.focused {
        return;
    }
    // Don't scroll with mouse if it's not inside the viewport or if a dialog is open.
    if let Some(cursor) = window.cursor_position() {
        if (cursor[0] > ui_state.viewport_size[0])
            || (cursor[1] > ui_state.viewport_size[1])
            || ui_state.file_dialog.is_some()
        {
            return;
        }
    }
    let mut camera = cameras.single_mut();
    // Get zoom value.
    let mut scroll = 0.0;
    if let Some(event) = mouse_wheel.read().next() {
        match event.unit {
            MouseScrollUnit::Line => scroll = event.y,
            MouseScrollUnit::Pixel => scroll = event.y * 2.0,
        }
    }
    let mut z = ui_state.camera.zoom;
    // Zoom in.
    if scroll > 0.0 {
        z *= 1.0f32 - CAMERA_ZOOM_SPEED * (1.0 + scroll);
    // Zoom out.
    } else if scroll < 0.0 {
        z *= 1.0f32 + CAMERA_ZOOM_SPEED * (1.0 - scroll);
    }
    ui_state.camera.zoom = z.clamp(MIN_CAMERA_ZOOM, MAX_CAMERA_ZOOM);
    camera.translation.z = ui_state.camera.zoom;

    // Handle rotation/move
    if ui_state.camera.rotate_mode {
        camera.translation.x = 0.0;
        camera.translation.y = 0.0;
        if mouse_button.just_pressed(MouseButton::Right) {
            if let Some(position) = window.cursor_position() {
                ui_state.camera.vec = position;
                ui_state.camera.saved_rotation = ui_state.camera.rotation;
            }
        } else if mouse_button.pressed(MouseButton::Right) {
            if let Some(position) = window.cursor_position() {
                let delta = (position - ui_state.camera.vec) / ui_state.viewport_size;
                let speed = CAMERA_ROTATE_SPEED * camera.translation.z / MAX_CAMERA_ZOOM;
                let yaw = Quat::from_rotation_y(-delta.x * speed);
                let pitch = Quat::from_rotation_x(-delta.y * speed);
                ui_state.camera.rotation = yaw * ui_state.camera.rotation;
                ui_state.camera.rotation = pitch * ui_state.camera.rotation;
            }
        }
    } else {
        ui_state.camera.rotation = Quat::default();
        if mouse_button.just_pressed(MouseButton::Right) {
            if let Some(position) = window.cursor_position() {
                ui_state.camera.vec = position;
                ui_state.camera.vec2 = camera.translation;
            }
        } else if mouse_button.pressed(MouseButton::Right) {
            if let Some(position) = window.cursor_position() {
                let delta = (position - ui_state.camera.vec) / ui_state.viewport_size;
                let speed = CAMERA_DRAG_SPEED * camera.translation.z / MAX_CAMERA_ZOOM;
                camera.translation.x = ui_state.camera.vec2.x - delta.x * speed;
                camera.translation.y = ui_state.camera.vec2.y + delta.y * speed;
            }
        }
    }
}
