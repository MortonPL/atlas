use atlas_lib::{
    bevy::{ecs as bevy_ecs, prelude::*},
    bevy_egui,
    serde_derive::{Deserialize, Serialize},
    ui::sidebar::*,
    MakeUi,
};

use crate::config::{CELSIUS_MAX, CELSIUS_MIN, PRECIP_MAX, PRECIP_MIN};

/// Specialised multi-segment lerp operating on latitude coordinates.
/// HACK: Different type for temperature and precipitation, because clamp limits are different.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct LatitudinalTemperatureLerp {
    #[name("Value At North Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_pole_value: f32,
    #[name("Value At 69 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_arctic_value: f32,
    #[name("Value At 46 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_temperate_value: f32,
    #[name("Value At 23 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub north_tropic_value: f32,
    #[name("Value At Equator")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub equator_value: f32,
    #[name("Value At 23 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_tropic_value: f32,
    #[name("Value At 46 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_temperate_value: f32,
    #[name("Value At 69 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_arctic_value: f32,
    #[name("Value At South Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(CELSIUS_MIN..=CELSIUS_MAX))]
    pub south_pole_value: f32,
    #[name("Non-Linear Tropics Bias")]
    #[control(SidebarCheckbox)]
    pub non_linear_tropics: bool,
}

/// Specialised multi-segment lerp operating on latitude coordinates.
/// HACK: Different type for temperature and precipitation, because clamp limits are different.
#[derive(Debug, Deserialize, Resource, Serialize, MakeUi)]
#[serde(crate = "atlas_lib::serde")]
pub struct LatitudinalPrecipitationLerp {
    #[name("Value At North Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_pole_value: f32,
    #[name("Value At 69 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_arctic_value: f32,
    #[name("Value At 46 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_temperate_value: f32,
    #[name("Value At 23 Degrees North")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub north_tropic_value: f32,
    #[name("Value At Equator")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub equator_value: f32,
    #[name("Value At 23 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_tropic_value: f32,
    #[name("Value At 46 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_temperate_value: f32,
    #[name("Value At 69 Degrees South")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_arctic_value: f32,
    #[name("Value At South Pole")]
    #[control(SidebarSlider)]
    #[add(clamp_range(PRECIP_MIN..=PRECIP_MAX))]
    pub south_pole_value: f32,
    #[name("Non-Linear Tropics Bias")]
    #[control(SidebarCheckbox)]
    pub non_linear_tropics: bool,
}
