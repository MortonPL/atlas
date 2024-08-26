use std::collections::BTreeSet;

use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*, utils::HashMap},
    config::{sim::AtlasSimConfig, AtlasConfig},
    rstar::RTree,
};
use conflict::Conflict;
use polity::PolityPlugin;

pub mod conflict;
pub mod polity;
pub mod region;
pub mod ui;

/// Plugin responsible for the actual simulation.
pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimControl>()
            .init_resource::<SimMapData>()
            .add_systems(FixedUpdate, tick)
            .add_plugins(PolityPlugin);
    }
}

/// Extra map data just for the simulation.
#[derive(Resource, Default)]
pub struct SimMapData {
    /// Owner regions of specific map tiles.
    pub tile_region: Vec<Option<Entity>>,
    /// Owner polities of specific map tiles.
    pub tile_polity: Vec<Option<Entity>>,
    /// Region city rtree.
    pub rtree: RTree<(i32, i32)>,
    /// Deferred region spawn data.
    pub deferred_regions: HashMap<Entity, Vec<(u32, Entity, Entity)>>,
    /// Tiles occupied by cities and surroundings.
    pub city_borders: BTreeSet<u32>,
    /// Active conflicts.
    pub conflicts: HashMap<u32, Conflict>,
    /// Counter for unique conflict ids.
    pub conflict_counter: u32,
    /// Polity pair map of current wars and truce time.
    pub war_map: WarMap,
    /// Pending tributes to polities (industry, wealth).
    pub tributes: HashMap<Entity, (f32, f32)>,
}

#[derive(Resource, Default)]
pub struct WarMap(HashMap<EntityPair, (u32, u32)>);

impl WarMap {
    pub fn get_war_map_num(&mut self, us: Entity, them: Entity) -> (u32, u32) {
        let pair = EntityPair::new(us, them);
        if let Some((num, truce)) = self.0.get(&pair) {
            (*num, *truce)
        } else {
            (0, 0)
        }
    }

    pub fn inc_war_map_num(&mut self, us: Entity, them: Entity) -> u32 {
        let pair = EntityPair::new(us, them);
        if let Some((num, truce)) = self.0.get_mut(&pair) {
            *num += 1;
            *truce = 0;
            *num
        } else {
            self.0.insert_unique_unchecked(pair, (1, 0));
            1
        }
    }

    pub fn dec_war_map_num(&mut self, us: Entity, them: Entity, truce: u32) -> u32 {
        let pair = EntityPair::new(us, them);
        if let Some((num, trc)) = self.0.get_mut(&pair) {
            *num -= 1;
            if *num == 0 {
                *trc = truce;
            }
            *num
        } else {
            self.0.insert_unique_unchecked(pair, (0, truce));
            0
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EntityPair(Entity, Entity);

impl EntityPair {
    pub fn new(a: Entity, b: Entity) -> Self {
        if a > b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

impl SimMapData {
    pub fn add_city_borders(&mut self, position: u32, config: &AtlasSimConfig) {
        self.city_borders.extend(config.get_border_tiles_9(position));
    }

    pub fn create_conflict(&mut self, start_date: u32, attacker: Entity, defender: Entity) -> u32 {
        let id = self.conflict_counter;
        self.conflict_counter += 1;
        let conflict = Conflict {
            id,
            start_date,
            concluded: false,
            attackers: Default::default(),
            defenders: Default::default(),
            primary_attacker: attacker,
            primary_defender: defender,
        };
        self.conflicts.insert(id, conflict);
        id
    }

    pub fn add_conflict_member(&mut self, us_e: Entity, color: [u8; 3], id: u32, is_attacker: bool, build_up: u32) {
        let conflict = self.conflicts.get_mut(&id).unwrap();
        conflict.add_member(us_e, color, build_up, is_attacker);
        let enemies = if is_attacker {
            conflict.defenders.keys()
        } else {
            conflict.attackers.keys()
        };
        for enemy_e in enemies {
            self.war_map.inc_war_map_num(us_e, *enemy_e);
        }
    }
}

/// Data for controlling the simulation flow (and extras).
#[derive(Resource, Clone, PartialEq)]
pub struct SimControl {
    /// Is the current tick the active tick (should other systems run?).
    pub tick: bool,
    /// Is the simulation paused?
    pub paused: bool,
    /// Simulation speed.
    pub speed: f32,
    /// Current simulation time, measured in simulated months.
    pub time: u32,
    /// Elapsed time at the moment of the last active tick.
    last_tick_time: f32,
}

impl Default for SimControl {
    fn default() -> Self {
        Self {
            tick: false,
            paused: true,
            speed: 1.0,
            time: 0,
            last_tick_time: -1000.0,
        }
    }
}

impl SimControl {
    /// Get the current simulation time as a "MM.YYYY" string.
    pub fn current_time_to_string(&self) -> String {
        time_to_string(self.time)
    }

    /// Check if this tick is a new year.
    pub fn is_new_year(&self) -> bool {
        self.time % 12 == 0
    }
}

/// FixedUpdate system
///
/// Control the time flow of the simulation.
fn tick(mut sim: ResMut<SimControl>, time: Res<Time<Fixed>>) {
    if sim.paused {
        sim.tick = false;
        return;
    }
    let current = time.elapsed_seconds();
    if (current - sim.last_tick_time) * sim.speed >= 1.0 {
        sim.time += 1;
        sim.last_tick_time = current;
        sim.tick = true;
    } else {
        sim.tick = false;
    }
}

/// Run condition
///
/// Only run simulation on active ticks.
pub fn check_tick(sim: Res<SimControl>) -> bool {
    sim.tick
}

pub fn time_to_string(time: u32) -> String {
    format!("{:02}.{}", time % 12 + 1, time / 12 + 1)
}

pub fn time_to_string_plus(time: u32, str: &str) -> String {
    format!("{:02}.{} {}", time % 12 + 1, time / 12 + 1, str)
}
