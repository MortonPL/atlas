use atlas_lib::{
    bevy::utils::petgraph::matrix_graph::Zero,
    bevy_prng::WyRand,
    bevy_rand::resource::GlobalEntropy,
    config::{
        climate::{precip_clamp, precip_to_byte, ALTITUDE_STEP},
        deposit::DepositChunk,
        gen::{AtlasGenConfig, ColorDisplayMode, InfluenceMode, InfluenceShape, NoiseAlgorithm},
        AtlasConfig, WorldModel,
    },
    domain::{
        graphics::{MapLogicData, CLIMATEMAP_SIZE},
        map::{is_sea, MapDataLayer},
    },
    rand::Rng,
};

use crate::map::samplers::{
    add_with_algorithm, apply_influence, apply_influence_from_src, fill_influence, fill_latitudinal_precip,
    fill_latitudinal_temp, fill_with_algorithm,
};

/// Choose relevant generation procedure based on layer.
pub fn generate(
    layer: MapDataLayer,
    logics: &mut MapLogicData,
    config: &mut AtlasGenConfig,
    rng: &mut GlobalEntropy<WyRand>,
) -> Vec<MapDataLayer> {
    let model = config.general.generation_model;
    let world_size = config.general.world_size;
    match layer {
        MapDataLayer::Preview => generate_preview(logics, config),
        MapDataLayer::Continents => generate_continents(logics, config, layer),
        MapDataLayer::Topography => generate_generic(logics, &config.topography, model, world_size, layer),
        MapDataLayer::Temperature => generate_temperature(logics, config, layer),
        MapDataLayer::Precipitation => generate_precipitation(logics, config, layer),
        MapDataLayer::Climate => generate_climate(logics, config, layer),
        MapDataLayer::Deposits => generate_resources(logics, config, layer, rng),
        // Influence
        MapDataLayer::ContinentsInfluence => {
            generate_influence(logics, &config.continents, model, world_size, layer)
        }
        MapDataLayer::TopographyInfluence => {
            generate_influence(logics, &config.topography, model, world_size, layer)
        }
        MapDataLayer::TemperatureInfluence => {
            generate_influence(logics, &config.temperature, model, world_size, layer)
        }
        MapDataLayer::PrecipitationInfluence => {
            generate_influence(logics, &config.precipitation, model, world_size, layer)
        }
        // Unreachable
        MapDataLayer::RealTopography => unreachable!(),
        MapDataLayer::TopographyFilter => unreachable!(),
    }
}

/// Refresh other layers (if needed) after modifying this layer.
pub fn after_generate(
    layer: MapDataLayer,
    logics: &mut MapLogicData,
    config: &mut AtlasGenConfig,
    rng: &mut GlobalEntropy<WyRand>,
) -> Vec<MapDataLayer> {
    let mut regen_layers = match layer {
        MapDataLayer::Continents => {
            generate_utility_topo_filter(logics, config);
            generate_utility_real_topo(logics);
            generate_temperature(logics, config, MapDataLayer::Temperature);
            generate_precipitation(logics, config, MapDataLayer::Precipitation);
            generate_climate(logics, config, MapDataLayer::Climate);
            generate_resources(logics, config, MapDataLayer::Deposits, rng);
            vec![
                MapDataLayer::TopographyFilter,
                MapDataLayer::RealTopography,
                MapDataLayer::Temperature,
                MapDataLayer::Precipitation,
                MapDataLayer::Climate,
            ]
        }
        MapDataLayer::Topography => {
            generate_utility_topo_filter(logics, config);
            generate_utility_real_topo(logics);
            generate_temperature(logics, config, MapDataLayer::Temperature);
            generate_precipitation(logics, config, MapDataLayer::Precipitation);
            generate_climate(logics, config, MapDataLayer::Climate);
            generate_resources(logics, config, MapDataLayer::Deposits, rng);
            vec![
                MapDataLayer::TopographyFilter,
                MapDataLayer::RealTopography,
                MapDataLayer::Temperature,
                MapDataLayer::Precipitation,
                MapDataLayer::Climate,
            ]
        }
        MapDataLayer::Temperature => {
            generate_climate(logics, config, MapDataLayer::Climate);
            generate_resources(logics, config, MapDataLayer::Deposits, rng);
            vec![MapDataLayer::Climate]
        }
        MapDataLayer::Precipitation => {
            generate_climate(logics, config, MapDataLayer::Climate);
            generate_resources(logics, config, MapDataLayer::Deposits, rng);
            vec![MapDataLayer::Climate]
        }
        MapDataLayer::Climate => {
            generate_resources(logics, config, MapDataLayer::Deposits, rng);
            vec![]
        }
        _ => vec![],
    };
    match layer {
        MapDataLayer::Preview => { /* Do nothing */ }
        _ => {
            generate_preview(logics, config);
            regen_layers.push(MapDataLayer::Preview);
        }
    }
    regen_layers
}

/// Generate pretty map preview.
fn generate_preview(logics: &mut MapLogicData, config: &AtlasGenConfig) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut preview_data = logics.pop_layer(MapDataLayer::Preview);
    let real_data = logics.get_layer(MapDataLayer::RealTopography);
    let cont_data = logics.get_layer(MapDataLayer::Continents);
    let climate_data = logics.get_layer(MapDataLayer::Climate);
    // Get relevant config settings.
    let climate_display = config.general.color_display;
    let height_levels = config.general.height_levels as f32;
    let highest = (config.general.altitude_limit / ALTITUDE_STEP).floor();
    // Paint preview.
    for i in 0..real_data.len() {
        // Fetch preview color.
        let rgb = match climate_display {
            ColorDisplayMode::Topography => {
                if is_sea(cont_data[i]) {
                    [0, 160, 255]
                } else if real_data[i] < 5 {
                    [70, 180, 75]
                } else if real_data[i] < 13 {
                    [110, 190, 70]
                } else if real_data[i] < 25 {
                    [240, 230, 60]
                } else if real_data[i] < 38 {
                    [190, 130, 80]
                } else if real_data[i] < 50 {
                    [180, 85, 40]
                } else {
                    [140, 140, 140]
                }
            }
            ColorDisplayMode::SimplifiedClimate => {
                let biome = config.get_biome(climate_data[i]);
                biome.simple_color
            }
            ColorDisplayMode::DetailedClimate => {
                let biome = config.get_biome(climate_data[i]);
                biome.color
            }
        }
        .map(|x| x as f32 / 255.0);
        let mut v = 1.0;
        // Shift color value according to height.
        if !highest.is_zero() {
            let height = real_data[i] as f32 / highest;
            v = (((1.0 - height.clamp(0.0, 1.0)) * height_levels).ceil() / height_levels).clamp(0.2, 1.0);
        }
        // Set final color.
        let (r, g, b) = (
            (rgb[0] * v * 255.0) as u8,
            (rgb[1] * v * 255.0) as u8,
            (rgb[2] * v * 255.0) as u8,
        );
        let j = i * 4;
        preview_data[j] = r;
        preview_data[j + 1] = g;
        preview_data[j + 2] = b;
        preview_data[j + 3] = 255;
    }
    // Set new layer data.
    logics.put_layer(MapDataLayer::Preview, preview_data);
    // This layer should be refreshed.
    vec![MapDataLayer::Preview]
}

/// Generate continental data.
fn generate_continents(
    logics: &mut MapLogicData,
    config: &AtlasGenConfig,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut cont_data = logics.pop_layer(layer);
    // Get relevant config info.
    let model = config.general.generation_model;
    let world_size = config.general.world_size;
    // Run the noise algorithm to obtain height data for continental discrimination.
    fill_with_algorithm(&mut cont_data, model, world_size, &config.continents);
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
    // This layer should be refreshed.
    vec![layer]
}

/// A generic generation routine.
fn generate_generic(
    logics: &mut MapLogicData,
    config: impl AsRef<NoiseAlgorithm> + AsRef<InfluenceShape>,
    model: WorldModel,
    world_size: [u32; 2],
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut data = logics.pop_layer(layer);
    // Run the noise algorithm for map topography (height data).
    fill_with_algorithm(&mut data, model, world_size, &config);
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
    config: &AtlasGenConfig,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut temp_data = logics.pop_layer(layer);
    let topo_data = logics.get_layer(MapDataLayer::RealTopography);
    // Get relevant config info.
    let model = config.general.generation_model;
    let world_size = config.general.world_size;
    // Set temperature based on latitude.
    fill_latitudinal_temp(&mut temp_data, model, world_size, &config.temperature.latitudinal);
    // Append the noise algorithm data.
    add_with_algorithm(
        &mut temp_data,
        model,
        world_size,
        &config.temperature,
        config.temperature.algorithm_strength,
    );
    // Apply height penalty.
    let real_lapse_rate = config.temperature.lapse_rate * 2.0 / 25.0;
    if !real_lapse_rate.is_zero() {
        for i in 0..temp_data.len() {
            let drop = (topo_data[i] as f32 * real_lapse_rate).min(255f32) as u8;
            temp_data[i] = temp_data[i].saturating_sub(drop);
        }
    }
    // Apply the influence map if requested.
    if let Some(inf_layer) = layer.get_influence_layer() {
        handle_influence(&mut temp_data, logics, inf_layer, &config.temperature);
    }
    // Set new layer data.
    logics.put_layer(layer, temp_data);
    // This layer should be refreshed.
    vec![layer]
}

fn generate_precipitation(
    logics: &mut MapLogicData,
    config: &AtlasGenConfig,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut humi_data = logics.pop_layer(layer);
    let topo_data = logics.get_layer(MapDataLayer::RealTopography);
    // Get relevant config info.
    let model = config.general.generation_model;
    let world_size = config.general.world_size;
    // Set precipitation based on latitude.
    fill_latitudinal_precip(
        &mut humi_data,
        model,
        world_size,
        &config.precipitation.latitudinal,
    );
    // Append the noise algorithm data.
    add_with_algorithm(
        &mut humi_data,
        model,
        world_size,
        &config.precipitation,
        config.precipitation.algorithm_strength,
    );
    // Apply height penalty.
    if !config.precipitation.drop_per_height.is_zero() {
        for i in 0..humi_data.len() {
            let altitude = topo_data[i] as f32 * 40.0;
            if altitude > config.precipitation.amp_point {
                let drop = (altitude - config.precipitation.amp_point) * config.precipitation.drop_per_height;
                let drop = precip_to_byte(precip_clamp(drop));
                humi_data[i] = humi_data[i].saturating_sub(drop);
            }
        }
    }
    // Apply the influence map if requested.
    if let Some(inf_layer) = layer.get_influence_layer() {
        handle_influence(&mut humi_data, logics, inf_layer, &config.precipitation);
    }
    // Set new layer data.
    logics.put_layer(layer, humi_data);

    vec![layer]
}

fn generate_climate(
    logics: &mut MapLogicData,
    config: &AtlasGenConfig,
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut clim_data = logics.pop_layer(layer);
    let cont_data = logics.get_layer(MapDataLayer::Continents);
    let temp_data = logics.get_layer(MapDataLayer::Temperature);
    let prec_data = logics.get_layer(MapDataLayer::Precipitation);
    let len = config.climate.biomes.len() as u8;
    // Use climate map.
    let climatemap = logics.get_climatemap();
    for i in 0..clim_data.len() {
        let climate = if is_sea(cont_data[i]) {
            0
        } else {
            let map_index = prec_data[i] as usize * CLIMATEMAP_SIZE + temp_data[i] as usize;
            climatemap[map_index]
        };
        clim_data[i] = if climate < len { climate } else { 0 };
    }
    // Set new layer data.
    logics.put_layer(layer, clim_data);
    // This layer should be refreshed.
    vec![layer]
}

fn generate_resources(
    logics: &mut MapLogicData,
    config: &mut AtlasGenConfig,
    _layer: MapDataLayer,
    rng: &mut GlobalEntropy<WyRand>,
) -> Vec<MapDataLayer> {
    // Get layer data.
    let cont_data = logics.get_layer(MapDataLayer::Continents);
    let clim_data = logics.get_layer(MapDataLayer::Climate);
    // Generate region chunks.
    let (width, height) = config.get_world_size();
    let cwidth = width.div_ceil(config.deposits.chunk_size as u32) as usize;
    let cheight = height.div_ceil(config.deposits.chunk_size as u32) as usize;
    let (width, height) = (width as usize, height as usize);
    let chunk_count = cwidth * cheight;
    let size = config.deposits.chunk_size as usize;
    let mut chunks = vec![DepositChunk::default(); chunk_count];
    for i in 0..(width * height) {
        // Find the chunk for this tile.
        let j = ((i / width) / size) * cwidth + (i % width) / size;
        let chunk = &mut chunks[j];
        if !is_sea(cont_data[i]) {
            chunk.tile_count += 1;
        }
        // Assign flora and fauna resources based on climate.
        let biome = config.get_biome(clim_data[i]);
        for biome_deposit in &biome.deposits {
            let deposit = config.deposits.types.get(biome_deposit.id as usize);
            if deposit.is_none()
                || biome_deposit.chance.is_zero()
                || !rng.gen_bool(biome_deposit.chance as f64)
            {
                continue;
            }
            let deposit = deposit.unwrap();
            if let Some(v) = chunk.deposits.get_mut(&biome_deposit.id) {
                *v += (deposit.gen_average + rng.gen_range(-deposit.gen_deviation..=deposit.gen_deviation))
                    .max(0.0);
            } else {
                let v = (deposit.gen_average + rng.gen_range(-deposit.gen_deviation..=deposit.gen_deviation))
                    .max(0.0);
                chunk.deposits.insert(biome_deposit.id, v);
            }
        }
        // Assign other natural resources randomly.
        for (id, deposit) in config.deposits.types.iter().enumerate() {
            if deposit.gen_chance.is_zero()
                || !rng.gen_bool(deposit.gen_chance as f64)
                || is_sea(cont_data[i])
            {
                continue;
            }
            if let Some(v) = chunk.deposits.get_mut(&(id as u32)) {
                *v += (deposit.gen_average + rng.gen_range(-deposit.gen_deviation..=deposit.gen_deviation))
                    .max(0.0);
            } else {
                let v = (deposit.gen_average + rng.gen_range(-deposit.gen_deviation..=deposit.gen_deviation))
                    .max(0.0);
                chunk.deposits.insert(id as u32, v);
            }
        }
    }
    config.deposits.chunks = chunks;
    // Don't refresh any layer.
    vec![]
}

/// Generate FINAL topography data.
fn generate_utility_real_topo(logics: &mut MapLogicData) -> Vec<MapDataLayer> {
    // Move out layer data.
    let mut real_data = logics.pop_layer(MapDataLayer::RealTopography);
    let topo_data = logics.get_layer(MapDataLayer::Topography);
    let filter_data = logics.get_layer(MapDataLayer::TopographyFilter);
    // Little trick: topography filter is basically an influence layer.
    apply_influence_from_src(
        &mut real_data,
        topo_data,
        filter_data,
        InfluenceMode::ScaleDown,
        1.0,
    );
    // Set new layer data.
    logics.put_layer(MapDataLayer::RealTopography, real_data);
    // This layer should be refreshed.
    vec![MapDataLayer::RealTopography]
}

/// Generate beach smoothing topography filter.
fn generate_utility_topo_filter(logics: &mut MapLogicData, config: &AtlasGenConfig) -> Vec<MapDataLayer> {
    let mut filter_data = logics.pop_layer(MapDataLayer::TopographyFilter);
    filter_data.fill(0);
    // Fetch kernel size and world dimensions.
    let kernel: i32 = config.topography.coastal_erosion as i32;
    let (width, height) = (
        config.general.world_size[0] as i32,
        config.general.world_size[1] as i32,
    );
    let cont_data = logics.get_layer(MapDataLayer::Continents);
    // If this feature is disabled, fill out the layer with 1s.
    if kernel == 0 {
        for y in 0..height {
            for x in 0..width {
                let i = (y * width + x) as usize;
                if !is_sea(cont_data[i]) {
                    filter_data[i] = 255;
                }
            }
        }
        logics.put_layer(MapDataLayer::TopographyFilter, filter_data);
        return vec![];
    }
    // Simple NxN blur.
    match &config.general.generation_model {
        WorldModel::Flat => {
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
        WorldModel::Globe => todo!(),
    }
    // Set new layer data.
    logics.put_layer(MapDataLayer::TopographyFilter, filter_data);
    // This layer should be refreshed.
    vec![MapDataLayer::TopographyFilter]
}

/// Generate an influence map for a layer with a given influence shape.
fn generate_influence(
    logics: &mut MapLogicData,
    shape: impl AsRef<InfluenceShape>,
    model: WorldModel,
    world_size: [u32; 2],
    layer: MapDataLayer,
) -> Vec<MapDataLayer> {
    let map_data = logics.get_layer_mut(layer);
    fill_influence(map_data, shape.as_ref(), model, world_size);
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
        InfluenceShape::None => (false, 0.0, Default::default()),
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
