use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex, Perlin, SuperSimplex};

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
    let force_island = config.topography.force_island;
    // Run the noise algorithm for map topography (height data).
    match algorithm {
        SimpleAlgorithm::Perlin => fbm_noise::<Perlin>(&mut topo_data, fbm_config, model, force_island),
        SimpleAlgorithm::OpenSimplex => {
            fbm_noise::<OpenSimplex>(&mut topo_data, fbm_config, model, force_island)
        }
        SimpleAlgorithm::SuperSimplex => {
            fbm_noise::<SuperSimplex>(&mut topo_data, fbm_config, model, force_island)
        }
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

fn fbm_noise<T>(data: &mut Vec<u8>, config: FbmConfig, model: &WorldModel, force_island: bool)
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
    sample_noise(data, model, noise, config.offset, force_island);
}

fn sample_noise(
    data: &mut Vec<u8>,
    model: &WorldModel,
    noise: impl NoiseFn<f64, 2>,
    offset: [f64; 2],
    force_island: bool,
) {
    match model {
        WorldModel::Flat(flat) => {
            let width = flat.world_size[0];
            let height = flat.world_size[1];
            let scale = f64::sqrt((width * height) as f64);
            for y in 0..flat.world_size[1] {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let mut val = noise.get([x as f64 / scale + offset[0], y as f64 / scale + offset[1]]) + 1.0;
                    // TODO: change with a helper layer that influences the heightmap
                    if force_island {
                        let distance = f64::sqrt((width as f64 / 2.0 - x as f64).powi(2) + (height as f64 / 2.0 - y as f64).powi(2));
                        val *= (scale / (distance * 10.0)).clamp(0.0, 1.0);
                    }
                    data[i] = (val * 128f64) as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
}
