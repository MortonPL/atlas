use bevy::prelude::Resource;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Resource)]
pub struct Genfile {
    pub general: General,
    pub tiles: Vec<TileType>,
    pub transforms: Vec<Transform>,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub name: String,
    pub version: String,
    pub base_version: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TileType {
    pub name: String,
    pub description: Option<String>,
    pub color: [f32; 3],
}

#[derive(Debug, Deserialize)]
pub struct Transform {
    pub name: String,
    pub mode: OperateMode,
}

#[derive(Debug, Deserialize)]
pub enum OperateMode {
    Fill(FillTypes),
    Noise(NoiseTypes),
    Custom,
}

#[derive(Debug, Deserialize)]
pub enum FillTypes {
    Simple(FillSimpleParams),
    Conditional(FillConditionalParams),
    Flood(FillFloodParams),
}

#[derive(Debug, Deserialize)]
pub enum NoiseTypes {
    Pepper(NoisePepperParams),
    Perlin(NoisePerlinParams),
}

#[derive(Debug, Deserialize)]
pub struct NoisePepperParams {
    pub value: OperateOnType,
    pub frequency: f32,
    pub deviation: f32,
    pub on: Option<OperateOnType>,
}

#[derive(Debug, Deserialize)]
pub struct NoisePerlinParams {
    pub value: OperateOnType,
    pub offset: f32,
    pub scale: f32,
}

#[derive(Debug, Deserialize)]
pub struct FillSimpleParams {
    pub value: OperateOnType,
}

#[derive(Debug, Deserialize)]
pub struct FillConditionalParams {
    pub value: OperateOnType,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Deserialize)]
pub struct FillFloodParams {
    pub value: OperateOnType,
    pub origins: usize,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Deserialize)]
pub enum OperateOnType {
    Tile(String),
    Height(f32),
    Index(usize),
}

#[derive(Debug, Deserialize)]
pub struct Param<T> {
    pub default: T,
    pub datatype: String,
    pub values: Option<Vec<T>>,
    pub range: Option<Vec<T>>,
}
