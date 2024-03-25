use bevy::{
    app::{MainScheduleOrder, RunFixedUpdateLoop},
    ecs::schedule::ScheduleLabel,
    input::mouse::MouseWheel,
    prelude::*,
    render::camera::Viewport,
};
use bevy_egui::{EguiContexts, EguiPlugin};

use crate::{
    config::GeneratorConfig,
    event::EventStruct,
    ui::internal::{create_ui, handle_camera, UiState, UiStatePanel},
};

/// Plugin responsible for the entire GUI and viewport rectangle.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let mut schedule_order = app.world.get_resource_mut::<MainScheduleOrder>().unwrap();
        schedule_order.insert_after(RunFixedUpdateLoop, UiUpdate);
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        app.init_resource::<UiState>()
            .init_resource::<UiStatePanel>()
            .add_plugins(EguiPlugin)
            .init_schedule(UiUpdate)
            .add_systems(Startup, startup)
            .add_systems(UiUpdate, update_ui)
            .add_systems(UiUpdate, update_input)
            .add_systems(UiUpdate, update_viewport.after(update_ui));
    }
}

/// Schedule run before [Update], designed for UI handling.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UiUpdate;

/// Camera tag.
#[derive(Component)]
struct MainCamera;

/// Update system
///
/// Redraw the immediate UI.
fn update_ui(
    config: ResMut<GeneratorConfig>,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut ui_panel: ResMut<UiStatePanel>,
    mut events: ResMut<EventStruct>,
) {
    create_ui(
        contexts.ctx_mut(),
        config,
        &mut ui_state,
        &mut ui_panel,
        &mut events,
    );
}

/// Update system
///
/// Handle user input
fn update_input(
    kb: Res<Input<KeyCode>>,
    mouse: EventReader<MouseWheel>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
) {
    handle_camera(kb, mouse, &mut cameras.single_mut());
}

/// Update system (after [update_ui]).
///
/// Set the viewport rectangle to whatever is not occupied by the UI sidebar.
fn update_viewport(
    settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
    ui_state: Res<UiState>,
) {
    let viewport_size = ui_state.viewport_size * settings.scale_factor as f32;
    // Layout: viewport on the left, sidebar on the right. Together they take up the entire screen space.
    let mut camera = cameras.single_mut();
    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        ..Default::default()
    });
}

/// Startup system
///
/// Spawn the main camera that the viewport will use.
fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
            ..Default::default()
        },
        MainCamera,
    ));
}
