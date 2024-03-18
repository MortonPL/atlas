use std::f32::consts::FRAC_PI_2;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat}, utils::HashMap,
};

use atlas_lib::UiConfigurableEnum;

use crate::{config::{GeneratorConfig, WorldModel}, map::ViewedMapLayer};

#[derive(Default, Resource)]
pub struct MapGraphicsData {
    pub current: ViewedMapLayer,
    pub previous: ViewedMapLayer,
    pub layers: HashMap<ViewedMapLayer, MapGraphicsLayer>,
}

#[derive(Default)]
pub struct MapGraphicsLayer {
    pub material: Handle<StandardMaterial>,
    pub image: Handle<Image>,
    pub invalidated: bool,
    pub outdated: bool,
}

impl MapGraphicsLayer {
    pub fn new(material: Handle<StandardMaterial>, image: Handle<Image>) -> Self {
        Self {
            material,
            image,
            invalidated: false,
            outdated: false,
        }
    }
}

// World map model tag.
#[derive(Component)]
pub struct WorldMapMesh;

// World globe model tag.
#[derive(Component)]
pub struct WorldGlobeMesh;
