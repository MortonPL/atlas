use bevy_egui::egui::lerp;
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex, Perlin, SuperSimplex};

use crate::{
    config::{AdvancedGenerator, FbmConfig, InfluenceCircleConfig, InfluenceMapType, SimpleAlgorithm, SimpleGenerator, WorldModel},
    map::ViewedMapLayer,
};

use super::internal::MapLogicData;

/// Choose relevant "simple" generation procedure based on layer.
pub fn generate_simple(
    layer: ViewedMapLayer,
    logics: &mut MapLogicData,
    config: &SimpleGenerator,
    model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    // TODO
    match layer {
        ViewedMapLayer::Pretty => todo!(),
        ViewedMapLayer::Topography => generate_simple_topography(logics, config, model),
        ViewedMapLayer::TopographyInfluence => generate_simple_topography_influence(logics, config, model),
        ViewedMapLayer::Climate => todo!(),
        ViewedMapLayer::Resource => todo!(),
        _ => unreachable!(),
    }
}

/// Choose relevant "advanced" generation procedure based on layer.
pub fn generate_advanced(
    layer: ViewedMapLayer,
    _logics: &mut MapLogicData,
    _config: &AdvancedGenerator,
    _model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    // TODO
    match layer {
        _ => todo!(),
    }
}

/// Generate simple topography and continental data.
fn generate_simple_topography(
    logics: &mut MapLogicData,
    config: &SimpleGenerator,
    model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut topo_data = logics
        .layers
        .remove(&ViewedMapLayer::Topography)
        .expect("MapLogicData should map all layers");
    let mut cont_data = logics
        .layers
        .remove(&ViewedMapLayer::Continents)
        .expect("MapLogicData should map all layers");
    // Get relevant config info.
    let sea_level = config.topography.sea_level;
    let algorithm = config.topography.algorithm;
    let fbm_config = config.topography.config;
    let use_influence = !matches!(config.topography.influence_map_type, InfluenceMapType::None(_));
    // Run the noise algorithm for map topography (height data).
    generate_noise(&mut topo_data, fbm_config, model, algorithm);
    // Apply the influence map is requested.
    if use_influence {
        let map_data = logics
            .layers
            .get(&ViewedMapLayer::TopographyInfluence)
            .expect("MapLogicData should map all layers");
        apply_influence(&mut topo_data, map_data, config.topography.influence_map_strength);
    }
    // Globally set the ocean tiles with no flooding.
    for i in 0..cont_data.len() {
        cont_data[i] = if topo_data[i] > sea_level { 127 } else { 255 }; // TODO
    }
    // Set new layer data.
    logics.layers.insert(ViewedMapLayer::Topography, topo_data);
    logics.layers.insert(ViewedMapLayer::Continents, cont_data);

    vec![ViewedMapLayer::Continents, ViewedMapLayer::Topography]
}

/// Generate influence map for simple topography.
fn generate_simple_topography_influence(
    logics: &mut MapLogicData,
    config: &SimpleGenerator,
    model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut map_data = logics
        .layers
        .remove(&ViewedMapLayer::TopographyInfluence)
        .expect("MapLogicData should map all layers");
    // Get relevant config info.
    let map_type = &config.topography.influence_map_type;
    match map_type {
        InfluenceMapType::None(_) => unreachable!(),
        InfluenceMapType::Circle(x) => generate_circle(&mut map_data, x, model),
        InfluenceMapType::Strip(x) => todo!(),
        InfluenceMapType::Archipelago(x) => todo!(),
        InfluenceMapType::Fbm(x) => generate_noise(&mut map_data, x.config, model, x.algorithm),
    }
    // Set new layer data.
    logics
        .layers
        .insert(ViewedMapLayer::TopographyInfluence, map_data);
    vec![ViewedMapLayer::TopographyInfluence]
}

fn generate_circle(data: &mut Vec<u8>, config: &InfluenceCircleConfig, model: &WorldModel) {
    let offset = config.offset;
    let radius = config.radius as f32;
    let midpoint = config.midpoint;
    let value = config.midpoint_value;
    match model {
        WorldModel::Flat(flat) => {
            let width = flat.world_size[0];
            let height = flat.world_size[1];
            let x0 = (width / 2 + offset[0]) as i32;
            let y0 = (height / 2 + offset[1]) as i32;
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let val = get_circle_value(x as i32, x0, y as i32, y0, radius, midpoint, value);
                    data[i] = (val * 255f32) as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
}

fn get_circle_value(x: i32, x0: i32, y: i32, y0: i32, r: f32, midpoint: f32, value: f32) -> f32 {
    // Calculate the distance from circle center.
    let len = (((x - x0).pow(2) + (y - y0).pow(2)) as f32).sqrt();
    // Transform the distance as a fraction of radius.
    let norm = (len / r).clamp(0.0, 1.0);
    // Interpolate value using the midpoint and midpoint value.
    if norm <= midpoint {
        lerp(1.0..=value, norm / midpoint)
    } else {
        lerp(value..=0.0, (norm - midpoint) / (1.0 - midpoint))
    }
}

fn generate_noise(data: &mut Vec<u8>, config: FbmConfig, model: &WorldModel, algorithm: SimpleAlgorithm) {
    match algorithm {
        SimpleAlgorithm::Perlin => read_fbm_config_and_run::<Perlin>(data, config, model),
        SimpleAlgorithm::OpenSimplex => read_fbm_config_and_run::<OpenSimplex>(data, config, model),
        SimpleAlgorithm::SuperSimplex => read_fbm_config_and_run::<SuperSimplex>(data, config, model),
    }
}

fn read_fbm_config_and_run<T>(data: &mut Vec<u8>, config: FbmConfig, model: &WorldModel)
where
    T: noise::Seedable,
    T: std::default::Default,
    T: NoiseFn<f64, 2>,
{
    let noise = Fbm::<T>::new(config.seed)
        .set_octaves(config.detail)
        .set_frequency(config.frequency)
        .set_lacunarity(config.neatness)
        .set_persistence(config.roughness);
    sample_noise(data, model, noise, config.offset);
}

fn sample_noise(data: &mut Vec<u8>, model: &WorldModel, noise: impl NoiseFn<f64, 2>, offset: [f64; 2]) {
    match model {
        WorldModel::Flat(flat) => {
            let width = flat.world_size[0];
            let height = flat.world_size[1];
            let scale = f64::sqrt((width * height) as f64);
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let val = noise.get([x as f64 / scale + offset[0], y as f64 / scale + offset[1]]) + 1.0;
                    data[i] = (val * 128f64) as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
}

fn apply_influence(data: &mut Vec<u8>, influence: &Vec<u8>, strength: f32) {
    for i in 0..data.len() {
        let inf = 1.0 - (1.0 - influence[i] as f32 / 255.0) * strength;
        data[i] = (data[i] as f32 * inf) as u8;
    }
}
