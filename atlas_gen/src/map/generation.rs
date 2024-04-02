use noise::{Fbm, MultiFractal, NoiseFn, Perlin, Simplex};

use crate::{
    config::{AdvancedGenerator, FbmConfig, SimpleAlgorithm, SimpleGenerator, WorldModel},
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
        SimpleAlgorithm::Perlin(x) => fbm_noise::<Perlin>(&mut topo_data, x, model),
        SimpleAlgorithm::Simplex(x) => fbm_noise::<Simplex>(&mut topo_data, x, model),
        SimpleAlgorithm::DiamondSquare(_) => todo!(), // TODO
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

fn fbm_noise<T>(data: &mut Vec<u8>, config: FbmConfig, model: &WorldModel)
where
    T: noise::Seedable,
    T: std::default::Default,
    T: NoiseFn<f64, 2>,
{
    let noise = Fbm::<T>::new(config.seed)
        .set_octaves(config.detail)
        .set_frequency(config.frequency)
        .set_lacunarity(config.scale)
        .set_persistence(config.smoothness);
    any_noise(data, model, noise, config.offset)
}

fn any_noise(data: &mut Vec<u8>, model: &WorldModel, noise: impl NoiseFn<f64, 2>, offset: [f64; 2]) {
    match model {
        WorldModel::Flat(flat) => {
            for y in 0..flat.world_size[1] {
                for x in 0..flat.world_size[0] {
                    let i = (y * flat.world_size[0] + x) as usize;
                    let val = noise.get([x as f64 + offset[0], y as f64 + offset[1]]) + 1.0;
                    data[i] = (val * 128f64) as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
}
