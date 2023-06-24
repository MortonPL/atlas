use bevy::prelude::Resource;
use serde_derive::Deserialize;

#[derive(Deserialize, Resource)]
pub struct Genfile {
    pub general: General,
    pub tiles: Vec<TileType>,
    pub transforms: Vec<Transform>,
}

#[derive(Deserialize)]
pub struct General {
    pub name: String,
    pub version: String,
    pub base_version: String,
    pub description: Option<String>
}

#[derive(Deserialize)]
pub struct TileType {
    pub name: String,
    pub color: [f32; 3],
}

#[derive(Deserialize)]
pub struct Transform {
    pub name: String,
    pub mode: OperateMode,
}

#[derive(Deserialize)]
pub enum OperateMode {
    Fill(OperateOnType),
    Noise(NoiseTypes),
    Custom,
}

#[derive(Deserialize)]
pub enum NoiseTypes {
    Pepper(NoisePepperParams),
}

#[derive(Deserialize)]
pub struct NoisePepperParams {
    pub value: OperateOnType,
    pub frequency: f32
}

#[derive(Deserialize)]
pub enum OperateOnType {
    Tile(String),
    Height(i32),
    Index(usize),
}

#[derive(Deserialize)]
pub struct Param<T> {
    pub default: T,
    pub datatype: String,
    pub values: Option<Vec<T>>,
    pub range: Option<Vec<T>>,
}
