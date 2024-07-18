use atlas_lib::bevy::{ecs as bevy_ecs, prelude::*};
use polity::PolityPlugin;

pub mod civ;
pub mod polity;

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
#[derive(Resource)]
pub struct SimMapData {
    /// Owner polities of specific map tiles.
    pub tile_owner: Vec<Option<Entity>>,
}

impl Default for SimMapData {
    fn default() -> Self {
        Self {
            tile_owner: Default::default(),
        }
    }
}

/// Data for controlling the simulation flow.
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
    pub last_tick_time: f32,
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
    pub fn time_to_string(&self) -> String {
        format!("{:02}.{}", self.time % 12 + 1, self.time / 12 + 1)
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

/// Run condition
///
/// Only run simulation on active ticks, once per year.
pub fn check_tick_annual(sim: Res<SimControl>) -> bool {
    sim.tick && (sim.time % 12 == 0)
}
