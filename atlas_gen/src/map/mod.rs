mod events;
mod generation;
mod internal;
mod samplers;

use atlas_lib::{
    base::{
        events::{
            check_event_changed, check_event_export, check_event_import, check_event_regen,
            check_event_world_model, update_event_changed, update_event_world_model, EventStruct,
        },
        ui::{UiStateBase, UiUpdate},
    },
    bevy::prelude::*,
    config::WorldModel,
    domain::graphics::{
        spawn_default_globe, spawn_default_plane, startup_layers, CurrentWorldModel, MapGraphicsData,
        MapLogicData,
    },
};

use crate::{
    config::AtlasGenConfig,
    map::events::{
        check_event_clear, check_event_climatemap, check_event_generate, check_event_loaded,
        check_event_rendered, check_event_saved, update_event_clear, update_event_climatemap,
        update_event_export, update_event_generate, update_event_import, update_event_loaded,
        update_event_regen, update_event_rendered, update_event_saved,
    },
};

/// Plugin responsible for the world graphics and generation.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapGraphicsData>()
            .init_resource::<MapLogicData>()
            .add_systems(Startup, startup_layers)
            .add_systems(Startup, startup_model.after(startup_layers))
            .add_systems(
                Update,
                update_event_world_model::<AtlasGenConfig>.run_if(check_event_world_model),
            )
            .add_systems(Update, update_event_changed.run_if(check_event_changed))
            .add_systems(Update, update_event_loaded.run_if(check_event_loaded))
            .add_systems(Update, update_event_saved.run_if(check_event_saved))
            .add_systems(Update, update_event_rendered.run_if(check_event_rendered))
            .add_systems(Update, update_event_clear.run_if(check_event_clear))
            .add_systems(Update, update_event_regen.run_if(check_event_regen))
            .add_systems(Update, update_event_generate.run_if(check_event_generate))
            .add_systems(Update, update_event_climatemap.run_if(check_event_climatemap))
            .add_systems(Update, update_event_import.run_if(check_event_import))
            .add_systems(Update, update_event_export.run_if(check_event_export))
            .add_systems(UiUpdate, update_model_rotation);
    }
}

/// Startup system
///
/// Spawn the map and globe world models.
fn startup_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: ResMut<MapGraphicsData>,
    mut events: ResMut<EventStruct>,
) {
    spawn_default_globe(&mut commands, &mut meshes, &graphics);
    spawn_default_plane(&mut commands, &mut meshes, &graphics);
    // Trigger model change.
    events.world_model_changed = Some(());
}

/// Update system
///
/// Update rotation mode and apply transformations.
fn update_model_rotation(
    mut model: Query<&mut Transform, With<CurrentWorldModel>>,
    config: Res<AtlasGenConfig>,
    mut ui_base: ResMut<UiStateBase>,
) {
    ui_base.camera.rotate_mode = match config.general.preview_model {
        WorldModel::Flat => false,
        WorldModel::Globe => true,
    };
    if ui_base.camera.rotate_mode {
        let mut transform = model.single_mut();
        transform.rotation = ui_base.camera.rotation;
    }
}
