mod io;

pub use io::*;

use atlas_macro::UiEditableEnum;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::UiEditableEnum;

pub const MAX_WORLD_SIZE: u32 = 1000;

/// World model describes the geometric model of the world which
/// impacts the coordinate system, map visualisation and map border
/// behavior.
#[derive(Copy, Clone, Default, Debug, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum WorldModel {
    #[default]
    Flat,
    Globe,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Resource, Serialize, UiEditableEnum)]
#[serde(rename_all = "lowercase")]
pub enum ClimatePreviewMode {
    SimplifiedColor,
    #[default]
    DetailedColor,
}

pub trait AtlasConfig: Resource {
    fn get_world_size(&self) -> (u32, u32);
    fn get_preview_model(&self) -> WorldModel;
    fn get_climate_preview(&self) -> ClimatePreviewMode;
    fn climate_index_to_color(&self, i: u8) -> [u8; 4];
    fn climate_index_to_color_simple(&self, i: u8) -> [u8; 4];

    /// Convert a point from Bevy world space to map space.
    fn world_to_map(&self, point: (f32, f32)) -> (u32, u32) {
        let (width, height) = self.get_world_size();
        (
            (point.0 * 100.0 + width as f32 / 2.0) as u32,
            (-point.1 * 100.0 + height as f32 / 2.0) as u32,
        )
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
            (point.0 - width as f32 / 2.0 + 0.5) / 100.0,
            (height as f32 / 2.0 - point.1 - 0.5) / 100.0,
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
}
