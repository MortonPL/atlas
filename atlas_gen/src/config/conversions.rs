// Temperature Unit (Celsius).
// Byte representation is +1 u = +0.5 C, 100 u = 0 C.

pub const CELSIUS_MIN: f32 = -50.0;
pub const CELSIUS_MAX: f32 = 77.5;

pub fn celsius_clamp(x: f32) -> f32 {
    x.clamp(CELSIUS_MIN, CELSIUS_MAX)
}

pub fn celsius_to_byte(x: f32) -> u8 {
    (x * 2.0 + 100.0) as u8
}

pub fn celsius_from_byte(x: u8) -> f32 {
    (x - 100) as f32 / 2.0
}

pub fn celsius_to_fraction(x: f32) -> f32 {
    (x - CELSIUS_MIN) / 127.5
}

pub fn celsius_from_fraction(x: f32) -> f32 {
    x * (CELSIUS_MAX - CELSIUS_MIN) + CELSIUS_MIN
}

// Precipitation Unit (mm).
// Byte representation is 1 u = 20mm.

pub const PRECIP_MIN: f32 = 0.0;
pub const PRECIP_MAX: f32 = 5100.0;

pub fn precip_clamp(x: f32) -> f32 {
    x.clamp(PRECIP_MIN, PRECIP_MAX)
}

pub fn precip_to_byte(x: f32) -> u8 {
    (x / 20.0) as u8
}

pub fn precip_from_byte(x: u8) -> f32 {
    x as f32 * 20.0
}

pub fn precip_to_fraction(x: f32) -> f32 {
    x / PRECIP_MAX
}

pub fn precip_from_fraction(x: f32) -> f32 {
    x * PRECIP_MAX
}
