use bevy::{
    app::{AppExit, MainScheduleOrder, RunFixedMainLoop},
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    ecs::schedule::ScheduleLabel,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::Viewport,
};
use bevy_egui::{
    egui::{self, Context, RichText, Ui},
    EguiPlugin, EguiSettings,
};
use std::path::Path;

use crate::{
    base::events::EventStruct,
    domain::map::{MapDataLayer, MapDataOverlay},
    ui::{
        sidebar::{SidebarControl, SidebarEnumDropdown},
        window,
    },
};

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

/// Default sidebar width in points. Should be greater or equal to [`SIDEBAR_MIN_WIDTH`].
pub const SIDEBAR_WIDTH: f32 = 360.0;
/// Minimum sidebar width in points.
pub const SIDEBAR_MIN_WIDTH: f32 = 360.0;

/// Mode of operation for the generic file dialog.
#[derive(Clone, Copy, Default)]
pub enum FileDialogMode {
    /// Save configuration to TOML file.
    #[default]
    SaveConfig,
    /// Load configuration to TOML file.
    LoadConfig,
    /// Save this layer data to PNG file.
    SaveData(MapDataLayer),
    /// Load this layer data from PNG file.
    LoadData(MapDataLayer),
    /// Render this layer to a PNG file.
    RenderImage(MapDataLayer),
    /// Import all data.
    Import,
    /// Export all data.
    Export,
    /// Import initial data.
    ImportSpecial,
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
    /// Camera movement data.
    pub camera: UiCameraData,
    /// Is the about window open?
    pub about_open: bool,
    /// Currently viewed map layer.
    pub current_layer: MapDataLayer,
    /// Currently viewed map overlay.
    pub current_overlay: MapDataOverlay,
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
        schedule_order.insert_after(RunFixedMainLoop, UiUpdate);
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
                    FileDialogMode::LoadConfig => self.load_config(path),
                    FileDialogMode::SaveConfig => self.save_config(path),
                    FileDialogMode::LoadData(layer) => self.load_layer_data(path, layer),
                    FileDialogMode::SaveData(layer) => self.save_layer_data(path, layer),
                    FileDialogMode::RenderImage(layer) => self.render_image(path, layer),
                    FileDialogMode::Import => self.import(path),
                    FileDialogMode::ImportSpecial => self.import_special(path),
                    FileDialogMode::Export => self.export(path),
                };
            }
            ui_state.file_dialog = None;
        }
    }

    fn load_config(&mut self, path: &Path);
    fn save_config(&mut self, path: &Path);
    fn load_layer_data(&mut self, path: &Path, layer: MapDataLayer);
    fn save_layer_data(&mut self, path: &Path, layer: MapDataLayer);
    fn render_image(&mut self, path: &Path, layer: MapDataLayer);
    fn import(&mut self, path: &Path);
    fn import_special(&mut self, path: &Path);
    fn export(&mut self, path: &Path);
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

/// A handler for error window. Doesn't need to override anything.
#[derive(Default)]
pub struct ErrorWindowHandler;

impl HandleErrorWindow for ErrorWindowHandler {}

pub trait UiCreator<C> {
    fn create_ui(
        &mut self,
        ctx: &mut Context,
        config: &mut C,
        ui_base: &mut UiStateBase,
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
                self.create_sidebar_head(ui, config, ui_base, events, exit);
                ui.separator(); // HACK: Do not delete. The panel won't resize without it. Known issue.
                self.create_layer_view_settings(ui, ui_base, events);
                ui.separator();
                self.create_panel_tabs(ui, ui_base, events);
                ui.separator();
                self.create_current_panel(ui, config, events);
                // We've finished drawing the sidebar. Its size is now established
                // and we can calculate the viewport size.
                adjust_viewport(ui, ui_base);
            });
        // Handle file dialog.
        Self::handle_file_dialog(config, events, ctx, ui_base);
        // Handle error window.
        ErrorWindowHandler.handle(ctx, ui_base);
        // Handle about window.
        Self::handle_about(ctx, "Atlas History Simulator", &mut ui_base.about_open);
    }

    /// Create the top part of the sidebar with configuration S/L.
    fn create_sidebar_head(
        &mut self,
        ui: &mut Ui,
        config: &mut C,
        ui_base: &mut UiStateBase,
        events: &mut EventStruct,
        exit: &mut EventWriter<AppExit>,
    );

    /// Create sidebar settings for the layer display.
    fn create_layer_view_settings(&mut self, ui: &mut Ui, ui_base: &mut UiStateBase, events: &mut EventStruct) {
        // Layer visibility dropdown.
        // NOTE: `ui.horizontal_wrapped()` respects `ui.end_row()` used internally by a `SidebarControl`.
        ui.horizontal(|ui| {
            let old = ui_base.current_layer;
            let selection =
                SidebarEnumDropdown::new(ui, "Layer", &mut ui_base.current_layer).show(None);
            SidebarEnumDropdown::post_show(selection, &mut ui_base.current_layer);
            // Trigger layer change event as needed.
            if old != ui_base.current_layer {
                events.viewed_layer_changed = Some(ui_base.current_layer);
            }
        });
    }

    /// Create tabs for switching panels.
    fn create_panel_tabs(&mut self, ui: &mut Ui, ui_base: &mut UiStateBase, events: &mut EventStruct);

    /// Create the current panel.
    fn create_current_panel(&mut self, ui: &mut Ui, config: &mut C, events: &mut EventStruct);

    /// Handle displaying the "About" window.
    fn handle_about(ctx: &Context, name: impl Into<RichText>, open: &mut bool);

    /// Get a hadler for file dialog input.
    fn handle_file_dialog(config: &mut C, events: &mut EventStruct, ctx: &Context, ui_base: &mut UiStateBase);
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
    mouse_button: Res<ButtonInput<MouseButton>>,
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

/// Update system (after [`update_ui`]).
///
/// Set the viewport rectangle to whatever is not occupied by the UI sidebar.
pub fn update_viewport(
    settings: Res<EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
    ui_base: Res<UiStateBase>,
    window: Query<&Window>,
) {
    if !window.single().focused {
        return;
    }
    let viewport_size = ui_base.viewport_size * settings.scale_factor;
    // Layout: viewport on the left, sidebar on the right. Together they take up the entire screen space.
    let mut camera = cameras.single_mut();
    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        ..Default::default()
    });
}

/// Helper function
///
/// Open the file dialog in requested mode.
pub fn open_file_dialog(ui_base: &mut UiStateBase, mode: FileDialogMode) {
    let mut file_picker = match mode {
        FileDialogMode::SaveConfig => egui_file::FileDialog::save_file(None),
        FileDialogMode::SaveData(_) => egui_file::FileDialog::save_file(None),
        FileDialogMode::RenderImage(_) => egui_file::FileDialog::save_file(None),
        FileDialogMode::LoadConfig => egui_file::FileDialog::open_file(None),
        FileDialogMode::LoadData(_) => egui_file::FileDialog::open_file(None),
        FileDialogMode::Import => egui_file::FileDialog::select_folder(None),
        FileDialogMode::Export => egui_file::FileDialog::select_folder(None),
        FileDialogMode::ImportSpecial => egui_file::FileDialog::select_folder(None),
    };
    file_picker.open();
    ui_base.file_dialog = Some(file_picker);
    ui_base.file_dialog_mode = mode;
}

/// Helper function.
///
/// Calculate viewport size to not overlap the sidebar.
fn adjust_viewport(ui: &mut Ui, ui_base: &mut UiStateBase) {
    let window_size = ui.clip_rect().size();
    let ui_size = ui.max_rect().size();
    ui_base.viewport_size = Vec2 {
        x: (window_size.x - ui_size.x).max(1.0),
        y: window_size.y.max(1.0),
    };
}
