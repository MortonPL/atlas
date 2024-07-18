mod events;
mod internal;

use atlas_lib::{base::map::MapPluginBase, bevy::prelude::*, config::sim::AtlasSimConfig};
use events::{
    check_event_import_start, check_event_overlay_changed, check_event_random_start,
    check_event_start_simulation, update_event_overlay_changed, update_event_random_start,
    update_event_start_simulation,
};

use crate::map::events::update_event_import_start;

/// Plugin responsible for the world graphics and generation.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPluginBase::<AtlasSimConfig>::default())
            .add_systems(Update, update_event_import_start.run_if(check_event_import_start))
            .add_systems(Update, update_event_random_start.run_if(check_event_random_start))
            .add_systems(
                Update,
                update_event_overlay_changed.run_if(check_event_overlay_changed),
            )
            .add_systems(
                Update,
                update_event_start_simulation.run_if(check_event_start_simulation),
            );
        /*
        .add_systems(Update, update_event_import.run_if(check_event_import))
        .add_systems(Update, update_event_export.run_if(check_event_export))
        */
    }
}
