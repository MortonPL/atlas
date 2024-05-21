use atlas_lib::{
    bevy::{app::AppExit, ecs as bevy_ecs, prelude::*},
    bevy_egui::egui::Context,
    ui::plugin_base::UiStateBase,
};

use crate::{config::AtlasSimConfig, event::EventStruct};

#[derive(Default, Resource)]
pub struct UiState {
    base: UiStateBase,
}

pub fn create_ui(
    ctx: &mut Context,
    config: &mut AtlasSimConfig,
    ui_state: &mut UiState,
    events: &mut EventStruct,
    exit: &mut EventWriter<AppExit>,
) {
}
