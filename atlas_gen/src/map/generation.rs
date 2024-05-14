use bevy::utils::petgraph::matrix_graph::Zero;
use bevy_egui::egui::{ecolor::rgb_from_hsv, lerp};

use crate::{
    config::{InfluenceShape, NoiseAlgorithm, SessionConfig, TopographyDisplayMode, WorldModel},
    map::{
        internal::{climate_to_hsv, MapLogicData},
        samplers::{
            add_with_algorithm, apply_influence, apply_influence_from_src, fill_influence,
            fill_with_algorithm,
        },
        MapDataLayer,
    },
};

/// Choose relevant generation procedure based on layer.
pub fn generate(layer: MapDataLayer, logics: &mut MapLogicData, config: &SessionConfig) -> Vec<MapDataLayer> {
    let model = &config.general.world_model;
    match layer {
        MapDataLayer::Preview => generate_preview(logics, config),
        MapDataLayer::Continents => generate_continents(logics, config, layer),
        MapDataLayer::Topography => generate_generic(logics, &config.topography, model, layer),
        MapDataLayer::Temperature => generate_temperature(logics, config, layer),
        MapDataLayer::Humidity => generate_humidity(logics, config, layer),
        MapDataLayer::Climate => generate_climate(logics, config, layer),
        MapDataLayer::Resource => todo!(),  // TODO
        MapDataLayer::Fertility => todo!(), // TODO
        MapDataLayer::Richness => todo!(),  // TODO
        // Influence
        MapDataLayer::ContinentsInfluence => generate_influence(logics, &config.continents, model, layer),
        MapDataLayer::TopographyInfluence => generate_influence(logics, &config.topography, model, layer),
        MapDataLayer::TemperatureInfluence => generate_influence(logics, &config.temperature, model, layer),
        MapDataLayer::HumidityInfluence => generate_influence(logics, &config.humidity, model, layer),
        // Unreachable
        MapDataLayer::RealTopography => unreachable!(),
        MapDataLayer::TopographyFilter => unreachable!(),
    }
}

/// Refresh other layers (if needed) after modifying this layer.
pub fn after_generate(
    layer: MapDataLayer,
    logics: &mut MapLogicData,
    config: &SessionConfig,
) -> Vec<MapDataLayer> {
    let mut regen_layers = match layer {
        MapDataLayer::Continents => {
            generate_utility_topo_filter(logics, config);
            generate_utility_real_topo(logics);
            vec![MapDataLayer::RealTopography, MapDataLayer::TopographyFilter]
        }
        MapDataLayer::Topography => {
            generate_utility_topo_filter(logics, config);
            generate_utility_real_topo(logics);
            vec![MapDataLayer::RealTopography, MapDataLayer::TopographyFilter]
        }
        MapDataLayer::Temperature => {
            generate_climate(logics, config, MapDataLayer::Climate);
            vec![MapDataLayer::Climate]
        }
        MapDataLayer::Humidity => {
            generate_climate(logics, config, MapDataLayer::Climate);
            vec![MapDataLayer::Climate]
        }
        _ => vec![],
    };
    if !matches!(layer, MapDataLayer::Preview) {
        generate_preview(logics, config);
        regen_layers.push(MapDataLayer::Preview);
    }
    regen_layers
}

/// Generate pretty map preview.
fn generate_preview(logics: &mut MapLogicData, config: &SessionConfig) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut preview_data = logics.pop_layer(MapDataLayer::Preview);
    let real_data = logics.get_layer(MapDataLayer::RealTopography);
    let cont_data = logics.get_layer(MapDataLayer::Continents);
    let climate_data = logics.get_layer(MapDataLayer::Climate);

    let height_levels = config.general.height_levels as f32;
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
            let v = v
                * (((1.0 - height.clamp(0.0, 1.0)) * height_levels).ceil() / height_levels).clamp(0.15, 0.85);
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
    logics.put_layer(MapDataLayer::Preview, preview_data);

    vec![MapDataLayer::Preview]
}

/// Generate continental data.
fn generate_continents(
    logics: &mut MapLogicData,
    config: &SessionConfig,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut cont_data = logics.pop_layer(layer);
    // Get relevant config info.
    let model = &config.general.world_model;
    // Run the noise algorithm to obtain height data for continental discrimination.
    fill_with_algorithm(&mut cont_data, model, &config.continents);
    // Apply the influence map if requested.
    if let Some(inf_layer) = layer.get_influence_layer() {
        handle_influence(&mut cont_data, logics, inf_layer, &config.continents);
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
    logics.put_layer(layer, cont_data);

    vec![layer]
}

/// A generic generation routine.
fn generate_generic(
    logics: &mut MapLogicData,
    config: impl AsRef<NoiseAlgorithm> + AsRef<InfluenceShape>,
    model: &WorldModel,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut data = logics.pop_layer(layer);
    // Run the noise algorithm for map topography (height data).
    fill_with_algorithm(&mut data, model, &config);
    // Apply the influence map if requested.
    if let Some(inf_layer) = layer.get_influence_layer() {
        handle_influence(&mut data, logics, inf_layer, &config);
    }
    // Set new layer data.
    logics.put_layer(layer, data);
    // This layer should be refreshed.
    vec![layer]
}

fn generate_temperature(
    logics: &mut MapLogicData,
    config: &SessionConfig,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut temp_data = logics.pop_layer(layer);
    let topo_data = logics.get_layer(MapDataLayer::RealTopography);
    // Get relevant config info.
    let model = &config.general.world_model;
    // Set temperature based on latitude.
    match &config.general.world_model {
        WorldModel::Flat(x) => {
            let width = x.world_size[0];
            let height = x.world_size[1];
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let y2 = y as f32 / height as f32;
                    let value = if y2 < 0.5 {
                        let range =
                            config.temperature.north_value as f32..=config.temperature.equator_value as f32;
                        lerp(range, y2 * 2.0)
                    } else {
                        let range =
                            config.temperature.equator_value as f32..=config.temperature.south_value as f32;
                        lerp(range, (y2 - 0.5) * 2.0)
                    };
                    temp_data[i] = value as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
    // Append the noise algorithm data.
    add_with_algorithm(
        &mut temp_data,
        model,
        &config.temperature,
        config.temperature.algorithm_strength,
    );
    // Apply height penalty.
    if !config.temperature.drop_per_height.is_zero() {
        for i in 0..temp_data.len() {
            let drop = (topo_data[i] as f32 * config.temperature.drop_per_height).min(255f32) as u8;
            temp_data[i] = temp_data[i].saturating_sub(drop);
        }
    }
    // Apply the influence map if requested.
    if let Some(inf_layer) = layer.get_influence_layer() {
        handle_influence(&mut temp_data, logics, inf_layer, &config.temperature);
    }
    // Set new layer data.
    logics.put_layer(layer, temp_data);

    vec![layer]
}

fn generate_humidity(
    logics: &mut MapLogicData,
    config: &SessionConfig,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut humi_data = logics.pop_layer(layer);
    let topo_data = logics.get_layer(MapDataLayer::RealTopography);
    // Get relevant config info.
    let model = &config.general.world_model;
    // Set temperature based on latitude.
    match &config.general.world_model {
        WorldModel::Flat(x) => {
            let width = x.world_size[0];
            let height = x.world_size[1];
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let y2 = y as f32 / height as f32;
                    let value = if y2 < 0.246 {
                        // north-temperate
                        let range =
                            config.humidity.north_value as f32..=config.humidity.north_temperate_value as f32;
                        lerp(range, y2 / 0.246)
                    } else if y2 < 0.373 {
                        // temperate-tropic
                        let range = config.humidity.north_temperate_value as f32
                            ..=config.humidity.north_tropic_value as f32;
                        lerp(range, (y2 - 0.246) / (0.373 - 0.246))
                    } else if y2 < 0.5 {
                        // tropic-equator
                        let range =
                            config.humidity.north_tropic_value as f32..=config.humidity.equator_value as f32;
                        lerp(range, (y2 - 0.373) / (0.5 - 0.373))
                    } else if y2 < 0.627 {
                        // equator-tropic
                        let range =
                            config.humidity.equator_value as f32..=config.humidity.south_tropic_value as f32;
                        lerp(range, (y2 - 0.5) / (0.627 - 0.5))
                    } else if y2 < 0.754 {
                        // tropic-temperate
                        let range = config.humidity.south_tropic_value as f32
                            ..=config.humidity.south_temperate_value as f32;
                        lerp(range, (y2 - 0.627) / (0.754 - 0.627))
                    } else {
                        // temperate-south
                        let range =
                            config.humidity.south_temperate_value as f32..=config.humidity.south_value as f32;
                        lerp(range, (y2 - 0.754) / (1.0 - 0.754))
                    };
                    humi_data[i] = value as u8;
                }
            }
        }
        WorldModel::Globe(_) => todo!(), // TODO
    }
    // Append the noise algorithm data.
    add_with_algorithm(
        &mut humi_data,
        model,
        &config.humidity,
        config.humidity.algorithm_strength,
    );
    // Apply height penalty.
    if !config.humidity.drop_per_height.is_zero() {
        for i in 0..humi_data.len() {
            let drop = (topo_data[i] as f32 * config.humidity.drop_per_height).min(255f32) as u8;
            humi_data[i] = humi_data[i].saturating_sub(drop);
        }
    }
    // Apply the influence map if requested.
    if let Some(inf_layer) = layer.get_influence_layer() {
        handle_influence(&mut humi_data, logics, inf_layer, &config.humidity);
    }
    // Set new layer data.
    logics.put_layer(layer, humi_data);

    vec![layer]
}

fn generate_climate(
    logics: &mut MapLogicData,
    config: &SessionConfig,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    /*
    // Move out layer data.
    let mut clim_data = logics.pop_layer(layer);
    // Get relevant config info.
    let model = &config.general.world_model;
    // Set new layer data.
    logics.put_layer(layer, clim_data);
    */
    vec![layer]
}

/// Generate FINAL topography data.
fn generate_utility_real_topo(logics: &mut MapLogicData) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut real_data = logics.pop_layer(MapDataLayer::RealTopography);
    let topo_data = logics.get_layer(MapDataLayer::Topography);
    let filter_data = logics.get_layer(MapDataLayer::TopographyFilter);
    // Little trick: topography filter is basically an influence layer.
    apply_influence_from_src(&mut real_data, topo_data, filter_data, Default::default(), 1.0);
    // Set new layer data.
    logics.put_layer(MapDataLayer::RealTopography, real_data);

    vec![MapDataLayer::RealTopography]
}

/// Generate beach smoothing topography filter.
fn generate_utility_topo_filter(logics: &mut MapLogicData, config: &SessionConfig) -> Vec<MapDataLayer> {
    let mut filter_data = logics.pop_layer(MapDataLayer::TopographyFilter);
    filter_data.fill(0);

    let kernel: i32 = config.topography.coastal_erosion as i32;
    if kernel == 0 {
        return vec![];
    }

    let cont_data = logics.get_layer(MapDataLayer::Continents);

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
                        filter_data[i] = 255 - (value * multiplier * 2).min(255) as u8;
                    };
                }
            }
        }
        WorldModel::Globe(_) => todo!(),
    }

    // Set new layer data.
    logics.put_layer(MapDataLayer::TopographyFilter, filter_data);

    vec![MapDataLayer::TopographyFilter]
}

/// Generate an influence map for a layer with a given influence shape.
fn generate_influence(
    logics: &mut MapLogicData,
    shape: impl AsRef<InfluenceShape>,
    model: &WorldModel,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    let map_data = logics.get_layer_mut(layer);
    fill_influence(map_data, shape.as_ref(), model);
    vec![layer]
}

/// Check if influence map should be applied and apply it.
fn handle_influence(
    data: &mut [u8],
    logics: &mut MapLogicData,
    layer: MapDataLayer,
    config: impl AsRef<InfluenceShape>,
) {
    let (use_influence, influence_strength, influence_mode) = match config.as_ref() {
        InfluenceShape::None(_) => (false, 0.0, Default::default()),
        InfluenceShape::Circle(x) => (true, x.influence_strength, x.influence_mode),
        InfluenceShape::Strip(x) => (true, x.influence_strength, x.influence_mode),
        InfluenceShape::Fbm(x) => (true, x.influence_strength, x.influence_mode),
        InfluenceShape::FromImage(x) => (true, x.influence_strength, x.influence_mode),
    };
    if use_influence {
        let map_data = logics.get_layer(layer);
        apply_influence(data, map_data, influence_mode, influence_strength);
    }
}

/// Is this continent tile marked as water?
fn is_sea(value: u8) -> bool {
    value <= 127
}
