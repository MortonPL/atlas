use bevy::math::Vec2;
use bevy_egui::egui::{ecolor::rgb_from_hsv, lerp};
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex, Perlin, SuperSimplex};

use crate::{
    config::{
        AdvancedGenerator, FbmConfig, InfluenceCircleConfig, InfluenceMapType, InfluenceStripConfig,
        SimpleAlgorithm, SimpleGenerator, WorldModel,
    },
    map::{internal::climate_to_hsv, ViewedMapLayer},
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
        ViewedMapLayer::Preview => generate_simple_preview(logics, config, model),
        ViewedMapLayer::Continents => generate_simple_continents(logics, config, model),
        ViewedMapLayer::ContinentsInfluence => {
            generate_simple_influence(logics, &config.continents.influence_map_type, model, layer)
        }
        ViewedMapLayer::Topography => generate_simple_topography(logics, config, model),
        ViewedMapLayer::TopographyInfluence => {
            generate_simple_influence(logics, &config.topography.influence_map_type, model, layer)
        }
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

/// Generate pretty map preview.
fn generate_simple_preview(
    logics: &mut MapLogicData,
    config: &SimpleGenerator,
    model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut preview_data = logics
        .layers
        .remove(&ViewedMapLayer::Preview)
        .expect("MapLogicData should map all layers");
    let topo_data = logics
        .layers
        .get(&ViewedMapLayer::Topography)
        .expect("MapLogicData should map all layers");
    let cont_data = logics
        .layers
        .get(&ViewedMapLayer::Continents)
        .expect("MapLogicData should map all layers");
    let climate_data = logics
        .layers
        .get(&ViewedMapLayer::Climate)
        .expect("MapLogicData should map all layers");

    for i in 0..topo_data.len() {
        let (mut r, mut g, mut b) = (0, 0, 0);
        if is_sea(cont_data[i]) {
            (r, g, b) = (0, 160, 255);
        } else {
            let height = topo_data[i] as f32 / 300.0;//255.0;
            let (h, s, v) = climate_to_hsv(climate_data[i]);
            let v = v * (((1.0 - height) * 10.0).round() / 10.0);
            let rgb = rgb_from_hsv((h, s, v));
            (r, g, b) = (
                (rgb[0] * 255.0) as u8,
                (rgb[1] * 255.0) as u8,
                (rgb[2] * 255.0) as u8,
            );
        }
        let j = i * 4;
        preview_data[j] = r;
        preview_data[j + 1] = g;
        preview_data[j + 2] = b;
        preview_data[j + 3] = 255;
    }

    // Set new layer data.
    logics.layers.insert(ViewedMapLayer::Preview, preview_data);

    vec![ViewedMapLayer::Preview]
}

/// Generate simple continental data.
fn generate_simple_continents(
    logics: &mut MapLogicData,
    config: &SimpleGenerator,
    model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut cont_data = logics
        .layers
        .remove(&ViewedMapLayer::Continents)
        .expect("MapLogicData should map all layers");
    // Get relevant config info.
    let sea_level = config.continents.sea_level;
    let algorithm = config.continents.algorithm;
    let fbm_config = config.continents.config;
    let use_influence = !matches!(config.continents.influence_map_type, InfluenceMapType::None(_));
    // Run the noise algorithm to obtain height data for continental discrimination.
    generate_noise(&mut cont_data, fbm_config, model, algorithm);
    // Apply the influence map if requested.
    if use_influence {
        let map_data = logics
            .layers
            .get(&ViewedMapLayer::ContinentsInfluence)
            .expect("MapLogicData should map all layers");
        apply_influence(&mut cont_data, map_data, config.continents.influence_map_strength);
    }
    // Globally set the ocean tiles with no flooding.
    for i in 0..cont_data.len() {
        cont_data[i] = if cont_data[i] > sea_level { 255 } else { 127 };
    }
    // Set new layer data.
    logics.layers.insert(ViewedMapLayer::Continents, cont_data);
    generate_simple_topo_filter(logics, config, model);
    // Regenerate real topography.
    generate_simple_real_topo(logics, config, model);

    vec![ViewedMapLayer::Continents]
}

/// Generate simple topography data.
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
    // Get relevant config info.
    let algorithm = config.topography.algorithm;
    let fbm_config = config.topography.config;
    let use_influence = !matches!(config.topography.influence_map_type, InfluenceMapType::None(_));
    // Run the noise algorithm for map topography (height data).
    generate_noise(&mut topo_data, fbm_config, model, algorithm);
    // Apply the influence map if requested.
    if use_influence {
        let map_data = logics
            .layers
            .get(&ViewedMapLayer::TopographyInfluence)
            .expect("MapLogicData should map all layers");
        apply_influence(&mut topo_data, map_data, config.topography.influence_map_strength);
    }
    // Set new layer data.
    logics.layers.insert(ViewedMapLayer::Topography, topo_data);
    // Regenerate real topography.
    generate_simple_real_topo(logics, config, model);

    vec![ViewedMapLayer::Topography, ViewedMapLayer::TopographyFilter] // DEBUG
}

/// Generate simple FINAL topography data.
fn generate_simple_real_topo(
    logics: &mut MapLogicData,
    config: &SimpleGenerator,
    model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut real_data = logics
        .layers
        .remove(&ViewedMapLayer::RealTopography)
        .expect("MapLogicData should map all layers");
    let topo_data = logics
        .layers
        .get(&ViewedMapLayer::Topography)
        .expect("MapLogicData should map all layers");
    let filter_data = logics
        .layers
        .get(&ViewedMapLayer::TopographyFilter)
        .expect("MapLogicData should map all layers");

    for i in 0..real_data.len() {
        real_data[i] = (topo_data[i] as f32 * ((255 - filter_data[i]) as f32 / 255.0)) as u8;
    }

    // Set new layer data.
    logics.layers.insert(ViewedMapLayer::RealTopography, real_data);

    vec![ViewedMapLayer::RealTopography]
}

fn generate_simple_topo_filter(
    logics: &mut MapLogicData,
    config: &SimpleGenerator,
    model: &WorldModel,
) -> Vec<ViewedMapLayer> {
    let mut filter_data = logics
        .layers
        .remove(&ViewedMapLayer::TopographyFilter)
        .expect("MapLogicData should map all layers");
    let cont_data = logics
        .layers
        .get(&ViewedMapLayer::Continents)
        .expect("MapLogicData should map all layers");

    filter_data.fill(0);
    match model {
        WorldModel::Flat(x) => {
            let width = x.world_size[0];
            let height = x.world_size[1];
            let kernel: u32 = 4;
            let multiplier = (255 / ((kernel.pow(2) - 1) * 2)) as u8;
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    // Case water: add smoothing *to* nearby tiles.
                    if is_sea(cont_data[i]) {
                        for v in 0..kernel {
                            for u in 0..kernel {
                                if ((y + v) >= height) || ((x + u) >= width) {
                                    continue;
                                }
                                let j = ((y + v) * width + (x + u)) as usize;
                                filter_data[j] += 1;
                            }
                        }
                        filter_data[i] = 0;
                    // Case land: add smoothing *from* nearby tiles.
                    } else {
                        let mut value = 0;
                        for v in 0..kernel {
                            for u in 0..kernel {
                                if ((y + v) >= height) || ((x + u) >= width) {
                                    continue;
                                }
                                let j = ((y + v) * width + (x + u)) as usize;
                                if is_sea(cont_data[j]) {
                                    value += 1;
                                }
                            }
                        }
                        value = (filter_data[i] + value) * multiplier;
                        filter_data[i] = if value > 0 { value.max(multiplier) } else { 0 };
                    };
                }
            }
        }
        WorldModel::Globe(_) => todo!(),
    }

    // Set new layer data.
    logics
        .layers
        .insert(ViewedMapLayer::TopographyFilter, filter_data);

    vec![ViewedMapLayer::TopographyFilter]
}

/// Generate influence map for a layer.
fn generate_simple_influence(
    logics: &mut MapLogicData,
    map_type: &InfluenceMapType,
    model: &WorldModel,
    layer: ViewedMapLayer,
) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut map_data = logics
        .layers
        .remove(&layer)
        .expect("MapLogicData should map all layers");
    // Get relevant config info.
    match map_type {
        InfluenceMapType::None(_) => unreachable!(),
        InfluenceMapType::FromImage(_) => unreachable!(),
        InfluenceMapType::Circle(x) => generate_circle(&mut map_data, x, model),
        InfluenceMapType::Strip(x) => generate_strip(&mut map_data, x, model),
        InfluenceMapType::Fbm(x) => generate_noise(&mut map_data, x.config, model, x.algorithm),
    }
    // Set new layer data.
    logics.layers.insert(layer, map_data);
    vec![layer]
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
    // See if the point is within first or second end circle.
    } else {
        let len = p.distance(p1).min(p.distance(p2));
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
        SimpleAlgorithm::FromImage => {}
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
    sample_noise(data, model, noise, config.offset, config.bias, config.range);
}

fn sample_noise(data: &mut [u8], model: &WorldModel, noise: impl NoiseFn<f64, 2>, offset: [f64; 2], bias: i16, range: f64) {
    match model {
        WorldModel::Flat(flat) => {
            let width = flat.world_size[0];
            let height = flat.world_size[1];
            let scale = f64::sqrt((width * height) as f64);
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let val = (noise.get([x as f64 / scale + offset[0], y as f64 / scale + offset[1]]) + 1.0) * range;
                    data[i] = (val * 128f64 + bias as f64).clamp(0.0, 255.0) as u8;
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

fn is_sea(value: u8) -> bool {
    value <= 127
}
