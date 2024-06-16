mod internal;
mod panel;

use atlas_lib::{
    bevy::{app::AppExit, prelude::*, render::camera::Viewport},
    bevy_egui::{EguiContexts, EguiSettings},
    ui::plugin_base::{MainCamera, UiPluginBase, UiStateBase, UiUpdate},
};
use internal::UiStatePanel;

use crate::{
    config::AtlasSimConfig,
    event::EventStruct,
    ui::internal::{create_ui, UiState},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPluginBase)
            .init_resource::<UiStatePanel>()
            .init_resource::<UiState>()
            .add_systems(UiUpdate, update_ui)
            .add_systems(UiUpdate, update_viewport.after(update_ui));
    }
}

/// Update system
///
/// Redraw the immediate UI.
fn update_ui(
    mut config: ResMut<AtlasSimConfig>,
    mut contexts: EguiContexts,
    mut ui_base: ResMut<UiStateBase>,
    mut ui_state: ResMut<UiState>,
    mut ui_panel: ResMut<UiStatePanel>,
    mut events: ResMut<EventStruct>,
    mut exit: EventWriter<AppExit>,
    window: Query<&Window>,
) {
    if !window.single().focused {
        return;
    }
    create_ui(
        contexts.ctx_mut(),
        &mut config,
        &mut ui_base,
        &mut ui_state,
        &mut ui_panel,
        &mut events,
        &mut exit,
    );
}

/// Update system (after [`update_ui`]).
///
/// Set the viewport rectangle to whatever is not occupied by the UI sidebar.
fn update_viewport(
    settings: Res<EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
    ui_base: Res<UiStateBase>,
    window: Query<&Window>,
) {
    if !window.single().focused {
        return;
    }
    let viewport_size = ui_base.viewport_size * settings.scale_factor as f32;
    // Layout: viewport on the left, sidebar on the right. Together they take up the entire screen space.
    let mut camera = cameras.single_mut();
    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        ..Default::default()
    });
}
