use atlas_lib::domain::{graphics::MapLogicData, map::MapDataLayer};

use crate::config::{AtlasSimConfig, BiomeConfig, ClimatePreviewMode};

pub const CONFIG_NAME: &str = "atlassim.toml";

/// Convert logical layer data to a texture.
/// For most cases, this just expands greyscale to grey RGBA.
pub fn data_to_view(data_layers: &MapLogicData, layer: MapDataLayer, config: &AtlasSimConfig) -> Vec<u8> {
    match layer {
        MapDataLayer::Climate => {
            let data = data_layers.get_layer(layer);
            climate_to_view(data, config)
        }
        _ => atlas_lib::domain::graphics::data_to_view(data_layers, layer),
    }
}

pub fn fetch_climate(i: usize, config: &AtlasSimConfig) -> &BiomeConfig {
    if i > config.climate.biomes.len() {
        &config.climate.default_biome
    } else {
        &config.climate.biomes[i]
    }
}

fn climate_to_view(data: &[u8], config: &AtlasSimConfig) -> Vec<u8> {
    match config.climate.preview_mode {
        ClimatePreviewMode::SimplifiedColor => {
            let fun = |x: &u8| {
                let climate = fetch_climate(*x as usize, config);
                [
                    climate.simple_color[0],
                    climate.simple_color[1],
                    climate.simple_color[2],
                    255,
                ]
            };
            data.iter().flat_map(fun).collect()
        }
        ClimatePreviewMode::DetailedColor => {
            let fun = |x: &u8| {
                let climate = fetch_climate(*x as usize, config);
                [climate.color[0], climate.color[1], climate.color[2], 255]
            };
            data.iter().flat_map(fun).collect()
        }
    }
}
