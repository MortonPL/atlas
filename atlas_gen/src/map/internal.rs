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

/// Convert graphical representation of a map layer to a logical representation of the map layer.
/// The underlying conversion may differ based on layer variant.
///
/// This function is the inverse of [`magic_convert_data_to_png`].
pub fn magic_convert_png_to_data(data: &mut [u8], _layer: ViewedMapLayer) -> Vec<u8> {
    // TODO
    data.to_vec()
}

/// Convert logical representation of a map layer to a graphical representation of the map layer.
/// The underlying conversion may differ based on layer variant.
///
/// This function is the inverse of [`magic_convert_png_to_data`].
pub fn magic_convert_data_to_png(data_layers: &MapLogicData, layer: ViewedMapLayer) -> Vec<u8> {
    // TODO
    let data = data_layers
        .layers
        .get(&layer)
        .expect("MapLogicData should map all layers");
    data.to_vec()
}
