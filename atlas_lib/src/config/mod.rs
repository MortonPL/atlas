mod io;

pub use io::*;

use atlas_macro::UiEditableEnum;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::UiEditableEnum;

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
