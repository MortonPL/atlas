mod events;
mod generation;
mod internal;
mod samplers;

use atlas_lib::bevy::prelude::*;

use crate::{
    config::AtlasGenConfig,
    event::EventStruct,
    map::{
        events::{
            check_event_changed, check_event_clear, check_event_climatemap, check_event_export, check_event_import,
            check_event_generate, check_event_loaded, check_event_regen, check_event_rendered,
            check_event_saved, check_event_world_model, update_event_changed, update_event_clear,
            update_event_climatemap, update_event_export, update_event_import, update_event_generate, update_event_loaded,
            update_event_regen, update_event_rendered, update_event_saved, update_event_world_model,
        },
        internal::{
            make_image, spawn_default_globe, spawn_default_plane, MapGraphicsData, MapGraphicsLayer,
            MapLogicData, MAP_DATA_LAYERS,
        },
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
            .add_systems(Update, update_event_world_model.run_if(check_event_world_model))
            .add_systems(Update, update_event_changed.run_if(check_event_changed))
            .add_systems(Update, update_event_loaded.run_if(check_event_loaded))
            .add_systems(Update, update_event_saved.run_if(check_event_saved))
            .add_systems(Update, update_event_rendered.run_if(check_event_rendered))
            .add_systems(Update, update_event_clear.run_if(check_event_clear))
            .add_systems(Update, update_event_regen.run_if(check_event_regen))
            .add_systems(Update, update_event_generate.run_if(check_event_generate))
            .add_systems(Update, update_event_climatemap.run_if(check_event_climatemap))
            .add_systems(Update, update_event_import.run_if(check_event_import))
            .add_systems(Update, update_event_export.run_if(check_event_export));
    }
}

/// Startup system
///
/// Initialize each map layer.
fn startup_layers(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut graphics: ResMut<MapGraphicsData>,
    mut logics: ResMut<MapLogicData>,
) {
    // Create the default texture and material.
    let empty_texture = make_image(1, 1, vec![0, 0, 0, 255]);
    let empty_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(empty_texture.clone())),
        unlit: true,
        ..default()
    });
    graphics.empty_material = empty_material;
    // Initialize all graphic and logical map layers.
    for layer in MAP_DATA_LAYERS {
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(empty_texture.clone())),
            unlit: true,
            ..default()
        });
        graphics
            .layers
            .insert(layer, MapGraphicsLayer::new(material.clone()));
        logics.put_layer(layer, vec![]);
    }
}

/// Startup system
///
/// Spawn the map and globe world models.
fn startup_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: ResMut<MapGraphicsData>,
    config: Res<AtlasGenConfig>,
    mut events: ResMut<EventStruct>,
) {
    spawn_default_globe(&mut commands, &mut meshes, &graphics);
    spawn_default_plane(&mut commands, &mut meshes, &graphics);
    // Trigger model change.
    events.world_model_changed = Some(config.general.world_model.clone());
}
