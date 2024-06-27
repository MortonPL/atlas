use atlas_lib::{
    bevy::utils::{hashbrown::HashSet, HashMap},
    domain::{graphics::MapLogicData, map::MapDataLayer},
    rand::Rng,
};
use weighted_rand::{
    builder::{NewBuilder, WalkerTableBuilder},
    table::WalkerTable,
};

use crate::config::{AtlasSimConfig, StartCivAlgorithm, StartPointAlgorithm};

/// Calculate weights for random choice of starting points. Returns all weights and weight strips (horizontal).
pub fn calc_start_point_weights(
    config: &AtlasSimConfig,
    logics: &mut MapLogicData,
    width: usize,
    height: usize,
) -> (Vec<u32>, Vec<u32>) {
    // Baseline: unweighted choice between all continental tiles.
    let conts = logics.get_layer(MapDataLayer::Continents);
    let mut weights: Vec<u32> = conts.iter().map(|x| if *x <= 127 { 0 } else { 1 }).collect();
    // Weight by habitability if requested.
    match config.scenario.random_point_algorithm {
        // Calculate weights per tile.
        StartPointAlgorithm::Weighted => {
            let climate = logics.get_layer(MapDataLayer::Climate);
            weights = habitability_weight(&mut weights, climate, config).collect();
        }
        // Calculate weights per tile (squared habitability).
        StartPointAlgorithm::WeightedSquared => {
            let climate = logics.get_layer(MapDataLayer::Climate);
            weights = habitability_weight(&mut weights, climate, config)
                .map(|x| x * x)
                .collect();
        }
        // Calculate weights in 8-neighbourhood.
        StartPointAlgorithm::WeightedArea => {
            let climate = logics.get_layer(MapDataLayer::Climate);
            let part_weights: Vec<u32> = habitability_weight(&mut weights, climate, config).collect();
            kernel_sum(&part_weights, &mut weights, width, height);
        }
        // Calculate weights in 8-neighbourhood (squared habitability).
        StartPointAlgorithm::WeightedSquaredArea => {
            let climate = logics.get_layer(MapDataLayer::Climate);
            let part_weights: Vec<u32> = habitability_weight(&mut weights, climate, config)
                .map(|x| x * x)
                .collect();
            kernel_sum(&part_weights, &mut weights, width, height);
        }
        StartPointAlgorithm::Uniform => {}
    };
    // Prep one-tile-high strips for speedup.
    let strip_weights: Vec<u32> = match config.scenario.random_point_algorithm {
        StartPointAlgorithm::Uniform => weights
            .chunks(width)
            .map(|x| x.iter().sum::<u32>().max(1))
            .collect(),
        _ => weights
            .chunks(width)
            .map(|x| x.iter().sum::<u32>() / x.len() as u32)
            .collect(),
    };
    (weights, strip_weights)
}

pub fn randomize_start_points(
    config: &mut AtlasSimConfig,
    rng: &mut impl Rng,
    weights: &[u32],
    strip_weights: &[u32],
    width: usize,
) -> bool {
    let mut success = false;
    let strip_table = WalkerTableBuilder::new(strip_weights).build();
    let mut tables = HashMap::<usize, WalkerTable>::default();
    // Randomize all points.
    let mut used_positions = HashSet::<usize>::default();
    for point in &mut config.scenario.start_points {
        if point.position_locked {
            continue;
        }
        // Ensure that the position is not in use. If it is, try again. If that fails too, show an error.
        success = false;
        for _ in 0..5 {
            // Get or make the table for this strip.
            let i = strip_table.next_rng(rng);
            let table = if let Some(table) = tables.get(&i) {
                table
            } else {
                let start = i * width;
                let table = WalkerTableBuilder::new(&weights[start..(start + width)]).build();
                let (_, table) = tables.insert_unique_unchecked(i, table);
                table
            };
            let j = table.next_rng(rng);
            let j = i * width + j;
            if !used_positions.contains(&j) {
                used_positions.insert(j);
                success = true;
                point.position[0] = (j % width) as u32;
                point.position[1] = (j / width) as u32;
                break;
            }
        }
    }
    success
}

pub fn randomize_civ_points(config: &mut AtlasSimConfig, rng: &mut impl Rng) {
    match config.scenario.random_civ_algorithm {
        StartCivAlgorithm::Repeated => {
            for point in &mut config.scenario.start_points {
                if point.owner_locked {
                    continue;
                }
                point.owner = rng.gen_range(0..config.scenario.num_civs);
            }
        }
        StartCivAlgorithm::Choice => {
            let mut num = config.scenario.num_civs;
            let mut civs: HashSet<u8> = (0..config.scenario.num_civs).collect();
            for point in &mut config.scenario.start_points {
                if point.owner_locked {
                    civs.remove(&point.owner);
                    num -= 1;
                }
            }
            let mut civs: Vec<u8> = civs.into_iter().collect();
            for point in &mut config.scenario.start_points {
                if point.owner_locked {
                    continue;
                }
                if num > 0 {
                    let i = rng.gen_range(0..num) as usize;
                    point.owner = civs.remove(i as usize);
                    num -= 1;
                } else {
                    point.owner = 0;
                }
            }
        }
    }
}

fn habitability_weight<'a>(
    weights: &'a mut [u32],
    climate: &'a [u8],
    config: &'a AtlasSimConfig,
) -> impl std::iter::Iterator<Item = u32> + 'a {
    climate
        .iter()
        .zip(weights.iter())
        .map(|(c, w)| (config.get_biome(*c).habitability * 1000.0) as u32 * w)
}

fn kernel_sum(input: &[u32], output: &mut [u32], width: usize, height: usize) {
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let i = width * y + x;
            if input[i] < 100 {
                output[i] = 0;
            } else {
                let (up, down) = (i - width, i + width);
                output[i] = input[i - 1]
                    + input[i]
                    + input[i + 1]
                    + input[up - 1]
                    + input[up]
                    + input[up + 1]
                    + input[down - 1]
                    + input[down]
                    + input[down + 1];
            }
        }
    }
}
