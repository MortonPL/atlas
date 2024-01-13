use bevy::app::{MainScheduleOrder, RunFixedUpdateLoop};
use bevy::input::mouse::MouseWheel;
use bevy::render::camera::Viewport;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_egui::{EguiContexts, EguiPlugin};

use crate::config::GeneratorConfig;
use crate::ui::internal::{create_ui, MainCamera, UiState};

use super::internal::handle_camera;

/// Plugin responsible for the entire GUI and viewport rectangle.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let mut schedule_order = app.world.get_resource_mut::<MainScheduleOrder>().unwrap();
        schedule_order.insert_after(RunFixedUpdateLoop, UiUpdate);
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        app.init_resource::<UiState>()
            .add_plugins(EguiPlugin)
            .init_schedule(UiUpdate)
            .add_systems(Startup, startup)
            .add_systems(UiUpdate, update)
            .add_systems(UiUpdate, after_update.after(update));
    }
}

/// Schedule run before [Update], designed for UI handling.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UiUpdate;

/// Update system
///
/// Redraw the immediate UI.
pub fn update(
    config: ResMut<GeneratorConfig>,
    mut contexts: EguiContexts,
    state: ResMut<UiState>,
    kb: Res<Input<KeyCode>>,
    mouse: EventReader<MouseWheel>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
) {
    let ctx = contexts.ctx_mut();
    create_ui(ctx, config, state);
    handle_camera(kb, mouse, &mut cameras.single_mut());
}

/// Startup system
///
/// Spawn the main camera that the viewport will use.
pub fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
            ..Default::default()
        },
        MainCamera,
    ));
}

/// Update system (after update).
///
/// Set the viewport rectangle to whatever is not occupied by the UI sidebar.
pub fn after_update(
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
    ui_state: Res<UiState>,
) {
    let mut cam = cameras.single_mut();
    let viewport_size = ui_state.viewport_size * egui_settings.scale_factor as f32;
    // Layout: viewport on the left, sidebar on the right. Together they take up the entire screen space.
    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        ..Default::default()
    });
}
