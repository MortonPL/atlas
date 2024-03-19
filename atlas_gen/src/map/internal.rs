use bevy::{prelude::*, utils::HashMap};

use crate::map::ViewedMapLayer;

#[derive(Default, Resource)]
pub struct MapLogicData {
    pub layers: HashMap<ViewedMapLayer, Vec<u8>>,
}

#[derive(Default, Resource)]
pub struct MapGraphicsData {
    pub current: ViewedMapLayer,
    pub layers: HashMap<ViewedMapLayer, MapGraphicsLayer>,
    pub empty_material: Handle<StandardMaterial>,
}

#[derive(Default)]
pub struct MapGraphicsLayer {
    pub material: Handle<StandardMaterial>,
    pub invalid: bool,
}

impl MapGraphicsLayer {
    pub fn new(material: Handle<StandardMaterial>) -> Self {
        Self {
            material,
            invalid: true,
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
