mod events;
mod generation;
mod samplers;

use atlas_lib::{base::map::MapPluginBase, bevy::prelude::*, config::gen::AtlasGenConfig};

use crate::map::events::*;

/// Plugin responsible for the world graphics and generation.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPluginBase::<AtlasGenConfig>::default())
            .add_systems(
                Update,
                (
                    update_event_loaded.run_if(check_event_loaded),
                    update_event_saved.run_if(check_event_saved),
                    update_event_rendered.run_if(check_event_rendered),
                    update_event_clear.run_if(check_event_clear),
                    update_event_generate.run_if(check_event_generate),
                    update_event_climatemap.run_if(check_event_climatemap),
                    update_event_import.run_if(check_event_import),
                    update_event_export.run_if(check_event_export),
                ),
            );
    }
}
