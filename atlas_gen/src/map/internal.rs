use bevy::{prelude::*, utils::HashMap};

use crate::map::ViewedMapLayer;

#[derive(Default, Resource)]
pub struct MapLogicData {
    pub layers: HashMap<ViewedMapLayer, Vec<u8>>,
}

#[derive(Default, Resource)]
pub struct MapGraphicsData {
    pub current: ViewedMapLayer,
    pub previous: ViewedMapLayer,
    pub layers: HashMap<ViewedMapLayer, MapGraphicsLayer>,
    pub empty_material: Handle<StandardMaterial>,
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

// Currently visible world model.
#[derive(Component)]
pub struct CurrentWorldModel;
