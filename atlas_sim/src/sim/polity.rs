use atlas_lib::{
    bevy::{
        ecs as bevy_ecs,
        prelude::*,
        utils::{hashbrown::HashMap, petgraph::matrix_graph::Zero, HashSet},
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
    rand_distr::{Distribution, Normal},
    rstar::RTree,
    ui::sidebar::*,
    weighted_rand::builder::{NewBuilder, WalkerTableBuilder},
    MakeUi,
};

use crate::{
    map::get_random_policies,
    sim::{
        check_tick,
        region::{spawn_region_with_city, City, Region},
        ui::PolityUi,
        SimControl, SimMapData,
    },
};

/// Polity simulation.
pub struct PolityPlugin;

impl Plugin for PolityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_visuals).add_systems(
            FixedUpdate,
            (
                update_territory,
                update_jobs_resources,
                update_construction,
                update_culture,
                update_tech,
                update_pops,
                update_diplomacy,
                update_war,
                update_splits,
            )
                .chain()
                .run_if(check_tick),
        );
    }
}

/// A political entity that owns land and population.
#[derive(Component, Clone)]
pub struct Polity {
    /// This entity.
    pub this: Option<Entity>,
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
    /// Number of essential jobs (covering basic resource consumption).
    pub essential_jobs: f32,
    /// Owned regions.
    pub regions: HashSet<Entity>,
    /// Region rtree.
    pub rtree: RTree<(i32, i32)>,
    /// Accumulated heritage.
    pub heritage: [f32; LEN_TRAD],
    /// Created great works.
    pub great_works: Vec<GreatWork>,
    /// Created great people.
    pub great_people: Vec<GreatPerson>,
    /// Advanced resource capacities.
    pub capacities: [f32; LEN_STR],
    /// Average stability of all regions/pops.
    pub avg_stability: f32,
    /// Average health of all regions/pops.
    pub avg_health: f32,
    /// Neighbouring polities and: their border regions, relations, and flip-flag.
    pub neighbours: HashMap<Entity, (HashSet<Entity>, f32, bool)>,
    /// Active conflicts.
    pub conflicts: HashSet<u32>,
    /// Available reinforcements per conflict (this month).
    pub reinforcements: Option<(f32, f32)>,
    /// Damage dealt to forts (this month).
    pub fort_damage: f32,
    /// Damage dealt to civilians (this month).
    pub civilian_damage: f32,
    /// Demobilized military waiting to be re-added to the population pool.
    pub demobilized_troops: f32,
    /// Tributes to pay.
    pub tributes: Vec<Vec<Tribute>>,
    /// Date of the next policy change.
    pub next_policy: u32,
    /// Regional sprawl penalty.
    pub sprawl_penalty: f32,
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
            this: None,
            reinforcements: None,
            avg_stability: 1.0,
            avg_health: 1.0,
            fort_damage: 0.0,
            civilian_damage: 0.0,
            population: 0.0,
            demobilized_troops: 0.0,
            sprawl_penalty: 0.0,
            next_policy: 0,
            essential_jobs: 0.0,
            color: Default::default(),
            resources: Default::default(),
            resources_acc: Default::default(),
            tech: Default::default(),
            traditions: Default::default(),
            policies: Default::default(),
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
            conflicts: Default::default(),
            neighbours: Default::default(),
            tributes: Default::default(),
        }
    }
}

impl Polity {
    pub fn into_ui(&self, _config: &AtlasSimConfig) -> PolityUi {
        PolityUi {
            this: self.this.unwrap(),
            color: color_to_u8(&self.color),
            regions: self.regions.len() as u32,
            resources: self.resources.clone(),
            resources_acc: self.resources_acc.clone(),
            tech: self.tech.clone(),
            next_policy: self.next_policy,
            traditions: self.traditions.clone(),
            policies: self.policies.clone(),
            population: self.population,
            heritage: self.heritage.clone(),
            great_works: self.great_works.clone(),
            great_people: self.great_people.clone(),
            jobs: self.jobs.clone(),
            avg_stability: self.avg_stability,
            avg_health: self.avg_health,
            tributes: self.tributes.iter().flatten().map(|x| x.clone()).collect(),
            neighbours: self.neighbours.iter().map(|(k, v)| (*k, v.1)).collect(),
        }
    }
}

#[derive(Clone)]
pub struct Tribute {
    pub receiver: Entity,
    pub fraction: f32,
    pub time_left: u32,
}

impl Tribute {
    pub fn new(receiver: Entity, fraction: f32, time_left: u32) -> Self {
        Self {
            receiver,
            fraction,
            time_left,
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
    #[name("Military")]
    #[control(SidebarSlider)]
    pub military: f32,
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

#[derive(Default)]
pub struct ResBonusStruct {
    pub max_supply: f32,
    pub max_industry: f32,
    pub max_wealth: f32,
    pub bonus: f32,
}

pub const LEN_RES: usize = 10;
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
/// Loyalty
pub const RES_LOYALTY: usize = 7;
/// Industry Tribute
pub const RES_INDU_TRIB: usize = 8;
/// Wealth Tribute
pub const RES_WEALTH_TRIB: usize = 9;

pub const RES_LABELS: [&str; LEN_RES] = [
    "Supply",
    "Industry Consumption",
    "Civilian Industry",
    "Military Industry",
    "Wealth Consumption",
    "Research",
    "Culture",
    "Loyalty",
    "Industry Tributes",
    "Wealth Tributes",
];

pub const LEN_SCI: usize = 10;
/// Deposit bonuses
const SCI_GEOSCIENCE: usize = 0;
/// Pop growth bonus / lower damage
pub const SCI_MEDICINE: usize = 1;
/// Civil engineering bonus
const SCI_ENGINEERING: usize = 2;
/// Military engineering bonus
const SCI_METALLURGY: usize = 3;
/// Culture bonus
const SCI_PHILOSOPHY: usize = 4;
/// Science bonus
const SCI_MATHEMATICS: usize = 5;
/// Loyalty bonus
const SCI_MANAGEMENT: usize = 6;
/// Governance bonus
const SCI_LAW: usize = 7;
/// Diplomacy bonus
const SCI_LINGUISTICS: usize = 8;
/// Military bonus
pub const SCI_MILTECH: usize = 9;

pub const SCI_LABELS: [&str; LEN_SCI] = [
    "Geoscience",
    "Medicine",
    "Engineering",
    "Metallurgy",
    "Philosophy",
    "Mathematics",
    "Management",
    "Law",
    "Linguistics",
    "Military Tech",
];

pub const LEN_POL: usize = 6;
/// Growth policy: Isolationist (improve land) vs Expansionist (claim land)
const POL_EXPANSIONIST: usize = 0;
/// Diplomacy policy: Cooperative (deals) vs Competitive (threats)
const POL_COMPETITIVE: usize = 1;
/// Work Split policy: Industrial (industry) vs Mercantile (wealth)
const POL_MERCANTILE: usize = 2;
/// Production policy: Pacifist (civilian ind/wealth) vs Militarist (military ind/loyalty)
pub const POL_MILITARIST: usize = 3;
/// Wealth policy: Traditional (culture) vs Progressive (science)
const POL_PROGRESSIVE: usize = 4;
/// Stability policy
const POL_LEGALIST: usize = 5;

pub const POL_LABELS: [&str; LEN_POL] = [
    "Expansionist",
    "Competitive",
    "Mercantile",
    "Militarist",
    "Progressive",
    "Legalist",
];

pub const LEN_TRAD: usize = 8;

/// Expansion bonus / Great Explorer
const TRAD_PIONEERING: usize = 0;
/// Development bonus / Great Architect
const TRAD_CREATIVE: usize = 1;
/// Science bonus / Great Scientist
const TRAD_INVENTIVE: usize = 2;
/// Culture bonus / Great Artist
const TRAD_ARTISTIC: usize = 3;
/// Resource bonus / Great Economist
const TRAD_INDUSTRIOUS: usize = 4;
/// Governance bonus / Great Governor
const TRAD_HONORABLE: usize = 5;
/// Diplomacy bonus / Great Diplomat
pub const TRAD_DIPLOMATIC: usize = 6;
/// Military bonus / Great General
pub const TRAD_MILITANT: usize = 7;

pub const TRAD_LABELS: [&str; LEN_TRAD] = [
    "Pioneering",
    "Creative",
    "Inventive",
    "Artistic",
    "Industrious",
    "Honorable",
    "Diplomatic",
    "Militant",
];

pub const GRT_LABELS: [&str; LEN_TRAD] = [
    "Great Pioneer",
    "Great Architect",
    "Great Scientist",
    "Great Artist",
    "Great Economist",
    "Great Governor",
    "Great Diplomat",
    "Great General",
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
/// Fortress / Conflict fortification level
pub const STR_FORTRESS: usize = 6;

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
fn update_territory(
    config: Res<AtlasSimConfig>,
    mut regions: Query<&mut Region>,
    mut polities: Query<&mut Polity>,
    logics: Res<MapLogicData>,
    mut extras: ResMut<SimMapData>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let climate = logics.get_layer(MapDataLayer::Climate);
    let conts = logics.get_layer(MapDataLayer::Continents);
    for mut polity in polities.iter_mut() {
        polity.update_territory(&config, &mut regions, &mut extras, conts, climate, rng.as_mut());
    }
}

/// Update system
///
/// Assign jobs and update resources(supply/industry/wealth).
fn update_jobs_resources(
    config: Res<AtlasSimConfig>,
    mut polities: Query<(Entity, &mut Polity)>,
    regions: Query<&Region>,
    mut extras: ResMut<SimMapData>,
) {
    for (entity, mut polity) in polities.iter_mut() {
        let res_bonus = polity.update_deposits(&config, &regions);
        let tribute = if let Some(tribute) = extras.tributes.get_mut(&entity) {
            let values = tribute.to_owned();
            *tribute = (0.0, 0.0);
            values
        } else {
            (0.0, 0.0)
        };
        polity.update_jobs(&config, &res_bonus, tribute.clone());
        polity.update_resources(&config, &res_bonus, tribute, &mut extras);
    }
}

/// Update system
///
/// Update construction.
fn update_construction(
    config: Res<AtlasSimConfig>,
    mut polities: Query<(Entity, &mut Polity)>,
    mut regions: Query<&mut Region>,
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
                let (_, mut polity) = polities.get_mut(polity_entity).unwrap();
                let mut region = Region::new(polity_entity, city_entity, position);
                region.color_l = rng.gen_range(-0.1..=0.1);
                polity.add_new_region(
                    region_entity,
                    &mut region,
                    position,
                    &config,
                    &mut regions,
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
        for (polity_entity, mut polity) in polities.iter_mut() {
            let positions = polity.update_construction(&config, &mut extras, &mut regions, &mut rng);
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
    mut polities: Query<&mut Polity>,
    sim: Res<SimControl>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if sim.is_new_year() {
        for mut polity in polities.iter_mut() {
            polity.update_culture(&config, &sim, &mut rng)
        }
    }
}

/// Update system
///
/// Update tech.
fn update_tech(config: Res<AtlasSimConfig>, mut polities: Query<&mut Polity>, sim: Res<SimControl>) {
    if sim.is_new_year() {
        polities.iter_mut().for_each(|mut x| x.update_tech(&config));
    }
}

/// Update system
///
/// Grow/shrink population based on supply.
fn update_pops(
    config: Res<AtlasSimConfig>,
    mut polities: Query<&mut Polity>,
    mut regions: Query<&mut Region>,
) {
    polities
        .iter_mut()
        .for_each(|mut x| x.update_social(&config, &mut regions));
}

/// Update system
///
/// Update diplomatic stance.
fn update_diplomacy(
    config: Res<AtlasSimConfig>,
    mut polities: Query<(Entity, &mut Polity)>,
    regions: Query<&Region>,
    sim: Res<SimControl>,
    mut extras: ResMut<SimMapData>,
) {
    let vec: Vec<_> = polities
        .iter_mut()
        .map(|(e, mut x)| {
            x.update_neighbours(&regions, &mut extras);
            e
        })
        .collect();
    if sim.is_new_year() {
        for (_, truce) in extras.war_map.0.values_mut() {
            *truce = truce.checked_sub(1).unwrap_or_default();
        }
        for polity_e in vec {
            unsafe {
                let (_, mut polity) = polities.get_unchecked(polity_e).unwrap();
                polity.update_diplomacy(&config, &polities, polity_e, &mut extras, sim.time);
            }
        }
    }
}

/// Update system
///
/// Update conflicts.
fn update_war(
    config: Res<AtlasSimConfig>,
    mut polities: Query<&mut Polity>,
    mut regions: Query<&mut Region>,
    mut extras: ResMut<SimMapData>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let mut conflicts = std::mem::take(&mut extras.conflicts);
    conflicts.retain(|_, conflict| {
        conflict.update(&config, &mut polities, &mut regions, rng.as_mut(), &mut extras);
        if conflict.concluded {
            // TODO cleanup
            false
        } else {
            true
        }
    });
    extras.conflicts = conflicts;
    polities
        .iter_mut()
        .for_each(|mut x| x.update_post_conflict(&config, &mut regions));
}

/// Update system
///
/// Update resource splits and policies.
fn update_splits(
    config: Res<AtlasSimConfig>,
    mut polities: Query<&mut Polity>,
    sim: Res<SimControl>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if sim.is_new_year() {
        polities
            .iter_mut()
            .for_each(|mut x| x.update_splits(&config, sim.time, rng.as_mut()));
    }
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
        let mut region_vec: Vec<_> = self.regions.drain().collect();
        // Gather all polity tiles.
        for (i, region) in region_vec.iter().enumerate() {
            let mut region = regions.get_mut(*region).unwrap();
            let tiles = std::mem::take(&mut region.tiles);
            all_tiles.push(tiles);
            new_tiles.push(vec![region.city_position]);
            lookup.insert(region.city_position, i);
        }
        // Add new city (region).
        region_vec.push(region_entity);
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
                extras.tile_region[*tile as usize] = Some(region_vec[i]);
            }
        }
        // Update other region properties.
        for (entity, tiles) in region_vec.iter().zip(new_tiles.drain(..)) {
            let entity = *entity;
            if let Ok(mut region) = regions.get_mut(entity) {
                region.population = self.population * (tiles.len() as f32 / num_tiles as f32);
                region.reset_tiles(entity, tiles, config, extras, conts, climate);
            } else {
                new_region.population = self.population * (tiles.len() as f32 / num_tiles as f32);
                new_region.reset_tiles(entity, tiles, config, extras, conts, climate);
            };
        }
        self.regions = region_vec.drain(..).collect();
    }

    pub fn update_territory(
        &mut self,
        config: &AtlasSimConfig,
        regions: &mut Query<&mut Region>,
        extras: &mut SimMapData,
        conts: &[u8],
        climate: &[u8],
        rng: &mut GlobalEntropy<WyRand>,
    ) {
        for region_entity in self.regions.iter() {
            let mut region = if let Ok(region) = regions.get_mut(*region_entity) {
                region
            } else {
                continue;
            };
            // Only claim land when enough investment was made.
            if region.land_claim_fund < config.rules.region.land_claim_cost {
                continue;
            }
            // Check border tiles for free land.
            let weights = region.update_expansion(&config, extras, conts, climate);
            // Don't bother if all land is taken or very bad.
            if !region.can_expand {
                continue;
            }
            // Choose one of the tiles.
            let table = WalkerTableBuilder::new(&weights).build();
            let i = table.next_rng(rng);
            // Add to region.
            let tile = *region.border_tiles.iter().nth(i).unwrap();
            let weight = weights[i];
            region.claim_tile(*region_entity, tile, weight, self.sprawl_penalty, extras, &config);
        }
    }

    pub fn update_deposits(&mut self, config: &AtlasSimConfig, regions: &Query<&Region>) -> ResBonusStruct {
        if self.regions.is_empty() {
            return ResBonusStruct::default();
        }
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

    pub fn update_jobs(&mut self, config: &AtlasSimConfig, res_bonus: &ResBonusStruct, tribute: (f32, f32)) {
        if self.regions.is_empty() {
            return;
        }
        let manpower = self.population;
        let spare_manpower = manpower;
        self.essential_jobs = 0.0;
        self.jobs = JobStruct {
            military: self.jobs.military,
            ..Default::default()
        };
        // Early exit if no supplies to be made.
        if res_bonus.max_supply.is_zero() {
            self.jobs.supply = manpower;
            self.essential_jobs += manpower;
            return;
        }
        // Helper function.
        let calc_minimum = |polity: &Polity, res_id: usize, trad: usize, free: f32| {
            let minimum_manpower = (polity.get_consumption(&config, res_id) - free).max(0.0)
                / config.rules.economy.resources[res_id].efficiency
                / res_bonus.bonus
                / polity.get_tradition_multiplier(config, trad);
            minimum_manpower.min(manpower)
        };
        // Supply.
        let supply_manpower = calc_minimum(self, RES_SUPPLY, TRAD_INDUSTRIOUS, 0.0);
        let spare_manpower = spare_manpower - supply_manpower;
        self.jobs.supply = supply_manpower;
        self.essential_jobs += supply_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            return;
        }
        // Industry.
        let indu_manpower = calc_minimum(self, RES_INDU_POPS, TRAD_INDUSTRIOUS, tribute.0);
        let spare_manpower = spare_manpower - indu_manpower;
        self.jobs.industry = indu_manpower;
        self.essential_jobs += indu_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            return;
        }
        // Wealth.
        let wealth_manpower = calc_minimum(self, RES_WEALTH_POPS, TRAD_INDUSTRIOUS, tribute.1);
        let spare_manpower = spare_manpower - wealth_manpower;
        self.jobs.wealth = wealth_manpower;
        self.essential_jobs += wealth_manpower;
        // Early exit if we used up all manpower.
        if spare_manpower <= 0.0 {
            return;
        }
        // Assign spare manpower.
        self.jobs.supply += spare_manpower * self.manpower_split[0];
        self.jobs.industry += spare_manpower * self.manpower_split[1];
        self.jobs.wealth += spare_manpower * self.manpower_split[2];
    }

    pub fn update_resources(
        &mut self,
        config: &AtlasSimConfig,
        rb: &ResBonusStruct,
        tribute: (f32, f32),
        extras: &mut SimMapData,
    ) {
        if self.regions.is_empty() {
            return;
        }
        // Calculate primary resource output.
        let supply = self.get_resource_yield(
            (self.jobs.supply * rb.bonus, rb.max_supply, -1.0),
            (RES_SUPPLY, 1001, TRAD_INDUSTRIOUS),
            config,
        );
        let indu_pop = self.get_consumption(&config, RES_INDU_POPS);
        let industry = self.get_resource_yield(
            (self.jobs.industry * rb.bonus, rb.max_industry, -1.0),
            (RES_INDU_POPS, 1001, TRAD_INDUSTRIOUS),
            config,
        ) + tribute.0;
        let industry = (industry - indu_pop).max(0.0);
        let wealth_pop = self.get_consumption(&config, RES_WEALTH_POPS);
        let wealth = self.get_resource_yield(
            (self.jobs.wealth * rb.bonus, rb.max_wealth, -1.0),
            (RES_WEALTH_POPS, 1001, TRAD_INDUSTRIOUS),
            config,
        ) + tribute.1;
        let wealth = (wealth - wealth_pop).max(0.0);
        // Handle tributes.
        let tribute_num = if config.rules.combat.tribute_ratio == 0.0 {
            0
        } else {
            (1.0 / config.rules.combat.tribute_ratio) as usize
        };
        let mut indu_tribute = 0.0;
        let mut wealth_tribute = 0.0;
        for (i, tribute_list) in self.tributes.iter_mut().enumerate() {
            if i >= tribute_num {
                break;
            }
            for tribute in tribute_list.iter_mut() {
                let account = extras.tributes.get_mut(&tribute.receiver).unwrap();
                let fraction = config.rules.combat.tribute_ratio * tribute.fraction;
                let indu2 = industry * fraction;
                let wealth2 = wealth * fraction;
                indu_tribute += indu2;
                wealth_tribute += wealth2;
                account.0 += indu2 * config.rules.economy.resources[RES_INDU_TRIB].efficiency;
                account.1 += wealth2 * config.rules.economy.resources[RES_WEALTH_TRIB].efficiency;
                tribute.time_left -= 1;
            }
        }
        self.tributes
            .retain(|x| x.first().is_some_and(|x| x.time_left > 0));
        let industry = (industry - indu_tribute).max(0.0);
        let wealth = (wealth - wealth_tribute).max(0.0);
        // Split primary resources into secondary resources (industry).
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
        let mut research = 0.0;
        let mut culture = 0.0;
        let mut loyalty = 0.0;
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
            loyalty = self.get_resource_yield(
                (wealth * self.wealth_split[2], rb.max_wealth, -1.0),
                (RES_LOYALTY, SCI_MANAGEMENT, 1001),
                config,
            );
        }
        // Set new resources.
        self.resources = [
            supply,
            indu_pop,
            civ_indu,
            mil_indu,
            wealth_pop,
            research,
            culture,
            loyalty,
            indu_tribute,
            wealth_tribute,
        ];
        self.resources_acc[RES_CIVILIAN] += civ_indu;
        self.resources_acc[RES_RESEARCH] += research;
        self.resources_acc[RES_CULTURE] += culture;
        self.resources_acc[RES_MILITARY] = (self.resources_acc[RES_MILITARY] + mil_indu).min(mil_indu * 60.0);
        self.resources_acc[RES_LOYALTY] = (self.resources_acc[RES_LOYALTY] + loyalty).min(loyalty * 60.0);
    }

    pub fn update_construction(
        &mut self,
        config: &AtlasSimConfig,
        extras: &mut SimMapData,
        query: &mut Query<&mut Region>,
        rng: &mut GlobalEntropy<WyRand>,
    ) -> Vec<u32> {
        if self.regions.is_empty() {
            return vec![];
        }
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
        self.sprawl_penalty = config.rules.region.sprawl_penalty * regions_len as f32;
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
            let build = build * development_points;
            for i in 0..len {
                let increment = build * self.struct_split[i] * config.rules.region.structures[i].cost;
                region.structures[i] = (region.structures[i] + increment).min(str_limit);
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
            let diff = region.new_city_fund - config.rules.region.new_city_cost - self.sprawl_penalty;
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
        if self.regions.is_empty() {
            return;
        }
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
        if self.regions.is_empty() {
            return;
        }
        let total_major_points = self.resources_acc[RES_RESEARCH] * config.rules.tech.speed_major;
        let total_minor_points = self.resources_acc[RES_RESEARCH] * config.rules.tech.speed_minor;
        for (i, val) in self.tech.iter_mut().enumerate() {
            let tech = &config.rules.tech.techs[i];
            let major = val[0].trunc();
            // Get decay and difficulty based on major level.
            let decay = config.rules.tech.base_decay + config.rules.tech.level_decay * major;
            let level_difficulty = 1.0 + major * config.rules.tech.level_difficulty;
            // Advance major level if the minor level is maxxed, otherwise advance minor level.
            // Minor level is easier to advance.
            if val[1] >= config.rules.tech.max_level_minor * major {
                // Advance major level.
                let mut major_points = (total_major_points * self.tech_split[i]) / tech.cost - decay;
                if major_points > 0.0 {
                    major_points /= level_difficulty;
                }
                val[0] = (val[0] + major_points).clamp(0.0, config.rules.tech.max_level_major);
            } else {
                // Advance minor level.
                let mut minor_points = (total_minor_points * self.tech_split[i]) / tech.cost - decay;
                if minor_points > 0.0 {
                    minor_points /= level_difficulty;
                }
                val[1] = (val[1] + minor_points).clamp(0.0, config.rules.tech.max_level_minor * major);
            }
        }
        self.resources_acc[RES_RESEARCH] = 0.0;
    }

    pub fn update_social(&mut self, config: &AtlasSimConfig, query: &mut Query<&mut Region>) {
        if self.regions.is_empty() {
            return;
        }
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
            let sickness = ((region.population - region.health).max(0.0) / region.population)
                .min(config.rules.economy.max_health_penalty);
            region.healthcare = 1.0 - sickness;
            // Grow the region pops.
            region.population = (region.population * (coverage + growth * region.healthcare))
                .max(config.rules.economy.min_pop);
            self.population += region.population;
            // Calculate region crime/stability.
            let crime_pops = region.population * (config.rules.economy.crime_rate + region.rebel_rate)
                / self.get_tradition_multiplier(config, TRAD_HONORABLE);
            let crime = (crime_pops - region.security).max(0.0) / region.population;
            region.stability = 1.0 - crime;
            region.rebel_rate = (region.rebel_rate
                - (region.stability - region.rebel_rate) * config.rules.economy.rebelion_speed)
                .clamp(0.0, 2.0);
            self.avg_stability += region.population * region.stability;
            self.avg_health += region.population * region.healthcare;
        }
        self.avg_stability = (self.avg_stability / self.population).clamp(0.0, 1.0);
        self.avg_health /= self.population;
    }

    pub fn update_neighbours(&mut self, regions: &Query<&Region>, extras: &mut SimMapData) {
        if self.regions.is_empty() {
            return;
        }
        // Update neighbour list.
        let flip = !self.neighbours.values().next().map(|x| x.2).unwrap_or_default();
        for region in self.regions.iter() {
            let region = if let Ok(region) = regions.get(*region) {
                region
            } else {
                continue;
            };
            let border_regions = region.get_border_regions(&extras);
            for (polity_e, new_set) in border_regions {
                if let Some((old_set, _, flop)) = self.neighbours.get_mut(&polity_e) {
                    old_set.extend(new_set);
                    *flop = flip;
                } else {
                    self.neighbours.insert(polity_e, (new_set, 0.0, flip));
                }
            }
        }
        self.neighbours.retain(|_, v| v.2 == flip);
    }

    pub fn update_diplomacy(
        &mut self,
        config: &AtlasSimConfig,
        polities: &Query<(Entity, &mut Polity)>,
        us_e: Entity,
        extras: &mut SimMapData,
        time: u32,
    ) {
        if self.regions.is_empty() {
            return;
        }
        let mut neighbours = std::mem::take(&mut self.neighbours);
        for (them_e, (_, relation_us, _)) in neighbours.iter_mut() {
            let them_e = *them_e;
            let (_, mut them) = unsafe { polities.get_unchecked(them_e).unwrap() };
            // General stance: x > 0 cooperative, x < 0 competitive, x ~ 0 neutral
            let stance_us = (0.5 - self.policies[POL_COMPETITIVE])
                * 2.0
                * self.get_tech_multiplier(config, SCI_LINGUISTICS);
            let stance_them = (0.5 - them.policies[POL_COMPETITIVE])
                * 2.0
                * them.get_tech_multiplier(config, SCI_LINGUISTICS);
            // Get reference to relations from their POV.
            let (_, relation_them, _) = if let Some(x) = them.neighbours.get_mut(&us_e) {
                x
            } else {
                continue;
            };
            // Relations improve if both cooperative, heavily degrade if both competitive, otherwise slightly degrade.
            let shift = if (stance_us >= 0.0) == (stance_them >= 0.0) {
                (stance_us + stance_them) / 2.0
            } else {
                stance_us.min(stance_them)
            } + config.rules.diplomacy.base_good_shift;
            // Set new relations.
            let new_relations = (*relation_us
                + (shift * config.rules.diplomacy.relations_speed) * (1.4 - *relation_us))
                .clamp(-1.0, 1.0);
            *relation_us = new_relations;
            *relation_them = new_relations;
            // Handle conflicts.
            if new_relations >= config.rules.diplomacy.ally_threshold {
                self.update_diplo_ally(us_e, them_e, &mut them, config, extras);
            } else if new_relations >= config.rules.diplomacy.friend_threshold {
                self.update_diplo_friend(us_e, them_e, &mut them, config, extras);
            } else if new_relations <= config.rules.diplomacy.enemy_threshold {
                // 15 minutes no rush.
                if config.rules.diplomacy.initial_peace_length * 12 >= time {
                    continue;
                }
                // Don't start a war if we're still fighting or if there's a truce.
                let (wars, truce) = extras.war_map.get_war_map_num(us_e, them_e);
                if wars > 0 || truce > 0 {
                    continue;
                }
                // Declare war (polity with higher competitiveness is the attacker).
                let us_attack = self.policies[POL_COMPETITIVE] > them.policies[POL_COMPETITIVE];
                let att_def = if us_attack { (us_e, them_e) } else { (them_e, us_e) };
                let id = extras.create_conflict(time, att_def.0, att_def.1);
                self.join_conflict(us_e, id, extras, us_attack);
                them.join_conflict(them_e, id, extras, !us_attack);
            } else if new_relations <= config.rules.diplomacy.rival_threshold {
                /* Do nothing. */
            }
        }
        self.neighbours = neighbours;
    }

    pub fn update_post_conflict(&mut self, config: &AtlasSimConfig, regions: &mut Query<&mut Region>) {
        if self.regions.is_empty() {
            return;
        }
        if self.conflicts.is_empty() && self.jobs.military > 0.0 {
            self.demobilized_troops += self.jobs.military;
            self.resources_acc[RES_MILITARY] +=
                self.jobs.military * config.rules.combat.equipment_manpower_ratio;
            self.jobs.military = 0.0;
        }
        self.reinforcements = None;
        let adjust_forts = self.fort_damage != 0.0;
        let adjust_pops = (self.civilian_damage != 0.0) || (self.demobilized_troops != 0.0);
        if !adjust_forts && !adjust_pops {
            return;
        }
        let forts_factor = 1.0 - (self.fort_damage / self.capacities[STR_FORTRESS]).max(0.0);
        let pops_factor =
            (self.population + self.demobilized_troops - self.civilian_damage).max(0.0) / self.population;
        self.fort_damage = 0.0;
        self.demobilized_troops = 0.0;
        self.civilian_damage = 0.0;
        if adjust_forts {
            self.capacities[STR_FORTRESS] -= self.fort_damage;
        }
        if adjust_pops {
            self.population = 0.0;
        }
        for region in self.regions.iter() {
            let mut region = if let Ok(region) = regions.get_mut(*region) {
                region
            } else {
                continue;
            };
            if adjust_forts {
                region.structures[STR_FORTRESS] *= forts_factor;
            }
            if adjust_pops {
                region.population = (region.population * pops_factor).max(config.rules.economy.min_pop);
                self.population += region.population;
            }
        }
    }

    pub fn update_splits(&mut self, config: &AtlasSimConfig, time: u32, rng: &mut impl Rng) {
        if self.regions.is_empty() {
            return;
        }
        // Update policies.
        if time >= self.next_policy {
            let normal = Normal::new(config.scenario.policy_mean, config.scenario.policy_deviation).unwrap();
            self.policies = get_random_policies(rng, &normal);
            let normal = Normal::new(
                config.rules.diplomacy.policy_time_mean,
                config.rules.diplomacy.policy_time_dev,
            )
            .unwrap();
            self.next_policy = time + normal.sample(rng).trunc() as u32 * 12;
        }
        // Update manpower split.
        self.manpower_split = [
            0.0,
            2.0 - self.policies[POL_MERCANTILE],
            1.0 + self.policies[POL_MERCANTILE],
        ];
        let sum: f32 = self.manpower_split.iter().sum();
        self.manpower_split = self.manpower_split.map(|x| x / sum);
        // Update industry split.
        let not_military = 1.0 - self.policies[POL_MILITARIST];
        self.indu_split = [1.0 + not_military, 1.0 + self.policies[POL_MILITARIST]];
        let sum: f32 = self.indu_split.iter().sum();
        self.indu_split = self.indu_split.map(|x| x / sum);
        // Update wealth split.
        self.wealth_split = [
            1.0 + not_military * self.policies[POL_PROGRESSIVE],
            1.0 + not_military * (1.0 - self.policies[POL_PROGRESSIVE]),
            1.0 + self.policies[POL_MILITARIST],
        ];
        let sum: f32 = self.wealth_split.iter().sum();
        self.wealth_split = self.wealth_split.map(|x| x / sum);
        // Update technology split.
        self.tech_split = [
            self.policies[POL_EXPANSIONIST],
            1.0 - self.policies[POL_EXPANSIONIST],
            not_military,
            self.policies[POL_MILITARIST],
            1.0 - self.policies[POL_PROGRESSIVE],
            self.policies[POL_PROGRESSIVE],
            self.policies[POL_MILITARIST],
            self.policies[POL_LEGALIST],
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
            1.0 - self.policies[POL_LEGALIST],
            self.policies[POL_LEGALIST],
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
            self.policies[POL_LEGALIST],
            1.0 - self.policies[POL_COMPETITIVE],
        ];
        let sum: f32 = self.struct_split.iter().sum();
        self.struct_split = self.struct_split.map(|x| x / sum);
    }

    pub fn update_diplo_ally(
        &mut self,
        us_e: Entity,
        them_e: Entity,
        them: &mut Polity,
        config: &AtlasSimConfig,
        extras: &mut SimMapData,
    ) {
        for id in them.conflicts.iter() {
            // Don't join a conflict we're already in.
            if self.conflicts.contains(id) {
                continue;
            }
            let conflict = extras.conflicts.get_mut(id).unwrap();
            // Don't join a conflict in which we would fight an ally.
            let invalid = self.neighbours.iter().any(|(e, (_, relation, _))| {
                conflict.is_opposing(&us_e, e)
                    && ((*relation >= config.rules.diplomacy.ally_threshold)
                        || (extras.war_map.get_war_map_num(us_e, them_e).1 > 0))
            });
            if invalid {
                continue;
            }
            // Join defensive wars of allies.
            if conflict.is_primary_defender(them_e) {
                self.join_conflict(us_e, conflict.id, extras, false);
                continue;
            // Join other wars only if not at war.
            } else if self.conflicts.is_empty() {
                // Join offensive wars of allies against unknowns or rivals.
                if conflict.is_primary_attacker(them_e) {
                    if !self
                        .known_and_above(&conflict.primary_defender, config.rules.diplomacy.rival_threshold)
                    {
                        self.join_conflict(us_e, conflict.id, extras, true);
                    }
                // Join support wars of allies if their primary is known and friendly and enemy primary is known and rival.
                } else {
                    let ally_attacker = conflict.is_member(&them_e, true, false);
                    let primary = if ally_attacker {
                        &conflict.primary_attacker
                    } else {
                        &conflict.primary_defender
                    };
                    if !self.known_and_above(primary, config.rules.diplomacy.friend_threshold) {
                        continue;
                    }
                    if self.known_and_below(primary, config.rules.diplomacy.rival_threshold) {
                        continue;
                    }
                    self.join_conflict(us_e, conflict.id, extras, ally_attacker);
                }
            }
        }
    }

    pub fn update_diplo_friend(
        &mut self,
        us_e: Entity,
        them_e: Entity,
        them: &mut Polity,
        config: &AtlasSimConfig,
        extras: &mut SimMapData,
    ) {
        for id in them.conflicts.iter() {
            // Don't join a conflict we're already in.
            if self.conflicts.contains(id) {
                continue;
            }
            let conflict = extras.conflicts.get_mut(id).unwrap();
            // Don't join a conflict in which we would fight an ally.
            let invalid = self.neighbours.iter().any(|(e, (_, relation, _))| {
                conflict.is_opposing(&us_e, e)
                    && ((*relation >= config.rules.diplomacy.ally_threshold)
                        || (extras.war_map.get_war_map_num(us_e, them_e).1 > 0))
            });
            if invalid {
                continue;
            }
            // Join defensive wars of friends against unknowns or rivals.
            if conflict.is_primary_defender(them_e) {
                if !self.known_and_above(&conflict.primary_defender, config.rules.diplomacy.rival_threshold) {
                    self.join_conflict(us_e, conflict.id, extras, true);
                }
            }
        }
    }

    pub fn known_and_above(&self, entity: &Entity, threshold: f32) -> bool {
        self.neighbours
            .get(entity)
            .and_then(|(_, x, _)| Some(*x > threshold))
            .unwrap_or_default()
    }

    pub fn known_and_below(&self, entity: &Entity, threshold: f32) -> bool {
        self.neighbours
            .get(entity)
            .and_then(|(_, x, _)| Some(*x <= threshold))
            .unwrap_or_default()
    }

    pub fn join_conflict(&mut self, us_e: Entity, id: u32, extras: &mut SimMapData, is_attacker: bool) {
        let color = self.color.as_rgba_u8();
        let color = [color[0], color[1], color[2]];
        extras.add_conflict_member(us_e, color, id, is_attacker);
        self.conflicts.insert(id);
    }

    pub fn mobilize(&mut self, config: &AtlasSimConfig) -> (f32, f32) {
        if self.reinforcements.is_some() {
            return self.reinforcements.clone().unwrap();
        }
        let total_pop = self.population + self.jobs.military;
        let recruitable =
            total_pop * (self.policies[POL_MILITARIST] + config.rules.combat.base_recruit_factor).min(1.0);
        let recruits_left = (recruitable - self.jobs.military - self.essential_jobs).max(0.0);
        let recruits = (recruitable * config.rules.combat.mobilization_speed).min(recruits_left);
        let len = self.conflicts.len() as f32;
        let morale = self.resources_acc[RES_LOYALTY] / len;
        self.resources_acc[RES_LOYALTY] = 0.0;
        // Can't go over military service limits.
        if recruits <= 0.0 {
            return (0.0, morale);
        }
        let material = recruits
            .min(self.resources_acc[RES_MILITARY] / config.rules.combat.equipment_manpower_ratio)
            / len;
        self.resources_acc[RES_MILITARY] -= material * len * config.rules.combat.equipment_manpower_ratio;
        self.population -= material * len;
        self.jobs.military += material * len;
        self.reinforcements = Some((material, morale));
        (material, morale)
    }

    pub fn demobilize(&mut self, id: u32, material: f32, morale: f32, config: &AtlasSimConfig) {
        self.conflicts.remove(&id);
        self.jobs.military -= material;
        self.demobilized_troops += material;
        self.resources_acc[RES_MILITARY] += material * config.rules.combat.equipment_manpower_ratio;
        self.resources_acc[RES_LOYALTY] += morale;
    }

    pub fn deal_military_damage(&mut self, damage: f32) {
        self.jobs.military = (self.jobs.military - damage).max(0.0);
    }

    pub fn deal_fort_damage(&mut self, damage: f32) {
        self.fort_damage += damage;
    }

    pub fn deal_civilian_damage(&mut self, damage: f32) {
        self.civilian_damage += damage * self.population;
    }

    fn get_consumption(&self, config: &AtlasSimConfig, res_id: usize) -> f32 {
        let chaos = 1.0 - self.avg_stability;
        (self.population + self.jobs.military)
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
    pub fn get_tech_multiplier(&self, config: &AtlasSimConfig, i: usize) -> f32 {
        let bonus = config.rules.tech.bonus_major * self.tech[i][0].trunc()
            + config.rules.tech.bonus_minor * self.tech[i][1].trunc();
        1.0 + bonus * config.rules.tech.techs[i].strength
    }

    #[inline(always)]
    pub fn get_tradition_multiplier(&self, config: &AtlasSimConfig, i: usize) -> f32 {
        let strength = config.rules.culture.traditions[i].strength
            * (self.traditions[i][0] + self.traditions[i][0]).trunc();
        1.0 + config.rules.culture.level_bonus * strength
    }
}
