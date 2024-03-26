use noise::NoiseFn;

use crate::{
    config::{AdvancedGenerator, PerlinConfig, SimpleGenerator, SimplexConfig, WorldModel},
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
        ViewedMapLayer::Climate => todo!(),
        ViewedMapLayer::Resource => todo!(),
        _ => unreachable!(),
    }
}

/// Choose relevant "advanced" generation procedure based on layer.
pub fn generate_advanced(
    _layer: ViewedMapLayer,
    _logics: &mut MapLogicData,
    _config: &AdvancedGenerator,
    _model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    // TODO
    todo!()
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
    let ocean_level = config.topography.ocean_level;
    let algorithm = config.topography.algorithm;
    // Run the noise algorithm for map topography (height data).
    match algorithm {
        crate::config::SimpleAlgorithm::Perlin(x) => perlin_noise(&mut topo_data, x, model),
        crate::config::SimpleAlgorithm::PerlinFractal(_) => todo!(), // TODO
        crate::config::SimpleAlgorithm::Simplex(x) => simplex_noise(&mut topo_data, x, model),
        crate::config::SimpleAlgorithm::SimplexFractal(_) => todo!(), // TODO
        crate::config::SimpleAlgorithm::DiamondSquare(_) => todo!(), // TODO
    }
    // Globally set the ocean tiles with no flooding.
    for i in 0..cont_data.len() {
        cont_data[i] = if topo_data[i] > ocean_level { 127 } else { 255 }; // TODO
    }
    // Set new layer data.
    logics.layers.insert(ViewedMapLayer::Topography, topo_data);
    logics.layers.insert(ViewedMapLayer::Continents, cont_data);
    return vec![ViewedMapLayer::Continents, ViewedMapLayer::Topography];
}

/// Fill a vector with perlin noise. The world model describes how to interpret linear vector
/// as higher-dimension space.
fn perlin_noise(data: &mut Vec<u8>, config: PerlinConfig, model: &WorldModel) {
    let noise = noise::Perlin::new(config.seed);
    any_noise(data, model, noise, config.scale);
}

/// Fill a vector with perlin noise. The world model describes how to interpret linear vector
/// as higher-dimension space.
fn simplex_noise(data: &mut Vec<u8>, config: SimplexConfig, model: &WorldModel) {
    let noise = noise::Simplex::new(config.seed);
    any_noise(data, model, noise, config.scale);
}

fn any_noise(data: &mut Vec<u8>, model: &WorldModel, noise: impl NoiseFn<f64, 2>, scale: f64) {
    match model {
        WorldModel::Flat(flat) => {
            for y in 0..flat.world_size[1] {
                for x in 0..flat.world_size[0] {
                    let i = (y * flat.world_size[0] + x) as usize;
                    let val = noise.get([x as f64 * scale, y as f64 * scale]) + 1.0f64;
                    data[i] = (val * 128f64 - 1f64) as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
}
