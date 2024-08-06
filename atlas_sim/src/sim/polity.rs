use atlas_lib::{
    bevy::{
        ecs as bevy_ecs,
        prelude::*,
        utils::{hashbrown::HashMap, petgraph::matrix_graph::Zero},
    },
    bevy_egui::{self},
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::{sim::AtlasSimConfig, AtlasConfig},
    domain::{
        graphics::{color_to_u8, MapLogicData},
        map::MapDataLayer,
    },
    rand::Rng,
    rstar::RTree,
    ui::{sidebar::*, UiEditableEnum},
    MakeUi,
};
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::sim::{check_tick, SimControl, SimMapData};

use super::{
    region::{spawn_region_with_city, City, Region},
    ui::PolityUi,
};

/// Polity simulation.
pub struct PolityPlugin;

impl Plugin for PolityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_visuals).add_systems(
            FixedUpdate,
            (
                update_mapgrab,
                update_jobs_resources,
                update_construction,
                update_culture,
                update_tech,
                update_splits,
                update_pops,
            )
                .chain()
                .run_if(check_tick),
        );
    }
}

/// Ownership status of a polity.
#[derive(Default, Clone, Copy)]
pub enum Ownership {
    /// This polity is independent and has no master.
    #[default]
    Independent,
    /// This polity has a master but keeps local autonomy.
    Autonomous(Entity),
    /// This polity has a master and no local ruler.
    Integrated(Entity),
    /// This polity is occupied by an external force.
    Occupied(Entity),
}

impl UiEditableEnum for Ownership {
    const LEN: usize = 4;

    fn self_as_index(&self) -> usize {
        match self {
            Ownership::Independent => 0,
            Ownership::Autonomous(_) => 1,
            Ownership::Integrated(_) => 2,
            Ownership::Occupied(_) => 3,
        }
    }

    fn index_as_self(&self, _idx: usize) -> Self {
        unreachable!()
    }

    fn index_to_str(idx: usize) -> &'static str {
        match idx {
            0 => "Independent",
            1 => "Autonomous",
            2 => "Integrated",
            3 => "Occupied",
            _ => unreachable!(),
        }
    }
}

/// A political entity that owns land and population.
#[derive(Component, Clone)]
pub struct Polity {
    /// Ownership status.
    pub ownership: Ownership,
    /// Map color.
    pub color: Color,
    /// Produced resources.
    pub resources: [f32; LEN_RES],
    /// Yearly accumulated resources.
    pub resources_acc: [f32; LEN_RES],
    /// Researched technology (major, minor level).
    pub tech: [[f32; 2]; LEN_SCI],
    /// Upkept traditions and Great work/person bonus.
    pub traditions: [[f32; 2]; LEN_TRAD],
    /// Govt policies.
    pub policies: [f32; LEN_POL],
    /// Total polity population.
    pub population: f32,
    /// Population jobs.
    pub jobs: JobStruct,
    /// Owned regions.
    pub regions: Vec<Entity>,
    /// Region rtree.
    pub rtree: RTree<(i32, i32)>,
    /// Accumulated heritage.
    pub heritage: [f32; LEN_TRAD],
    /// Created great works.
    pub great_works: Vec<GreatWork>,
    /// Created great people.
    pub great_people: Vec<GreatPerson>,
    /// Advanced resource capacities.
    pub capacities: [f32; 6],
    /// Average stability of all regions/pops.
    pub avg_stability: f32,
    /// Average health of all regions/pops.
    pub avg_health: f32,
    /// Population split.
    pub manpower_split: [f32; 3],
    /// Production split.
    pub indu_split: [f32; 2],
    /// Wealth split.
    pub wealth_split: [f32; 3],
    /// Technology split.
    pub tech_split: [f32; LEN_SCI],
    /// Tradition split.
    pub trad_split: [f32; LEN_TRAD],
    /// Structure split.
    pub struct_split: [f32; LEN_STR],
}

impl Default for Polity {
    fn default() -> Self {
        Self {
            ownership: Ownership::Independent,
            color: Default::default(),
            resources: Default::default(),
            resources_acc: Default::default(),
            tech: Default::default(),
            traditions: Default::default(),
            policies: Default::default(),
            population: 0.0,
            heritage: Default::default(),
            great_works: Default::default(),
            great_people: Default::default(),
            regions: Default::default(),
            rtree: Default::default(),
            jobs: Default::default(),
            capacities: Default::default(),
            manpower_split: Default::default(),
            indu_split: Default::default(),
            wealth_split: Default::default(),
            tech_split: Default::default(),
            trad_split: Default::default(),
            struct_split: Default::default(),
            avg_stability: 1.0,
            avg_health: 1.0,
        }
    }
}

impl Polity {
    pub fn into_ui(&self, _config: &AtlasSimConfig) -> PolityUi {
        PolityUi {
            ownership: self.ownership,
            color: color_to_u8(&self.color),
            regions: self.regions.len() as u32,
            resources: self.resources.clone(),
            resources_acc: self.resources_acc.clone(),
            tech: self.tech.clone(),
            traditions: self.traditions.clone(),
            policies: self.policies.clone(),
            population: self.population,
            heritage: self.heritage.clone(),
            great_works: self.great_works.clone(),
            great_people: self.great_people.clone(),
            jobs: self.jobs.clone(),
            avg_stability: self.avg_stability,
            avg_health: self.avg_health,
        }
    }
}

#[derive(Clone, Default)]
pub struct GreatWork {
    /// Tradition associated with this great work.
    pub tradition: u8,
    /// Time of creation.
    pub time: u32,
}

#[derive(Clone, Default)]
pub struct GreatPerson {
    /// Tradition associated with this great person.
    pub tradition: u8,
    /// Time of creation.
    pub time: u32,
    /// Is this person still active?
    pub active: bool,
}

#[derive(Clone, Default, MakeUi)]
pub struct JobStruct {
    #[name("Non-Working")]
    #[control(SidebarSlider)]
    pub non_working: f32,
    #[name("Agriculture Workers")]
    #[control(SidebarSlider)]
    pub supply: f32,
    #[name("Industry Workers")]
    #[control(SidebarSlider)]
    pub industry: f32,
    #[name("Servants & Specialists")]
    #[control(SidebarSlider)]
    pub wealth: f32,
}

pub struct ResBonusStruct {
    pub max_supply: f32,
    pub max_industry: f32,
    pub max_wealth: f32,
    pub bonus: f32,
}

pub const LEN_RES: usize = 8;
/// Supply
pub const RES_SUPPLY: usize = 0;
/// Industry Consumption
pub const RES_INDU_POPS: usize = 1;
/// Civilian Industry
pub const RES_CIVILIAN: usize = 2;
/// Military Industry
pub const RES_MILITARY: usize = 3;
/// Wealth Consumption
pub const RES_WEALTH_POPS: usize = 4;
/// Research
pub const RES_RESEARCH: usize = 5;
/// Culture
pub const RES_CULTURE: usize = 6;
/// Treasure
pub const RES_TREASURE: usize = 7;

pub const RES_LABELS: [&str; LEN_RES] = [
    "Supply",
    "Industry Consumption",
    "Civilian Industry",
    "Military Industry",
    "Wealth Consumption",
    "Research",
    "Culture",
    "Treasure",
];

pub const LEN_SCI: usize = 10;
/// Deposit bonuses
const SCI_GEOSCIENCE: usize = 0;
/// Pop growth bonus
const SCI_MEDICINE: usize = 1;
/// Civil engineering bonus
const SCI_ENGINEERING: usize = 2;
/// Military engineering bonus
const SCI_METALLURGY: usize = 3;
/// Culture bonus
const SCI_PHILOSOPHY: usize = 4;
/// Science bonus
const SCI_MATHEMATICS: usize = 5;
/// Treasure bonus
const SCI_FINANCES: usize = 6;
/// Governance bonus
const SCI_LAW: usize = 7;
/// Diplomacy bonus
const SCI_LINGUISTICS: usize = 8;
/// Military bonus
const SCI_PHYSICS: usize = 9;

pub const SCI_LABELS: [&str; LEN_SCI] = [
    "Geoscience",
    "Medicine",
    "Engineering",
    "Metallurgy",
    "Philosophy",
    "Mathematics",
    "Finances",
    "Law",
    "Linguistics",
    "Physics",
];

pub const LEN_POL: usize = 6;
/// Growth policy: Isolationist (improve land) vs Expansionist (claim land)
const POL_EXPANSIONIST: usize = 0;
/// Diplomacy policy: Cooperative (deals) vs Competitive (threats)
const POL_COMPETITIVE: usize = 1;
/// Work Split policy: Industrial (industry) vs Mercantile (wealth)
const POL_MERCANTILE: usize = 2;
/// Industry policy: Pacifist (civilian ind) vs Militarist (military ind)
const POL_MILITARIST: usize = 3;
/// Wealth policy: Traditional (culture) vs Progressive (science)
const POL_PROGRESSIVE: usize = 4;
/// Treasure policy: Spending (low treasure) vs Greedy (high treasure)
const POL_AUTOCRATIC: usize = 5;

pub const POL_LABELS: [&str; LEN_POL] = [
    "Expansionist",
    "Competitive",
    "Mercantile",
    "Militarist",
    "Progressive",
    "Autocratic",
];

pub const LEN_TRAD: usize = 8;

/// Expansion bonus / Great Explorer
const TRAD_PIONEERING: usize = 0;
/// Development bonus /  Great Architect
const TRAD_CREATIVE: usize = 1;
/// Science bonus / Great Scientist
const TRAD_INVENTIVE: usize = 2;
/// Culture bonus / Great Artist
const TRAD_ARTISTIC: usize = 3;
/// Resource bonus / Great Economy
const TRAD_INDUSTRIOUS: usize = 4;
/// Governance bonus / Great Governor
const TRAD_HONORABLE: usize = 5;
/// Diplomacy bonus / Great Diplomat
const TRAD_COOPERATIVE: usize = 6;
/// Military bonus / Great General
const TRAD_MILITANT: usize = 7;

pub const TRAD_LABELS: [&str; LEN_TRAD] = [
    "Pioneering",
    "Creative",
    "Inventive",
    "Artistic",
    "Industrious",
    "Honorable",
    "Cooperative",
    "Militant",
];

pub const LEN_STR: usize = 7;
/// Hospital / Pop growth bonus?
const STR_HOSPITAL: usize = 0;
/// Manufacture / Trade Goods cap
const STR_MANUFACTURE: usize = 1;
/// Forge / Military Equiment cap
const STR_FORGE: usize = 2;
/// University / Research cap
const STR_UNIVERSITY: usize = 3;
/// Amphitheater / Culture cap
const STR_AMPHITHEATER: usize = 4;
/// Courthouse / Governance cap
const STR_COURTHOUSE: usize = 5;
/// Fortress / Military cap
const STR_FORTRESS: usize = 6;

pub const STR_LABELS: [&str; LEN_STR] = [
    "Hospital",
    "Manufacture",
    "Forge",
    "University",
    "Amphitheater",
    "Courthouse",
    "Fortress",
];

/// Update system
///
/// Claim map tiles.
fn update_mapgrab(
    config: Res<AtlasSimConfig>,
    mut query: Query<(Entity, &mut Region)>,
    logics: Res<MapLogicData>,
    mut extras: ResMut<SimMapData>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let climate = logics.get_layer(MapDataLayer::Climate);
    let conts = logics.get_layer(MapDataLayer::Continents);
    for (entity, mut region) in query.iter_mut() {
        // Only claim land when enough investment was made.
        if region.land_claim_fund < config.rules.region.land_claim_cost {
            continue;
        }
        // Check border tiles for free land.
        let weights = region.update_can_expand(&config, &extras, conts, climate);
        // Don't bother if all land is taken or very bad.
        if !region.can_expand {
            continue;
        }
        // Choose one of the tiles.
        let table = WalkerTableBuilder::new(&weights).build();
        let i = table.next_rng(rng.as_mut());
        // Add to region.
        let i = *region.border_tiles.iter().nth(i).unwrap();
        region.claim_tile(entity, i, &mut extras, &config);
    }
}

/// Update system
///
/// Assign jobs and update resources(supply/industry/wealth).
fn update_jobs_resources(
    config: Res<AtlasSimConfig>,
    mut polities: Query<&mut Polity>,
    regions: Query<&Region>,
) {
    for mut polity in polities.iter_mut() {
        let res_bonus = polity.update_deposits(&config, &regions);
        polity.update_jobs(&config, &res_bonus);
        polity.update_resources(&config, &res_bonus);
    }
}

/// Update system
///
/// Update construction.
fn update_construction(
    config: Res<AtlasSimConfig>,
    mut query_p: Query<(Entity, &mut Polity)>,
    mut query_r: Query<&mut Region>,
    sim: Res<SimControl>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut extras: ResMut<SimMapData>,
    logics: Res<MapLogicData>,
    asset_server: Res<AssetServer>,
) {
    if !extras.deferred_regions.is_empty() {
        // Do logic on newly spawned regions.
        let mut deferred_regions = std::mem::take(&mut extras.deferred_regions);
        deferred_regions.retain(|_, v| !v.is_empty());
        for (polity_entity, vec) in deferred_regions.iter_mut() {
            let polity_entity = *polity_entity;
            if let Some((position, region_entity, city_entity)) = vec.pop() {
                let (_, mut polity) = query_p.get_mut(polity_entity).unwrap();
                let mut region = Region::new(polity_entity, city_entity, position);
                region.color_l = rng.gen_range(-0.2..=0.1);
                polity.add_new_region(
                    region_entity,
                    &mut region,
                    position,
                    &config,
                    &mut query_r,
                    &mut extras,
                    &logics,
                );
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
            }
        }
        extras.deferred_regions = deferred_regions;
    }
    // Update construction.
    if sim.is_new_year() {
        for (polity_entity, mut polity) in query_p.iter_mut() {
            let positions = polity.update_construction(&config, &mut extras, &mut query_r, &mut rng);
            if positions.is_empty() {
                continue;
            }
            // Defer region spawn.
            let defer = |vec: &mut Vec<_>| {
                for position in positions {
                    let region_entity = commands.spawn_empty().id();
                    let city_entity = commands.spawn_empty().id();
                    vec.push((position, region_entity, city_entity));
                }
            };
            if let Some(vec) = extras.deferred_regions.get_mut(&polity_entity) {
                defer(vec);
            } else {
                let mut vec = vec![];
                defer(&mut vec);
                extras.deferred_regions.insert(polity_entity, vec);
            };
        }
    }
}

/// Update system
///
/// Update culture.
fn update_culture(
    config: Res<AtlasSimConfig>,
    mut query: Query<&mut Polity>,
    sim: Res<SimControl>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if sim.is_new_year() {
        for mut polity in query.iter_mut() {
            polity.update_culture(&config, &sim, &mut rng)
        }
    }
}

/// Update system
///
/// Update tech.
fn update_tech(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>, sim: Res<SimControl>) {
    if sim.is_new_year() {
        query.iter_mut().for_each(|mut x| x.update_tech(&config));
    }
}

/// Update system
///
/// Update resource splits.
fn update_splits(config: Res<AtlasSimConfig>, mut query: Query<&mut Polity>, sim: Res<SimControl>) {
    if sim.is_new_year() {
        query.iter_mut().for_each(|mut x| x.update_splits(&config));
    }
}

/// Update system
///
/// Grow/shrink population based on supply.
fn update_pops(
    config: Res<AtlasSimConfig>,
    mut query_p: Query<&mut Polity>,
    mut query_r: Query<&mut Region>,
) {
    query_p
        .iter_mut()
        .for_each(|mut x| x.update_social(&config, &mut query_r));
}

/// Update system
///
/// Update polity & city visuals.
fn update_visuals(
    config: Res<AtlasSimConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    query_p: Query<&Polity>,
    mut query_r: Query<(&mut Region, &mut Transform, &mut Handle<StandardMaterial>)>,
    mut query_c: Query<&mut Handle<StandardMaterial>, (With<City>, Without<Region>)>,
) {
    for (mut region, mut tran, mut mat) in query_r.iter_mut() {
        // Don't update if not needed.
        if !region.need_visual_update {
            continue;
        }
        let polity = query_p.get(region.polity).unwrap();
        let mut city_mat = query_c.get_mut(region.city).unwrap();
        region.update_visuals(
            &config,
            polity,
            &mut tran,
            &mut mat,
            &mut city_mat,
            &mut images,
            &mut materials,
        );
    }
}

impl Polity {
    pub fn add_new_region(
        &mut self,
        region_entity: Entity,
        new_region: &mut Region,
        region_pos: u32,
        config: &AtlasSimConfig,
        regions: &mut Query<&mut Region>,
        extras: &mut SimMapData,
        logics: &MapLogicData,
    ) {
        let climate = logics.get_layer(MapDataLayer::Climate);
        let conts = logics.get_layer(MapDataLayer::Continents);
        let mut all_tiles = vec![];
        let mut new_tiles = vec![];
        let mut lookup: HashMap<u32, usize> = Default::default();
        // Gather all polity tiles.
        for (i, region) in self.regions.iter().enumerate() {
            let mut region = regions.get_mut(*region).unwrap();
            let tiles = std::mem::take(&mut region.tiles);
            all_tiles.push(tiles);
            new_tiles.push(vec![region.city_position]);
            lookup.insert(region.city_position, i);
        }
        // Add new city (region).
        self.regions.push(region_entity);
        new_tiles.push(vec![region_pos]);
        lookup.insert(region_pos, lookup.len());
        let pos = config.index_to_map_i(region_pos);
        self.rtree.insert(pos);
        extras.rtree.insert(pos);
        // Reassign tiles to regions based on distance to cities.
        let mut num_tiles = 0;
        for tile in all_tiles.drain(..).flatten() {
            num_tiles += 1;
            if lookup.get(&tile).is_some() {
                continue;
            }
            let pos = config.index_to_map_i(tile);
            let city_pos = self.rtree.nearest_neighbor(&pos).unwrap();
            let city_tile = config.map_i_to_index(*city_pos);
            let i = lookup[&city_tile];
            new_tiles[i].push(tile);
        }
        // Refresh tile ownership map.
        for (i, tiles) in new_tiles.iter().enumerate() {
            for tile in tiles {
                extras.tile_owner[*tile as usize] = Some(self.regions[i]);
            }
        }
        // Update other region properties.
        for (entity, tiles) in self.regions.iter().zip(new_tiles.drain(..)) {
            let entity = *entity;
            if let Ok(mut region) = regions.get_mut(entity) {
                region.population = self.population * (tiles.len() as f32 / num_tiles as f32);
                region.reset_tiles(entity, tiles, config, extras, conts, climate);
            } else {
                new_region.population = self.population * (tiles.len() as f32 / num_tiles as f32);
                new_region.reset_tiles(entity, tiles, config, extras, conts, climate);
            };
        }
    }

    pub fn update_deposits(&mut self, config: &AtlasSimConfig, regions: &Query<&Region>) -> ResBonusStruct {
        let mut max_supply = 0.0;
        let mut max_industry = 0.0;
        let mut max_wealth = 0.0;
        let mut total_dev_bonus = 0.0;
        self.population = 0.0;
        for region in self.regions.iter() {
            let region = if let Ok(region) = regions.get(*region) {
                region
            } else {
                continue;
            };
            let dev_bonus = 1.0 + region.development * config.rules.region.dev_bonus;
            for (id, amount) in region.deposits.iter() {
                let deposit = &config.deposits.types[*id as usize];
                let amount = amount * dev_bonus;
                max_supply += amount * deposit.supply;
                max_industry += amount * deposit.industry;
                max_wealth += amount * deposit.wealth;
            }
            total_dev_bonus += dev_bonus - 1.0;
            self.population += region.population;
        }
        let bonus = self.get_tech_multiplier(config, SCI_GEOSCIENCE);
        max_supply *= bonus;
        max_industry *= bonus;
        max_wealth *= bonus;
        let bonus = bonus * (1.0 + total_dev_bonus / self.regions.len() as f32);
        ResBonusStruct {
            max_supply,
            max_industry,
            max_wealth,
            bonus,
        }
    }

    pub fn update_jobs(&mut self, config: &AtlasSimConfig, res_bonus: &ResBonusStruct) {
        let manpower = self.population; // TODO get_population_manpower_ratio()
        let spare_manpower = manpower;
        self.jobs = JobStruct {
            non_working: self.population - manpower,
            ..Default::default()
        };
        // Early exit if no supplies to be made.
        if res_bonus.max_supply.is_zero() {
            self.jobs.supply = manpower;
            return;
        }
        // Helper function.
        let calc_minimum = |polity: &Polity, res_id: usize, trad: usize| {
            let minimum_manpower = polity.get_consumption(&config, res_id)
                / config.rules.economy.resources[res_id].efficiency
                / res_bonus.bonus
                / polity.get_tradition_multiplier(config, trad);
            minimum_manpower.min(manpower)
        };
        // Supply.
        let supply_manpower = calc_minimum(self, RES_SUPPLY, TRAD_INDUSTRIOUS);
        let spare_manpower = spare_manpower - supply_manpower;
        self.jobs.supply = supply_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            return;
        }
        // Industry.
        let indu_manpower = calc_minimum(self, RES_INDU_POPS, TRAD_INDUSTRIOUS);
        let spare_manpower = spare_manpower - indu_manpower;
        self.jobs.industry = indu_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            return;
        }
        // Wealth.
        let wealth_manpower = calc_minimum(self, RES_WEALTH_POPS, TRAD_INDUSTRIOUS);
        let spare_manpower = spare_manpower - wealth_manpower;
        self.jobs.wealth = wealth_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            return;
        }
        // Assign spare manpower.
        self.jobs.supply += spare_manpower * self.manpower_split[0];
        self.jobs.industry += spare_manpower * self.manpower_split[1];
        self.jobs.wealth += spare_manpower * self.manpower_split[2];
    }

    pub fn update_resources(&mut self, config: &AtlasSimConfig, rb: &ResBonusStruct) {
        let supply = self.get_resource_yield(
            (self.jobs.supply * rb.bonus, rb.max_supply, -1.0),
            (RES_SUPPLY, 1001, TRAD_INDUSTRIOUS),
            config,
        );
        // Split primary resources into secondary resources (industry).
        let indu_pop = self.get_consumption(&config, RES_INDU_POPS);
        let industry =
            self.jobs.industry * self.get_tradition_multiplier(config, TRAD_INDUSTRIOUS) * rb.bonus;
        let industry = (industry - indu_pop).max(0.0);
        let mut civ_indu = 0.0;
        let mut mil_indu = 0.0;
        if industry > 0.0 {
            civ_indu = self.get_resource_yield(
                (
                    industry * self.indu_split[0],
                    rb.max_industry,
                    self.capacities[STR_MANUFACTURE],
                ),
                (RES_CIVILIAN, SCI_ENGINEERING, 1001),
                config,
            );
            mil_indu = self.get_resource_yield(
                (
                    industry * self.indu_split[1],
                    rb.max_industry,
                    self.capacities[STR_FORGE],
                ),
                (RES_MILITARY, SCI_METALLURGY, 1001),
                config,
            );
        };
        // Split primary resources into secondary resources (wealth).
        let wealth_pop = self.get_consumption(&config, RES_WEALTH_POPS);
        let wealth = self.jobs.wealth * self.get_tradition_multiplier(config, TRAD_INDUSTRIOUS) * rb.bonus;
        let wealth = (wealth - wealth_pop).max(0.0);
        let mut research = 0.0;
        let mut culture = 0.0;
        let mut treasure = 0.0;
        if wealth > 0.0 {
            research = self.get_resource_yield(
                (
                    wealth * self.wealth_split[0],
                    rb.max_wealth,
                    self.capacities[STR_UNIVERSITY],
                ),
                (RES_RESEARCH, SCI_MATHEMATICS, TRAD_INVENTIVE),
                config,
            );
            culture = self.get_resource_yield(
                (
                    wealth * self.wealth_split[1],
                    rb.max_wealth,
                    self.capacities[STR_AMPHITHEATER],
                ),
                (RES_CULTURE, SCI_PHILOSOPHY, TRAD_ARTISTIC),
                config,
            );
            treasure = self.get_resource_yield(
                (wealth * self.wealth_split[2], rb.max_wealth, -1.0),
                (RES_TREASURE, SCI_FINANCES, 1001),
                config,
            );
        }
        // Set new resources.
        self.resources = [
            supply, indu_pop, civ_indu, mil_indu, wealth_pop, research, culture, treasure,
        ];
        self.resources_acc[RES_CIVILIAN] += civ_indu;
        self.resources_acc[RES_MILITARY] += mil_indu;
        self.resources_acc[RES_RESEARCH] += research;
        self.resources_acc[RES_CULTURE] += culture;
        self.resources_acc[RES_TREASURE] += treasure;
    }

    pub fn update_construction(
        &mut self,
        config: &AtlasSimConfig,
        extras: &mut SimMapData,
        query: &mut Query<&mut Region>,
        rng: &mut GlobalEntropy<WyRand>,
    ) -> Vec<u32> {
        let can_dev = |region: &Region| region.development < config.rules.region.max_dev_level;
        let can_build =
            |region: &Region| region.struct_levels < config.rules.region.max_dev_level * LEN_STR as f32;
        let can_exp = |region: &Region| region.can_expand;
        let can_split = |region: &Region| region.can_split();
        let mut build_cities = vec![];
        let regions_len = self.regions.len() as f32;
        let mut undeveloped_regions = 0.0;
        let mut expandable_regions = 0.0;
        // Clear resource capacities.
        self.capacities = Default::default();
        // Find cities that aren't maxxed out.
        for region in self.regions.iter() {
            let region = if let Ok(region) = query.get(*region) {
                region
            } else {
                continue;
            };
            if can_dev(&region) || can_build(&region) {
                undeveloped_regions += 1.0;
            }
            if can_exp(&region) || can_split(&region) {
                expandable_regions += 1.0;
            }
        }
        let acc_points = self.resources_acc[RES_CIVILIAN];
        // Clear accumulated resources.
        self.resources_acc[RES_CIVILIAN] = 0.0;
        // Divide industrial effort into expansion and development.
        let expansion_points =
            acc_points * self.policies[POL_EXPANSIONIST] * config.rules.region.base_exp_speed;
        let development_points = (acc_points - expansion_points)
            * config.rules.region.base_dev_speed
            * self.get_tradition_multiplier(config, TRAD_CREATIVE);
        let expansion_points = expansion_points * self.get_tradition_multiplier(config, TRAD_PIONEERING);
        let expansion_points = if expandable_regions.is_zero() {
            0.0
        } else {
            expansion_points / expandable_regions
        };
        let development_points = if undeveloped_regions.is_zero() {
            development_points / regions_len
        } else {
            development_points / undeveloped_regions
        };
        for region in self.regions.iter() {
            let mut region = if let Ok(region) = query.get_mut(*region) {
                region
            } else {
                continue;
            };
            // Check inner tiles for being close to existing cities.
            region.update_can_split(&extras);
            // Distribute expansion points.
            let (exp, split) = match (can_exp(&region), can_split(&region)) {
                (true, true) => (
                    self.policies[POL_EXPANSIONIST],
                    1.0 - self.policies[POL_EXPANSIONIST],
                ),
                (true, false) => (1.0, 0.0),
                (false, true) => (0.0, 1.0),
                (false, false) => (0.0, 0.0),
            };
            // Increase land claim and new city funds.
            region.land_claim_fund += exp * expansion_points;
            region.new_city_fund += split * expansion_points;
            // Distribute development points.
            let development_points =
                development_points / self.get_city_cost_multiplier(config, region.development);
            let (dev, build) = match (can_dev(&region), can_build(&region)) {
                (true, true) => (
                    self.policies[POL_EXPANSIONIST],
                    1.0 - self.policies[POL_EXPANSIONIST],
                ),
                (true, false) => (1.0, 0.0),
                (false, true) => (0.0, 1.0),
                (false, false) => (0.0, 0.0),
            };
            // Increase region development.
            region.development =
                (region.development + dev * development_points).min(config.rules.region.max_dev_level);
            let str_limit = region.development.trunc();
            // Increase region structure level.
            let len = region.structures.len();
            let mut overflow = 0.0;
            let mut maxxed_count = 0;
            let build = build * development_points;
            for i in 0..len {
                let increment = build * self.struct_split[i] * config.rules.region.structures[i].cost;
                let diff = region.structures[i] + increment - str_limit;
                if diff > 0.0 {
                    region.structures[i] = str_limit;
                    overflow += diff;
                    maxxed_count += 1;
                } else {
                    region.structures[i] += increment;
                }
            }
            let build = overflow / (len - maxxed_count) as f32;
            for i in 0..len {
                if region.structures[i] < str_limit {
                    let increment = build * self.struct_split[i] * config.rules.region.structures[i].cost;
                    region.structures[i] = (region.structures[i] + increment).min(str_limit);
                }
            }
            region.struct_levels = region.structures.iter().fold(0.0, |acc, x| acc + x);
            // Recalculate resource capacities.
            let calc_capacity = |i: usize, region: &mut Region| {
                region.structures[i]
                    * config.rules.region.structures[i].strength
                    * config.rules.region.base_capacity
            };
            for i in 0..self.capacities.len() {
                self.capacities[i] += calc_capacity(i, &mut region);
            }
            // Recalculate region powers.
            region.security =
                calc_capacity(STR_COURTHOUSE, &mut region) * self.get_tech_multiplier(config, SCI_LAW);
            region.health =
                calc_capacity(STR_HOSPITAL, &mut region) * self.get_tech_multiplier(config, SCI_MEDICINE);
            // Request splitting regions (by building new cities).
            let diff = region.new_city_fund - config.rules.region.new_city_cost;
            if can_split(&region) && diff > 0.0 {
                let i = rng.gen_range(0..region.split_tiles.len());
                let i = *region.split_tiles.iter().nth(i).unwrap();
                build_cities.push(i);
                extras.add_city_borders(i, &config);
                region.new_city_fund = diff;
            }
        }
        build_cities
    }

    pub fn update_culture(
        &mut self,
        config: &AtlasSimConfig,
        sim: &SimControl,
        rng: &mut GlobalEntropy<WyRand>,
    ) {
        let culture = self.resources_acc[RES_CULTURE] * config.rules.culture.base_speed;
        for (i, val) in self.traditions.iter_mut().enumerate() {
            let increment = self.trad_split[i] * culture / config.rules.culture.traditions[i].cost;
            let decay = config.rules.culture.base_decay + config.rules.culture.level_decay * val[0].floor();
            let new_val = val[0] + increment - decay;
            let overflow = new_val - config.rules.culture.max_level;
            if overflow > 0.0 {
                val[0] = config.rules.culture.max_level;
                self.heritage[i] += overflow * config.rules.culture.heritage_ratio;
            } else {
                val[0] = new_val.max(0.0);
            }
        }
        self.resources_acc[RES_CULTURE] = 0.0;
        if config.rules.culture.great_event_heritage <= 0.0 {
            return;
        }
        for (i, val) in self.heritage.iter_mut().enumerate() {
            let chance = (*val / config.rules.culture.great_event_heritage)
                .min(config.rules.culture.great_event_chance_max);
            let great_event = rng.gen_bool(chance as f64);
            if !great_event {
                continue;
            }
            let great_person = rng.gen_bool(config.rules.culture.great_person_chance as f64);
            if great_person {
                // Add great person.
                self.great_people.push(GreatPerson {
                    tradition: i as u8,
                    time: sim.time,
                    active: true,
                });
                self.traditions[i][1] += config.rules.culture.great_person_bonus;
            } else {
                // Add great work.
                self.great_works.push(GreatWork {
                    tradition: i as u8,
                    time: sim.time,
                });
                self.traditions[i][1] += config.rules.culture.great_work_bonus;
            }
            *val = 0.0;
        }
        for x in self.great_people.iter_mut() {
            if (sim.time >= x.time + config.rules.culture.great_person_duration) && x.active {
                x.active = false;
                self.traditions[x.tradition as usize][1] -=
                    config.rules.culture.great_person_bonus - config.rules.culture.great_work_bonus;
            }
        }
    }

    pub fn update_tech(&mut self, config: &AtlasSimConfig) {
        let total_major_points = self.resources_acc[RES_RESEARCH] * config.rules.tech.speed_major;
        let total_minor_points = self.resources_acc[RES_RESEARCH] * config.rules.tech.speed_minor;
        for (i, val) in self.tech.iter_mut().enumerate() {
            let tech = &config.rules.tech.techs[i];
            let major = val[0].trunc();
            // Get decay and difficulty based on major level.
            let decay = config.rules.tech.base_decay + config.rules.tech.level_decay * major;
            let level_difficulty = 1.0 + major * config.rules.tech.level_difficulty;
            let major_points = (total_major_points * self.tech_split[i] * level_difficulty) / tech.cost;
            let minor_points = (total_minor_points * self.tech_split[i] * level_difficulty) / tech.cost;
            // Advance major level if the minor level is maxxed, otherwise advance minor level.
            // Minor level is easier to advance.
            if val[1] >= config.rules.tech.max_level_minor * major {
                // Advance major level.
                val[0] = (val[0] + major_points - decay).clamp(0.0, config.rules.tech.max_level_major);
            } else {
                // Advance minor level.
                val[1] =
                    (val[1] + minor_points - decay).clamp(0.0, config.rules.tech.max_level_minor * major);
            }
        }
        self.resources_acc[RES_RESEARCH] = 0.0;
    }

    pub fn update_splits(&mut self, _config: &AtlasSimConfig) {
        // Update manpower split.
        self.manpower_split = [
            0.0,
            1.0 - self.policies[POL_MERCANTILE],
            self.policies[POL_MERCANTILE],
        ];
        let sum: f32 = self.manpower_split.iter().sum();
        self.manpower_split = self.manpower_split.map(|x| x / sum);
        // Update industry split.
        self.indu_split = [1.0 - self.policies[POL_MILITARIST], self.policies[POL_MILITARIST]];
        let sum: f32 = self.indu_split.iter().sum();
        self.indu_split = self.indu_split.map(|x| x / sum);
        // Update wealth split.
        let not_greedy = 1.0 - self.policies[POL_AUTOCRATIC];
        self.wealth_split = [
            not_greedy * self.policies[POL_PROGRESSIVE],
            not_greedy * (1.0 - self.policies[POL_PROGRESSIVE]),
            self.policies[POL_AUTOCRATIC],
        ];
        let sum: f32 = self.wealth_split.iter().sum();
        self.wealth_split = self.wealth_split.map(|x| x / sum);
        // Update technology split.
        self.tech_split = [
            self.policies[POL_EXPANSIONIST],
            1.0 - self.policies[POL_EXPANSIONIST],
            1.0 - self.policies[POL_MILITARIST],
            self.policies[POL_MILITARIST],
            1.0 - self.policies[POL_PROGRESSIVE],
            self.policies[POL_PROGRESSIVE],
            self.policies[POL_AUTOCRATIC],
            1.0 - self.policies[POL_AUTOCRATIC],
            1.0 - self.policies[POL_COMPETITIVE],
            self.policies[POL_COMPETITIVE],
        ];
        let sum: f32 = self.tech_split.iter().sum();
        self.tech_split = self.tech_split.map(|x| x / sum);
        // Update tradition split.
        self.trad_split = [
            self.policies[POL_EXPANSIONIST],
            1.0 - self.policies[POL_EXPANSIONIST],
            self.policies[POL_PROGRESSIVE],
            1.0 - self.policies[POL_PROGRESSIVE],
            1.0 - self.policies[POL_AUTOCRATIC],
            self.policies[POL_AUTOCRATIC],
            1.0 - self.policies[POL_COMPETITIVE],
            self.policies[POL_COMPETITIVE],
        ];
        let sum: f32 = self.trad_split.iter().sum();
        self.trad_split = self.trad_split.map(|x| x / sum);
        // Update structue split.
        self.struct_split = [
            1.0 - self.policies[POL_EXPANSIONIST],
            1.0 - self.policies[POL_MILITARIST],
            self.policies[POL_MILITARIST],
            self.policies[POL_PROGRESSIVE],
            1.0 - self.policies[POL_PROGRESSIVE],
            self.policies[POL_AUTOCRATIC],
            self.policies[POL_COMPETITIVE],
        ];
        let sum: f32 = self.struct_split.iter().sum();
        self.struct_split = self.struct_split.map(|x| x / sum);
    }

    pub fn update_social(&mut self, config: &AtlasSimConfig, query: &mut Query<&mut Region>) {
        // Calculate the current supply coverage (no consumption == 100% coverage as well).
        let consumption = self.get_consumption(&config, RES_SUPPLY);
        let coverage = if consumption.is_zero() {
            1.0
        } else {
            self.resources[RES_SUPPLY] / consumption
        };
        // Supply the population. Only supplied population survives and grows.
        // Medicine tech improves pop growth and hospital power.
        let growth = config.rules.economy.pop_growth * self.get_tech_multiplier(config, SCI_MEDICINE);
        self.population = 0.0;
        self.avg_stability = 0.0;
        self.avg_health = 0.0;
        for region in self.regions.iter() {
            let mut region = if let Ok(region) = query.get_mut(*region) {
                region
            } else {
                continue;
            };
            // Calculate region health.
            let sick_pops = ((region.population - region.health).max(0.0) / region.population)
                .min(config.rules.economy.max_health_penalty);
            region.healthcare = 1.0 - sick_pops;
            // Grow the region pops.
            region.population = (region.population * (coverage + growth * region.healthcare))
                .max(config.rules.economy.min_pop);
            self.population += region.population;
            // Calculate region crime/stability.
            let crime_pops = region.population * config.rules.economy.crime_rate
                / self.get_tradition_multiplier(config, TRAD_HONORABLE);
            let crime = (crime_pops - region.security).max(0.0) / region.population;
            region.stability = 1.0 - crime;
            self.avg_stability += region.population * region.stability;
            self.avg_health += region.population * region.healthcare;
        }
        self.avg_stability /= self.population;
        self.avg_health /= self.population;
    }

    fn get_consumption(&self, config: &AtlasSimConfig, res_id: usize) -> f32 {
        let chaos = 1.0 - self.avg_stability;
        self.population
            * match res_id {
                RES_SUPPLY => {
                    config.rules.economy.base_supply_need + config.rules.economy.chaos_supply_loss * chaos
                }
                RES_INDU_POPS => {
                    config.rules.economy.base_industry_need + config.rules.economy.chaos_industry_loss * chaos
                }
                RES_WEALTH_POPS => {
                    config.rules.economy.base_wealth_need + config.rules.economy.chaos_wealth_loss * chaos
                }
                _ => panic!(),
            }
    }

    fn get_resource_yield(
        &self,
        input_max_cap: (f32, f32, f32),
        res_tech_trad: (usize, usize, usize),
        config: &AtlasSimConfig,
    ) -> f32 {
        let (res, tech, trad) = res_tech_trad;
        let mut out = (input_max_cap.0 * config.rules.economy.resources[res].efficiency).min(input_max_cap.1);
        if tech < 1000 {
            out *= self.get_tech_multiplier(config, tech);
        }
        if trad < 1000 {
            out *= self.get_tradition_multiplier(config, trad);
        }
        if input_max_cap.2 >= 0.0 {
            let diff = out - input_max_cap.2;
            if diff > 0.0 {
                out = out + diff * config.rules.economy.resources[res].over_cap_efficiency;
            }
        }
        out
    }

    #[inline(always)]
    fn get_city_cost_multiplier(&self, config: &AtlasSimConfig, city: f32) -> f32 {
        1.0 + config.rules.region.dev_level_cost * city.floor()
    }

    #[inline(always)]
    fn get_tech_multiplier(&self, config: &AtlasSimConfig, i: usize) -> f32 {
        let bonus = config.rules.tech.bonus_major * self.tech[i][0].trunc()
            + config.rules.tech.bonus_minor * self.tech[i][1].trunc();
        1.0 + bonus * config.rules.tech.techs[i].strength
    }

    #[inline(always)]
    fn get_tradition_multiplier(&self, config: &AtlasSimConfig, i: usize) -> f32 {
        let strength = config.rules.culture.traditions[i].strength
            * (self.traditions[i][0] + self.traditions[i][0]).trunc();
        1.0 + config.rules.culture.level_bonus * strength
    }
}
