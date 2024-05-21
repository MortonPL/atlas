use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    serde_derive::{Deserialize, Serialize},
};

/// Complete configuration for the history simulator.
#[derive(Debug, Default, Deserialize, Resource, Serialize)]
#[serde(crate = "atlas_lib::serde")]
pub struct AtlasSimConfig {}
