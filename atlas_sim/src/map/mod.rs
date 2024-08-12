mod internal;

use atlas_lib::{
    base::{
        events::EventStruct,
        map::{resize_helper, MapPluginBase},
        ui::UiStateBase,
    },
    bevy::prelude::*,
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::{
        load_config, load_image, load_image_grey,
        sim::{AtlasSimConfig, CONFIG_NAME},
        AtlasConfig,
    },
    domain::{
        graphics::{MapLogicData, WorldGlobeMesh, WorldMapMesh},
        map::{MapDataLayer, MapDataOverlay, EXPORT_DATA_LAYERS},
    },
    weighted_rand::builder::{NewBuilder, WalkerTableBuilder},
};
use internal::randomize_point_policies;

use crate::{
    map::internal::{
        calc_start_point_weights, create_overlays, randomize_point_color, randomize_start_points,
    },
    sim::{
        polity::Polity,
        region::{spawn_region_with_city, Region},
        SimControl, SimMapData,
    },
    ui::MapOverlay,
};

pub use internal::get_random_policies;

/// Plugin responsible for the world graphics and generation.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPluginBase::<AtlasSimConfig>::default())
            .add_systems(Update, update_event_import_start.run_if(check_event_import_start))
            .add_systems(Update, update_event_random_start.run_if(check_event_random_start))
            .add_systems(Update, update_event_overlay_changed)
            .add_systems(
                Update,
                update_event_start_simulation.run_if(check_event_start_simulation),
            );
    }
}

/// Run condition
///
/// Check if "import initial world" event needs handling.
pub fn check_event_import_start(events: Res<EventStruct>) -> bool {
    events.import_start_request.is_some()
}

/// Run condition
///
/// Check if "randomize start points" event needs handling.
pub fn check_event_random_start(events: Res<EventStruct>) -> bool {
    events.randomize_starts_request.is_some()
}

/// Run condition
///
/// Check if "start simulation" event needs handling.
pub fn check_event_start_simulation(events: Res<EventStruct>) -> bool {
    events.simulation_start_request.is_some()
}

/// Update system
///
/// Import the initial world for simulation.
pub fn update_event_import_start(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasSimConfig>,
    mut extras: ResMut<SimMapData>,
    map: Query<(Entity, &mut Visibility, &mut Transform), With<WorldMapMesh>>,
    globe: Query<(Entity, &mut Visibility), (With<WorldGlobeMesh>, Without<WorldMapMesh>)>,
    commands: Commands,
) {
    let base_path = events.import_start_request.take().expect("Always Some");
    // Import config.
    let path = base_path.join(CONFIG_NAME);
    match load_config(path) {
        Ok(data) => {
            *config = data;
            events.world_model_changed = Some(());
        }
        Err(error) => {
            events.error_window = Some(error.to_string());
            return;
        }
    }
    // Import all layers.
    let (width, height) = (config.general.world_size[0], config.general.world_size[1]);
    let mut regen_layers = vec![];
    for (layer, name) in EXPORT_DATA_LAYERS {
        let path = base_path.join(name);
        let result = match layer {
            MapDataLayer::Preview => load_image(path, width, height),
            _ => load_image_grey(path, width, height),
        };
        match result {
            Ok(data) => {
                logics.put_layer(layer, data);
                regen_layers.push(layer);
            }
            Err(error) => {
                events.error_window = Some(error.to_string());
                return;
            }
        };
    }
    regen_layers.push(MapDataLayer::Preview);
    // Resize if needed.
    resize_helper(commands, config.as_ref(), map, globe, logics);
    extras.tile_region.resize((width * height) as usize, None);
    extras.tile_polity.resize((width * height) as usize, None);
    // Refresh layers.
    events.regen_layer_request = Some(regen_layers);
}

/// Update system
///
/// Randomize scenario start points.
pub fn update_event_random_start(
    mut events: ResMut<EventStruct>,
    mut logics: ResMut<MapLogicData>,
    mut config: ResMut<AtlasSimConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &MapOverlay), With<MapOverlay>>,
    commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut ui_base: ResMut<UiStateBase>,
) {
    events.randomize_starts_request.take().expect("Always Some");
    let (width, height) = config.get_world_size();
    let (width, height) = (width as usize, height as usize);
    // Generate tile weights.
    let (weights, strip_weights) = calc_start_point_weights(&config, &mut logics, width, height);
    // Randomize starting points.
    if !randomize_start_points(&mut config, rng.as_mut(), &weights, &strip_weights, width) {
        events.error_window =
            Some("Failed to choose unique random locations for all points. Try again.".to_string());
    }
    randomize_point_color(&mut config, rng.as_mut());
    randomize_point_policies(&mut config, rng.as_mut());
    // Recreate overlay markers.
    create_overlays(&config, commands, &mut meshes, &mut materials, query);
    // Force show start point overlay.
    ui_base.overlays[0] = true;
}

/// Update system
///
/// Switch the currently visible overlay.
pub fn update_event_overlay_changed(
    mut query: Query<(&mut Visibility, &MapOverlay), With<MapOverlay>>,
    ui_base: Res<UiStateBase>,
) {
    let mask = ui_base.overlays.map(|x| {
        if x {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    });
    // Show some overlays.
    for (mut vis, overlay) in query.iter_mut() {
        *vis = match overlay.overlay {
            MapDataOverlay::None => Visibility::Visible,
            MapDataOverlay::StartPoints => mask[0],
            MapDataOverlay::Polities => mask[1],
            MapDataOverlay::Cities => mask[2],
        };
    }
}

/// Update system
///
/// Set up everything needed to run the simulation.
pub fn update_event_start_simulation(
    mut events: ResMut<EventStruct>,
    mut config: ResMut<AtlasSimConfig>,
    mut sim: ResMut<SimControl>,
    mut extras: ResMut<SimMapData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut ui_base: ResMut<UiStateBase>,
    asset_server: Res<AssetServer>,
) {
    events.simulation_start_request.take();
    sim.paused = false;
    config.rules.combat.action_table_attacker =
        WalkerTableBuilder::new(&config.rules.combat.action_weights_attacker).build();
    config.rules.combat.action_table_defender =
        WalkerTableBuilder::new(&config.rules.combat.action_weights_defender).build();
    // Spawn polities.
    for start in &config.scenario.start_points {
        // Get all coords.
        let p = (start.position[0], start.position[1]);
        let i = config.map_to_index(p);
        let p = (p.0 as i32, p.1 as i32);
        // Prep empty entities.
        let polity_entity = commands.spawn_empty().id();
        let region_entity = commands.spawn_empty().id();
        let city_entity = commands.spawn_empty().id();
        // Prep region.
        let mut region = Region::new(polity_entity, city_entity, i);
        region.population = start.polity.population;
        region.land_claim_fund = config.scenario.starting_land_claim_points;
        region.claim_tile(region_entity, i, 2.0, 0.0, &mut extras, &config);
        spawn_region_with_city(
            region_entity,
            city_entity,
            region,
            &mut commands,
            &mut meshes,
            &mut images,
            &mut materials,
            &asset_server,
            &config,
        );
        // Prep polity.
        let mut polity = Polity {
            this: Some(polity_entity),
            color: Color::rgb_u8(
                start.polity.color[0],
                start.polity.color[1],
                start.polity.color[2],
            ),
            population: start.polity.population,
            regions: [region_entity].into(),
            policies: start.polity.policies.clone(),
            next_policy: start.polity.next_policy,
            ..Default::default()
        };
        polity.rtree.insert(p);
        commands.get_entity(polity_entity).unwrap().insert((polity,));
        // Post spawn actions.
        extras.tile_region[i as usize] = Some(region_entity.clone());
        extras.tile_polity[i as usize] = Some(polity_entity.clone());
        extras.rtree.insert(p);
        extras.add_city_borders(i, &config);
    }
    // Force hide start point overlay.
    ui_base.overlays[0] = false;
}
