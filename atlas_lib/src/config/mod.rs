pub mod climate;
pub mod gen;
mod io;
pub mod deposit;
pub mod sim;

use crate::ui::UiEditableEnum;
use atlas_macro::UiEditableEnum;
use bevy::prelude::*;
use climate::{BiomeConfig, ClimatePreviewMode};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, marker::PhantomData};

pub use io::*;

pub const MAX_WORLD_SIZE: u32 = 1000;

/// Plugin responsible for the generator/simulator configuration and I/O.
#[derive(Default)]
pub struct ConfigPlugin<C: AtlasConfig> {
    __: PhantomData<C>,
}

impl<C: AtlasConfig> Plugin for ConfigPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_resource::<C>();
    }
}

pub trait AtlasConfig: Resource + Default {
    fn get_world_size(&self) -> (u32, u32);
    fn get_preview_model(&self) -> WorldModel;
    fn get_climate_preview(&self) -> ClimatePreviewMode;
    fn get_biome(&self, i: u8) -> &BiomeConfig;

    /// Convert a point from Bevy world space to map space.
    fn world_to_map(&self, point: (f32, f32)) -> Option<(u32, u32)> {
        let (width, height) = self.get_world_size();
        let (width, height) = (width as f32, height as f32);
        let coords = ((point.0 * 100.0 + width / 2.0), (-point.1 * 100.0 + height / 2.0));
        if coords.0 > 0.0 && coords.0 < width && coords.1 > 0.0 && coords.1 < height {
            Some((coords.0 as u32, coords.1 as u32))
        } else {
            None
        }
    }

    /// Convert a point from map space to Bevy world space.
    fn map_to_world(&self, point: (u32, u32)) -> (f32, f32) {
        let (width, height) = self.get_world_size();
        (
            (point.0 as f32 - width as f32 / 2.0) / 100.0,
            (height as f32 / 2.0 - (point.1 as f32)) / 100.0,
        )
    }

    /// Convert a point from map space to Bevy world space.
    fn map_to_world_centered(&self, point: (u32, u32)) -> (f32, f32) {
        let (width, height) = self.get_world_size();
        (
            (point.0 as f32 - width as f32 / 2.0 + 0.5) / 100.0,
            (height as f32 / 2.0 - (point.1 as f32) - 0.5) / 100.0,
        )
    }

    /// Convert a float point from map space to Bevy world space.
    fn centroid_to_world_centered(&self, point: (f32, f32)) -> (f32, f32) {
        let (width, height) = self.get_world_size();
        (
            (point.0 - width as f32 / 2.0) / 100.0,
            (height as f32 / 2.0 - point.1) / 100.0,
        )
    }

    /// Convert a point from map space to linear tile index.
    fn map_to_index(&self, point: (u32, u32)) -> u32 {
        let (width, _) = self.get_world_size();
        point.0 + point.1 * width
    }

    /// Convert a point from linear tile index to map space.
    fn index_to_map(&self, index: u32) -> (u32, u32) {
        let (width, _) = self.get_world_size();
        (index % width, index / width)
    }

    /// Find the chunk that contains this map point.
    fn index_to_chunk(&self, index: u32, chunk_size: u32) -> u32 {
        let (width, _) = self.get_world_size();
        let (x, y) = (index % width, index / width);
        let width_in_chunks = width.div_ceil(chunk_size);
        (y / chunk_size) * width_in_chunks + x / chunk_size
    }

    /// Get 4 border tiles for the specified tile index.
    fn get_border_tiles(&self, index: u32) -> BTreeSet<u32> {
        let (width, height) = self.get_world_size();
        let mut result = BTreeSet::default();
        let modi = index % width;
        let divi = index / width;
        if modi != 0 {
            result.insert(index - 1);
        };
        if modi != width - 1 {
            result.insert(index + 1);
        }
        if divi != 0 {
            result.insert(index - width);
        }
        if divi != height - 1 {
            result.insert(index + width);
        }
        result
    }
}

pub trait IntoSimConfig {
    fn into_sim_config(&self) -> Self;
}

#[derive(Copy, Clone, Default, Debug, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum WorldModel {
    #[default]
    Flat,
    Globe,
}
