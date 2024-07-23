use atlas_lib::{
    bevy::{
        prelude::*,
        utils::{hashbrown::HashSet, HashMap},
    },
    config::{
        sim::{AtlasSimConfig, StartCivAlgorithm, StartPointAlgorithm},
        AtlasConfig,
    },
    domain::{
        graphics::MapLogicData,
        map::{MapDataLayer, MapDataOverlay},
    },
    rand::{distributions::Uniform, Rng},
};
use weighted_rand::{
    builder::{NewBuilder, WalkerTableBuilder},
    table::WalkerTable,
};

use crate::ui::MapOverlay;

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
    let mut success = true;
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

pub fn randomize_point_civ(config: &mut AtlasSimConfig, rng: &mut impl Rng) {
    match config.scenario.random_civ_algorithm {
        StartCivAlgorithm::Repeated => {
            for point in &mut config.scenario.start_points {
                if point.civ_locked {
                    continue;
                }
                point.civ = rng.gen_range(0..config.scenario.num_civs);
            }
        }
        StartCivAlgorithm::Choice => {
            let mut num = config.scenario.num_civs;
            let mut civs: HashSet<u8> = (0..config.scenario.num_civs).collect();
            for point in &mut config.scenario.start_points {
                if point.civ_locked {
                    civs.remove(&point.civ);
                    num -= 1;
                }
            }
            let mut civs: Vec<u8> = civs.into_iter().collect();
            for point in &mut config.scenario.start_points {
                if point.civ_locked {
                    continue;
                }
                if num > 0 {
                    let i = rng.gen_range(0..num) as usize;
                    point.civ = civs.remove(i as usize);
                    num -= 1;
                } else {
                    point.civ = 0;
                }
            }
        }
    }
}

pub fn randomize_point_color(config: &mut AtlasSimConfig, rng: &mut impl Rng) {
    let uniform_h = Uniform::new_inclusive(0.0, 360.0);
    let uniform_s = Uniform::new_inclusive(0.7, 1.0);
    let uniform_l = Uniform::new_inclusive(0.5, 1.0);
    for point in &mut config.scenario.start_points {
        if point.color_locked {
            continue;
        }
        let (h, s, l) = (
            rng.sample(uniform_h),
            rng.sample(uniform_s),
            rng.sample(uniform_l),
        );
        let color = Color::hsl(h, s, l).as_rgba_u8();
        point.polity.color = [color[0], color[1], color[2]];
    }
}

pub fn create_overlays(
    config: &AtlasSimConfig,
    mut commands: Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mut query: Query<(Entity, &MapOverlay), With<MapOverlay>>,
) {
    // Despawn old markers.
    for (entity, overlay) in query.iter_mut() {
        match overlay.overlay {
            MapDataOverlay::StartPoints => commands.entity(entity).despawn(),
            _ => {}
        }
    }
    // Create new meshes and materials.
    let mesh = meshes.add(Cuboid::from_size(Vec3::ONE / 50.0).mesh());
    // Spawn new markers.
    for point in &config.scenario.start_points {
        let coords = config.map_to_world((point.position[0], point.position[1]));
        commands.spawn((
            MaterialMeshBundle {
                mesh: mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb_u8(
                        point.polity.color[0],
                        point.polity.color[1],
                        point.polity.color[2],
                    ),
                    unlit: true,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(coords.0, coords.1, 0.0),
                ..default()
            },
            MapOverlay::new(MapDataOverlay::StartPoints),
        ));
    }
}

pub fn make_similar_color(color: &Color, rng: &mut impl Rng) -> Color {
    let uniform = Uniform::new_inclusive(-0.1, 0.1);
    let hsla = color.as_hsla_f32();
    let mut h = hsla[0] + rng.sample(uniform) * 50.0;
    let s = (hsla[1] + rng.sample(uniform)).clamp(0.7, 1.0);
    let l = (hsla[2] + rng.sample(uniform)).clamp(0.5, 1.0);
    if h > 360.0 {
        h -= 360.0;
    } else if h < 0.0 {
        h += 360.0;
    }
    Color::hsl(h, s, l)
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
