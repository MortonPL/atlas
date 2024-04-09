use bevy::math::Vec2;
use bevy_egui::egui::lerp;
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex, Perlin, SuperSimplex};

use crate::{
    config::{
        AdvancedGenerator, FbmConfig, InfluenceCircleConfig, InfluenceMapType, InfluenceStripConfig,
        SimpleAlgorithm, SimpleGenerator, WorldModel,
    },
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
        InfluenceMapType::Strip(x) => generate_strip(&mut map_data, x, model),
        InfluenceMapType::Archipelago(x) => todo!(),
        InfluenceMapType::Fbm(x) => generate_noise(&mut map_data, x.config, model, x.algorithm),
    }
    // Set new layer data.
    logics
        .layers
        .insert(ViewedMapLayer::TopographyInfluence, map_data);
    vec![ViewedMapLayer::TopographyInfluence]
}

fn generate_circle(data: &mut [u8], config: &InfluenceCircleConfig, model: &WorldModel) {
    let offset = config.offset;
    let radius = config.radius as f32;
    let midpoint = config.midpoint;
    let value = config.midpoint_value;
    match model {
        WorldModel::Flat(flat) => {
            let width = flat.world_size[0];
            let height = flat.world_size[1];
            let p0 = Vec2::new((width / 2 + offset[0]) as f32, (height / 2 + offset[1]) as f32);
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let p = Vec2::new(x as f32, y as f32);
                    let val = get_circle_value(p, p0, radius, midpoint, value);
                    data[i] = (val * 255f32) as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
}

fn generate_strip(data: &mut [u8], config: &InfluenceStripConfig, model: &WorldModel) {
    let offset = config.offset;
    let thickness = config.thickness as f32;
    let length = config.length as f32;
    let angle = (config.angle as f32).to_radians();
    let flip = config.flip;
    let midpoint = config.midpoint;
    let value = config.midpoint_value;
    match model {
        WorldModel::Flat(flat) => {
            let width = flat.world_size[0];
            let height = flat.world_size[1];
            let p0 = Vec2::new((width / 2 + offset[0]) as f32, (height / 2 + offset[1]) as f32);
            let (p1, p2, a, b) = precalculate_strip(p0, length, angle, flip);
            let l2 = length / 2.0;
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let p = Vec2::new(x as f32, y as f32);
                    let val = get_strip_value(p, p0, p1, p2, thickness, l2, a, b, midpoint, value);
                    data[i] = (val * 255f32) as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
}

fn get_circle_value(p: Vec2, p0: Vec2, r: f32, midpoint: f32, value: f32) -> f32 {
    // Calculate the distance from circle center.
    let len = p.distance(p0);
    // Transform the distance as a fraction of radius.
    let norm = (len / r).clamp(0.0, 1.0);
    // Interpolate value using the midpoint and midpoint value.
    if norm <= midpoint {
        lerp(1.0..=value, norm / midpoint)
    } else {
        lerp(value..=0.0, (norm - midpoint) / (1.0 - midpoint))
    }
}

fn precalculate_strip(p0: Vec2, l: f32, a: f32, flip: bool) -> (Vec2, Vec2, f32, f32) {
    // Tan(alpha)
    let mut tana = a.tan();
    if flip {
        tana = -tana;
    }
    // Cos(alpha), sin(alpha)
    let triga = Vec2::from_angle(a);
    // Half of width side
    let d = triga * l / 2.0;
    // Line formula b
    let b = p0.y - tana * p0.x;
    // Return values
    let mut p1 = p0 - d;
    let mut p2 = p0 + d;
    if flip {
        p1 = Vec2::new(p0.x + d.x, p0.y - d.y);
        p2 = Vec2::new(p0.x - d.x, p0.y + d.y);
    }
    (p1, p2, tana, b)
}

fn get_strip_value(
    p: Vec2,
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    r: f32,
    l2: f32,
    a: f32,
    b: f32,
    midpoint: f32,
    value: f32,
) -> f32 {
    let mut norm = 1f32;
    // Project point on strip line and see if it's close enough.
    let (pp, len) = project_to_line(p, a, b);
    if p0.distance(pp) <= l2 {
        norm = (len / r).min(1.0);
    } else {
        // See if the point is within one end circle.
        let mut len = p.distance(p1);
        if len > r {
            // See if the point is within the other end circle.
            len = p.distance(p2);
        }
        if len <= r {
            norm = len / r;
        }
    }
    // Interpolate value using the midpoint and midpoint value.
    if norm <= midpoint {
        lerp(1.0..=value, norm / midpoint)
    } else {
        lerp(value..=0.0, (norm - midpoint) / (1.0 - midpoint))
    }
}

fn project_to_line(p: Vec2, a: f32, b: f32) -> (Vec2, f32) {
    let x;
    let y;
    if a == 0.0 {
        x = p.x;
        y = b;
    } else {
        let a2 = -a.recip();
        let b2 = p.y - a2 * p.x;
        let a_a2 = a - a2;
        x = (b2 - b) / a_a2;
        y = (a * b2 - b * a2) / a_a2;
    }
    let pp = Vec2::new(x, y);
    (pp, pp.distance(p))
}

fn generate_noise(data: &mut Vec<u8>, config: FbmConfig, model: &WorldModel, algorithm: SimpleAlgorithm) {
    match algorithm {
        SimpleAlgorithm::Perlin => read_fbm_config_and_run::<Perlin>(data, config, model),
        SimpleAlgorithm::OpenSimplex => read_fbm_config_and_run::<OpenSimplex>(data, config, model),
        SimpleAlgorithm::SuperSimplex => read_fbm_config_and_run::<SuperSimplex>(data, config, model),
    }
}

fn read_fbm_config_and_run<T>(data: &mut [u8], config: FbmConfig, model: &WorldModel)
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

fn sample_noise(data: &mut [u8], model: &WorldModel, noise: impl NoiseFn<f64, 2>, offset: [f64; 2]) {
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

fn apply_influence(data: &mut [u8], influence: &[u8], strength: f32) {
    for i in 0..data.len() {
        let inf = 1.0 - (1.0 - influence[i] as f32 / 255.0) * strength;
        data[i] = (data[i] as f32 * inf) as u8;
    }
}
