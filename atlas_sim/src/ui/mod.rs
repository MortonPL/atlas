mod internal;

use atlas_lib::bevy::{app::AppExit, prelude::*};
use atlas_lib::{
    bevy_egui::EguiContexts,
    ui::plugin_base::{UiPluginBase, UiUpdate},
};

use crate::{
    config::AtlasSimConfig,
    event::EventStruct,
    ui::internal::{create_ui, UiState},
};

pub struct Uiplugin;

impl Plugin for Uiplugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPluginBase).add_systems(UiUpdate, update_ui);
    }
}

/// Update system
///
/// Redraw the immediate UI.
fn update_ui(
    mut config: ResMut<AtlasSimConfig>,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
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
        &mut ui_state,
        &mut events,
        &mut exit,
    );
}
