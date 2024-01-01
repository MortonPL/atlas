use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy_egui::EguiContexts;

use crate::config::GeneratorConfig;

use crate::ui::internal::{create_ui, MainCamera, UiState};

use super::internal::handle_camera;

/// Plugin responsible for the entire GUI and viewport rectangle.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        app.init_resource::<UiState>()
            .add_systems(Startup, startup)
            .add_systems(Update, update)
            .add_systems(PostUpdate, post_update.after(update));
    }
}

/// Update system
///
/// Redraw the immediate UI.
fn update(
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
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        },
        MainCamera,
    ));

    // DEBUG Spawn a red sphere to confirm the camera is set up correctly.
    let mesh = meshes.add(shape::UVSphere::default().into());
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

/// Post update system.
///
/// Set the viewport rectangle to whatever is not occupied by the UI sidebar.
fn post_update(
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
