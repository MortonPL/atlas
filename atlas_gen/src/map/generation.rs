use bevy::utils::petgraph::matrix_graph::Zero;
use bevy_egui::egui::ecolor::rgb_from_hsv;

use crate::{
    config::{InfluenceShape, SessionConfig, TopographyDisplayMode, WorldModel},
    map::{
        internal::{climate_to_hsv, MapLogicData},
        samplers::{apply_influence, fill_influence, fill_noise},
        ViewedMapLayer,
    },
};

/// Choose relevant generation procedure based on layer.
pub fn generate(
    layer: ViewedMapLayer,
    logics: &mut MapLogicData,
    config: &SessionConfig,
) -> Vec<ViewedMapLayer> {
    let model = &config.general.world_model;
    match layer {
        ViewedMapLayer::Preview => generate_preview(logics, config),
        ViewedMapLayer::Continents => generate_continents(logics, config),
        ViewedMapLayer::ContinentsInfluence => {
            generate_influence(logics, &config.continents.influence_map_type, model, layer)
        }
        ViewedMapLayer::Topography => generate_topography(logics, config),
        ViewedMapLayer::TopographyInfluence => {
            generate_influence(logics, &config.topography.influence_map_type, model, layer)
        }
        ViewedMapLayer::Climate => todo!(),  // TODO
        ViewedMapLayer::Resource => todo!(), // TODO
        _ => unreachable!(),
    }
}

/// Generate pretty map preview.
fn generate_preview(logics: &mut MapLogicData, config: &SessionConfig) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut preview_data = logics.pop_layer(ViewedMapLayer::Preview);
    let real_data = logics.get_layer(ViewedMapLayer::RealTopography);
    let cont_data = logics.get_layer(ViewedMapLayer::Continents);
    let climate_data = logics.get_layer(ViewedMapLayer::Climate);

    let highest = match config.general.topo_display {
        TopographyDisplayMode::Absolute => 255.0,
        TopographyDisplayMode::Highest => {
            *real_data.iter().max().expect("RealTopography must not be empty") as f32
        }
    };
    for i in 0..real_data.len() {
        let (r, g, b);
        if is_sea(cont_data[i]) {
            (r, g, b) = (0, 160, 255);
        } else {
            let height = real_data[i] as f32 / (highest * 1.2);
            let (h, s, v) = climate_to_hsv(climate_data[i]);
            let v = v * (((1.0 - height.clamp(0.2, 0.8)) * 10.0).round() / 10.0);
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
    logics.put_layer(ViewedMapLayer::Preview, preview_data);

    vec![ViewedMapLayer::Preview]
}

/// Generate continental data.
fn generate_continents(logics: &mut MapLogicData, config: &SessionConfig) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut cont_data = logics.pop_layer(ViewedMapLayer::Continents);
    // Get relevant config info.
    let algorithm = config.continents.algorithm;
    let fbm_config = &config.continents.config;
    let model = &config.general.world_model;
    let use_influence = !matches!(config.continents.influence_map_type, InfluenceShape::None(_));

    // Run the noise algorithm to obtain height data for continental discrimination.
    fill_noise(&mut cont_data, fbm_config, model, algorithm);
    // Apply the influence map if requested.
    if use_influence {
        let map_data = logics.get_layer(ViewedMapLayer::ContinentsInfluence);
        apply_influence(&mut cont_data, map_data, config.continents.influence_map_strength);
    }
    // Globally set the ocean tiles with no flooding.
    if config.continents.sea_level.is_zero() {
        for value in &mut cont_data {
            *value = 255;
        }
    } else if config.continents.sea_level == 1.0 {
        for value in &mut cont_data {
            *value = 127;
        }
    } else {
        let sea_level = (255.0 * config.continents.sea_level) as u8;
        for value in &mut cont_data {
            *value = if *value > sea_level { 255 } else { 127 };
        }
    }

    // Set new layer data.
    logics.put_layer(ViewedMapLayer::Continents, cont_data);

    // Regenerate real topography.
    generate_utility_topo_filter(logics, config);
    generate_utility_real_topo(logics);

    vec![
        ViewedMapLayer::Continents,
        ViewedMapLayer::RealTopography,
        ViewedMapLayer::TopographyFilter,
    ] // DEBUG
}

/// Generate topography data.
fn generate_topography(logics: &mut MapLogicData, config: &SessionConfig) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut topo_data = logics.pop_layer(ViewedMapLayer::Topography);
    // Get relevant config info.
    let algorithm = config.topography.algorithm;
    let fbm_config = &config.topography.config;
    let model = &config.general.world_model;
    let use_influence = !matches!(config.continents.influence_map_type, InfluenceShape::None(_));

    // Run the noise algorithm for map topography (height data).
    fill_noise(&mut topo_data, fbm_config, model, algorithm);
    // Apply the influence map if requested.
    if use_influence {
        let map_data = logics.get_layer(ViewedMapLayer::TopographyInfluence);
        apply_influence(&mut topo_data, map_data, config.continents.influence_map_strength);
    }
    // Set new layer data.
    logics.put_layer(ViewedMapLayer::Topography, topo_data);

    // Regenerate real topography.
    generate_utility_topo_filter(logics, config);
    generate_utility_real_topo(logics);

    vec![
        ViewedMapLayer::Topography,
        ViewedMapLayer::RealTopography,
        ViewedMapLayer::TopographyFilter,
    ] // DEBUG
}

/// Generate FINAL topography data.
fn generate_utility_real_topo(logics: &mut MapLogicData) -> Vec<ViewedMapLayer> {
    // Move out layer data.
    let mut real_data = logics.pop_layer(ViewedMapLayer::RealTopography);
    let topo_data = logics.get_layer(ViewedMapLayer::Topography);
    let filter_data = logics.get_layer(ViewedMapLayer::TopographyFilter);

    for i in 0..real_data.len() {
        real_data[i] = (topo_data[i] as f32 * ((255 - filter_data[i]) as f32 / 255.0)) as u8;
    }

    // Set new layer data.
    logics.put_layer(ViewedMapLayer::RealTopography, real_data);

    vec![ViewedMapLayer::RealTopography]
}

/// Generate beach smoothing topography filter.
fn generate_utility_topo_filter(logics: &mut MapLogicData, config: &SessionConfig) -> Vec<ViewedMapLayer> {
    let mut filter_data = logics.pop_layer(ViewedMapLayer::TopographyFilter);
    filter_data.fill(0);

    let kernel: i32 = config.topography.coastal_erosion as i32;
    if kernel == 0 {
        return vec![];
    }

    let cont_data = logics.get_layer(ViewedMapLayer::Continents);

    match &config.general.world_model {
        WorldModel::Flat(x) => {
            let width = x.world_size[0] as i32;
            let height = x.world_size[1] as i32;
            let multiplier = (255 / ((kernel * 2 + 1).pow(2) - 1)) as u16;
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    if !is_sea(cont_data[i]) {
                        let mut value = 0;
                        for v in -kernel..=kernel {
                            for u in -kernel..=kernel {
                                if ((y + v) >= height) || ((x + u) >= width) || ((y + v) < 0) || ((x + u) < 0)
                                {
                                    continue;
                                }
                                let j = ((y + v) * width + (x + u)) as usize;
                                if is_sea(cont_data[j]) {
                                    value += 1;
                                }
                            }
                        }
                        filter_data[i] = (value * multiplier * 2).min(255) as u8;
                    };
                }
            }
        }
        WorldModel::Globe(_) => todo!(),
    }

    // Set new layer data.
    logics.put_layer(ViewedMapLayer::TopographyFilter, filter_data);

    vec![ViewedMapLayer::TopographyFilter]
}

/// Generate an influence map for a layer with a given influence shape.
fn generate_influence(
    logics: &mut MapLogicData,
    shape: &InfluenceShape,
    model: &WorldModel,
    layer: ViewedMapLayer,
) -> Vec<ViewedMapLayer> {
    let map_data = logics.get_layer_mut(layer);
    fill_influence(map_data, shape, model);
    vec![layer]
}

/// Is this continent tile marked as water?
fn is_sea(value: u8) -> bool {
    value <= 127
}
