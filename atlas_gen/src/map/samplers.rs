use atlas_lib::{
    bevy::{math::Vec2, utils::petgraph::matrix_graph::Zero},
    bevy_egui::egui::lerp,
    config::WorldModel,
};
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex, Perlin, SuperSimplex};

use crate::config::{
    celsius_to_fraction, precip_to_fraction, FbmConfig, InfluenceCircleConfig, InfluenceMode, InfluenceShape,
    InfluenceStripConfig, LatitudinalPrecipitationLerp, LatitudinalTemperatureLerp, NoiseAlgorithm,
    QuadPointLerp,
};

impl QuadPointLerp {
    /// Clone the struct and precalculate difference values.
    pub fn clone_precalc(&self) -> Self {
        Self {
            start: self.start,
            midpoint: self.midpoint,
            midpoint2: self.midpoint2,
            end: self.end,
            midpoint_position: self.midpoint_position,
            midpoint2_position: self.midpoint2_position,
            diff1: self.midpoint2_position - self.midpoint_position,
            diff2: 1.0 - self.midpoint2_position,
        }
    }

    /// Interpolate a value in [0.0, 1.0] range. NOTE: Self should have precalc'd diff1 and diff2 beforehand!
    pub fn lerp(&self, x: f32) -> f32 {
        if x <= self.midpoint_position {
            lerp(self.start..=self.midpoint, x / self.midpoint_position)
        } else if x <= self.midpoint2_position {
            lerp(
                self.midpoint..=self.midpoint2,
                (x - self.midpoint_position) / self.diff1,
            )
        } else {
            lerp(
                self.midpoint2..=self.end,
                (x - self.midpoint2_position) / self.diff2,
            )
        }
    }
}

/// A Sampler allows to sample (obtain) a value in [0.0, 1.0] range in 2D space.
trait Sampler {
    fn offset_origin(self, offset: Vec2) -> Self;
    fn set_scale(self, scale: f32) -> Self;
    fn sample(&self, p: Vec2) -> f32;
}

/// Sample "closeness" (radius - distance) to the center of a circle.
struct CircleSampler {
    offset: Vec2,
    radius: f32,
    midpoint: QuadPointLerp,
}

impl CircleSampler {
    fn new(config: &InfluenceCircleConfig) -> Self {
        let offset = Vec2::new(config.offset[0] as f32, config.offset[1] as f32);
        Self {
            offset,
            radius: config.radius as f32,
            midpoint: config.midpoint.clone_precalc(),
        }
    }
}

impl Sampler for CircleSampler {
    fn sample(&self, p: Vec2) -> f32 {
        // Calculate the distance from circle center.
        let len = p.distance(self.offset);
        // Transform the distance as a fraction of radius.
        let norm = (len / self.radius).clamp(0.0, 1.0);
        // Interpolate value.
        self.midpoint.lerp(norm)
    }

    fn offset_origin(mut self, offset: Vec2) -> Self {
        self.offset += offset;
        self
    }

    fn set_scale(self, _scale: f32) -> Self {
        self
    }
}

/// Sample "closeness" (radius - distance) to a strip defined by two circles connected by a thick strip.
struct StripSampler {
    offset: Vec2,
    start: Vec2,
    end: Vec2,
    length: f32,
    thickness: f32,
    slope_a: f32,
    midpoint: QuadPointLerp,
}

impl StripSampler {
    fn new(config: &InfluenceStripConfig) -> Self {
        let offset = Vec2::new(config.offset[0] as f32, config.offset[1] as f32);
        let (start, end, slope_a) =
            Self::precalculate_strip(config.length as f32, config.angle as f32, config.flip);
        Self {
            offset,
            start,
            end,
            length: config.length as f32,
            thickness: config.thickness as f32,
            slope_a,
            midpoint: config.midpoint.clone_precalc(),
        }
    }

    /// Calculate required strip data: positions of line ends and slope parameters.
    fn precalculate_strip(l: f32, a: f32, flip: bool) -> (Vec2, Vec2, f32) {
        // Tan(alpha)
        let mut tana = a.tan();
        if flip {
            tana = -tana;
        }
        // Cos(alpha), sin(alpha)
        let triga = Vec2::from_angle(a);
        // Half of width side
        let d = triga * l / 2.0;
        // Return values
        let mut p1 = -d;
        let mut p2 = d;
        if flip {
            p1.x = -p1.x;
            p2.x = -p2.x;
        }
        (p1, p2, tana)
    }

    /// Project a point on a line and return projected point position and distance from the line.
    /// Slope offset (`b` in line equation) is assumed to be 0.
    fn project_to_line(p: Vec2, a: f32) -> (Vec2, f32) {
        let x;
        let y;
        if a.is_zero() {
            x = p.x;
            y = 0.0;
        } else {
            let a2 = -a.recip();
            let b2 = p.y - a2 * p.x;
            let a_a2 = a - a2;
            x = b2 / a_a2;
            y = (a * b2) / a_a2;
        }
        let pp = Vec2::new(x, y);
        (pp, pp.distance(p))
    }
}

impl Sampler for StripSampler {
    fn sample(&self, p: Vec2) -> f32 {
        let p = p - self.offset;
        let mut norm = 1f32;
        // Project point on strip line and see if it's close enough.
        let (pp, len) = Self::project_to_line(p, self.slope_a);
        // NOTE: pp.length() is equivalent to (0, 0).distance(pp).
        if pp.length() <= (self.length / 2.0) {
            norm = (len / self.thickness).min(1.0);
        // See if the point is within first or second end circle.
        } else {
            let len = p.distance(self.start).min(p.distance(self.end));
            if len <= self.thickness {
                norm = len / self.thickness;
            }
        }
        // Interpolate value.
        self.midpoint.lerp(norm)
    }

    fn offset_origin(mut self, offset: Vec2) -> Self {
        self.offset += offset;
        self
    }

    fn set_scale(self, _scale: f32) -> Self {
        self
    }
}

/// Sample Fbm noise value in 2D space.
struct FbmSampler<N> {
    origin: Vec2,
    scale: f32,
    noise: Fbm<N>,
    bias: f32,
    midpoint: QuadPointLerp,
}

impl<N> FbmSampler<N>
where
    N: Default + noise::Seedable,
{
    pub fn new(config: &FbmConfig) -> Self {
        let origin = Vec2::new(config.offset[0], config.offset[1]);
        let noise = Fbm::<N>::new(config.seed)
            .set_octaves(config.detail as usize)
            .set_frequency(config.frequency as f64)
            .set_lacunarity(config.neatness as f64)
            .set_persistence(config.roughness as f64);
        Self {
            origin,
            scale: 1.0,
            noise,
            bias: config.bias,
            midpoint: config.midpoint.clone_precalc(),
        }
    }
}

impl<N> Sampler for FbmSampler<N>
where
    N: NoiseFn<f64, 2>,
{
    fn sample(&self, p: Vec2) -> f32 {
        let xy = p / self.scale + self.origin;
        let sample = (self.noise.get([xy.x as f64, xy.y as f64]) + 1.4) / 2.8;
        // Interpolate value.
        self.midpoint
            .lerp(((sample as f32).clamp(0.0, 1.0) + self.bias).clamp(0.0, 1.0))
    }

    fn offset_origin(self, _offset: Vec2) -> Self {
        self
    }

    fn set_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

/// Sample data by transforming point latitude (height) with a 9-point lerp.
struct LatitudinalSampler {
    pub south_value: f32,
    pub south_arctic_value: f32,
    pub south_temperate_value: f32,
    pub south_tropic_value: f32,
    pub equator_value: f32,
    pub north_tropic_value: f32,
    pub north_temperate_value: f32,
    pub north_arctic_value: f32,
    pub north_value: f32,
    pub height: f32,
    pub non_linear_tropics: bool,
}

impl LatitudinalSampler {
    pub fn new_temp(config: &LatitudinalTemperatureLerp, height: u32) -> Self {
        Self {
            south_value: celsius_to_fraction(config.south_pole_value),
            south_arctic_value: celsius_to_fraction(config.south_arctic_value),
            south_temperate_value: celsius_to_fraction(config.south_temperate_value),
            south_tropic_value: celsius_to_fraction(config.south_tropic_value),
            equator_value: celsius_to_fraction(config.equator_value),
            north_tropic_value: celsius_to_fraction(config.north_tropic_value),
            north_temperate_value: celsius_to_fraction(config.north_temperate_value),
            north_arctic_value: celsius_to_fraction(config.north_arctic_value),
            north_value: celsius_to_fraction(config.north_pole_value),
            height: height as f32,
            non_linear_tropics: config.non_linear_tropics,
        }
    }

    pub fn new_precip(config: &LatitudinalPrecipitationLerp, height: u32) -> Self {
        Self {
            south_value: precip_to_fraction(config.south_pole_value),
            south_arctic_value: precip_to_fraction(config.south_arctic_value),
            south_temperate_value: precip_to_fraction(config.south_temperate_value),
            south_tropic_value: precip_to_fraction(config.south_tropic_value),
            equator_value: precip_to_fraction(config.equator_value),
            north_tropic_value: precip_to_fraction(config.north_tropic_value),
            north_temperate_value: precip_to_fraction(config.north_temperate_value),
            north_arctic_value: precip_to_fraction(config.north_arctic_value),
            north_value: precip_to_fraction(config.north_pole_value),
            height: height as f32,
            non_linear_tropics: config.non_linear_tropics,
        }
    }
}

impl Sampler for LatitudinalSampler {
    fn offset_origin(self, _offset: Vec2) -> Self {
        self
    }

    fn set_scale(self, _scale: f32) -> Self {
        self
    }

    fn sample(&self, p: Vec2) -> f32 {
        let y = p.y / self.height;
        if y < 0.117 {
            // north-arctic
            let range = self.north_value..=self.north_arctic_value;
            let y = y / 0.117;
            lerp(range, y)
        } else if y < 0.244 {
            // arctic-temperate
            let range = self.north_arctic_value..=self.north_temperate_value;
            let y = (y - 0.117) / (0.244 - 0.117);
            lerp(range, y)
        } else if y < 0.372 {
            // temperate-tropic
            let range = self.north_temperate_value..=self.north_tropic_value;
            let mut y = (y - 0.244) / (0.372 - 0.244);
            if self.non_linear_tropics {
                y = y.powi(4);
            }
            lerp(range, y)
        } else if y < 0.5 {
            // tropic-equator
            let range = self.north_tropic_value..=self.equator_value;
            let y = (y - 0.372) / (0.5 - 0.372);
            lerp(range, y)
        } else if y < 0.628 {
            // equator-tropic
            let range = self.equator_value..=self.south_tropic_value;
            let y = (y - 0.5) / (0.628 - 0.5);
            lerp(range, y)
        } else if y < 0.756 {
            // tropic-temperate
            let range = self.south_tropic_value..=self.south_temperate_value;
            let mut y = (y - 0.628) / (0.756 - 0.628);
            if self.non_linear_tropics {
                y = y.sqrt().sqrt();
            }
            lerp(range, y)
        } else if y < 0.883 {
            // temperate-arctic
            let range = self.south_temperate_value..=self.south_arctic_value;
            let y = (y - 0.756) / (0.883 - 0.756);
            lerp(range, y)
        } else {
            // arctic-south
            let range = self.south_arctic_value..=self.south_value;
            let y = (y - 0.883) / (1.0 - 0.883);
            lerp(range, y)
        }
    }
}

/// Sample the entire world space with a latitudinal sampler and replace existing data.
pub fn fill_latitudinal_temp(
    data: &mut [u8],
    model: WorldModel,
    world_size: [u32; 2],
    config: &LatitudinalTemperatureLerp,
) {
    let sampler = LatitudinalSampler::new_temp(config, world_size[1]);
    sample_fill(data, sampler, model, world_size);
}

/// Sample the entire world space with a latitudinal sampler and replace existing data.
pub fn fill_latitudinal_precip(
    data: &mut [u8],
    model: WorldModel,
    world_size: [u32; 2],
    config: &LatitudinalPrecipitationLerp,
) {
    let sampler = LatitudinalSampler::new_precip(config, world_size[1]);
    sample_fill(data, sampler, model, world_size);
}

/// Sample the entire world space with a noise sampler and add it to existing data.
pub fn add_with_algorithm(
    data: &mut [u8],
    model: WorldModel,
    world_size: [u32; 2],
    algorithm: impl AsRef<NoiseAlgorithm>,
    strength: f32,
) {
    if strength.is_zero() {
        return;
    }
    match algorithm.as_ref() {
        NoiseAlgorithm::Perlin(config) => sample_add(
            data,
            FbmSampler::<Perlin>::new(config),
            model,
            world_size,
            strength,
        ),
        NoiseAlgorithm::OpenSimplex(config) => sample_add(
            data,
            FbmSampler::<OpenSimplex>::new(config),
            model,
            world_size,
            strength,
        ),
        NoiseAlgorithm::SuperSimplex(config) => sample_add(
            data,
            FbmSampler::<SuperSimplex>::new(config),
            model,
            world_size,
            strength,
        ),
        NoiseAlgorithm::FromImage => { /* Do nothing. */ }
    }
}

/// Sample the entire world space with a noise sampler and replace existing data.
pub fn fill_with_algorithm(
    data: &mut [u8],
    model: WorldModel,
    world_size: [u32; 2],
    algorithm: impl AsRef<NoiseAlgorithm>,
) {
    match algorithm.as_ref() {
        NoiseAlgorithm::Perlin(config) => {
            sample_fill(data, FbmSampler::<Perlin>::new(config), model, world_size)
        }
        NoiseAlgorithm::OpenSimplex(config) => {
            sample_fill(data, FbmSampler::<OpenSimplex>::new(config), model, world_size)
        }
        NoiseAlgorithm::SuperSimplex(config) => {
            sample_fill(data, FbmSampler::<SuperSimplex>::new(config), model, world_size)
        }
        NoiseAlgorithm::FromImage => { /* Do nothing. */ }
    }
}

/// Fill an influence layer with specified shape (or noise algorithm).
pub fn fill_influence(data: &mut [u8], shape: &InfluenceShape, model: WorldModel, world_size: [u32; 2]) {
    match shape {
        InfluenceShape::None => data.fill(0),
        InfluenceShape::FromImage(_) => { /*Do nothing. */ }
        InfluenceShape::Circle(x) => sample_fill(data, CircleSampler::new(x), model, world_size),
        InfluenceShape::Strip(x) => sample_fill(data, StripSampler::new(x), model, world_size),
        InfluenceShape::Fbm(x) => fill_with_algorithm(data, model, world_size, x),
    }
}

/// Apply influence data to real data with given influence strength.
/// Strength == 0.0 means no effect, strength == 1.0 means max effect.
pub fn apply_influence(data: &mut [u8], influence: &[u8], mode: InfluenceMode, strength: f32) {
    let strength = strength.clamp(0.0, 1.0);
    if strength.is_zero() {
        return;
    }
    match mode {
        // Scale "bad" influence to 0
        InfluenceMode::ScaleDown => {
            for i in 0..data.len() {
                let inf = 1.0 - (1.0 - influence[i] as f32 / 255.0) * strength;
                data[i] = lerp(0.0..=(data[i] as f32), inf) as u8;
            }
        }
        // Scale "good" influence to 1
        InfluenceMode::ScaleUp => {
            for i in 0..data.len() {
                let inf = (influence[i] as f32 / 255.0) * strength;
                data[i] = lerp((data[i] as f32)..=255.0, inf) as u8;
            }
        }
        // Scale both "bad" and "good" influence with baseline being 0.5
        InfluenceMode::ScaleUpDown => {
            for i in 0..data.len() {
                let inf = (influence[i] as f32 / 255.0 - 0.5) * strength;
                data[i] = if inf <= 0.0 {
                    lerp(0.0..=(data[i] as f32), 1.0 + inf * 2.0)
                } else {
                    lerp((data[i] as f32)..=255.0, inf * 2.0)
                } as u8;
            }
        }
    }
}

/// Apply influence data to real data with given influence strength.
/// Strength == 0.0 means no effect, strength == 1.0 means max effect.
pub fn apply_influence_from_src(
    dest: &mut [u8],
    src: &[u8],
    influence: &[u8],
    mode: InfluenceMode,
    strength: f32,
) {
    let strength = strength.clamp(0.0, 1.0);
    if strength.is_zero() {
        return;
    }
    match mode {
        // Scale "bad" influence to 0
        InfluenceMode::ScaleDown => {
            for i in 0..dest.len() {
                let inf = 1.0 - (1.0 - influence[i] as f32 / 255.0) * strength;
                dest[i] = lerp(0.0..=(src[i] as f32), inf) as u8;
            }
        }
        // Scale "good" influence to 1
        InfluenceMode::ScaleUp => {
            for i in 0..dest.len() {
                let inf = (influence[i] as f32 / 255.0) * strength;
                dest[i] = lerp((src[i] as f32)..=255.0, inf) as u8;
            }
        }
        // Scale both "bad" and "good" influence with baseline being 0.5
        InfluenceMode::ScaleUpDown => {
            for i in 0..dest.len() {
                let inf = (influence[i] as f32 / 255.0 - 0.5) * strength;
                dest[i] = if inf <= 0.0 {
                    lerp(0.0..=(src[i] as f32), 1.0 + inf * 2.0)
                } else {
                    lerp((src[i] as f32)..=255.0, inf * 2.0)
                } as u8;
            }
        }
    }
}

fn sample_add(
    data: &mut [u8],
    sampler: impl Sampler,
    model: WorldModel,
    world_size: [u32; 2],
    strength: f32,
) {
    match model {
        WorldModel::Flat => {
            let width = world_size[0] as i32;
            let height = world_size[1] as i32;
            // NOTE: Not respected by FbmSampler.
            let origin = Vec2::new(width as f32 / 2.0, height as f32 / 2.0);
            // NOTE: Only respected by FbmSampler.
            let scale = f32::sqrt((width * height) as f32);
            let sampler = sampler.offset_origin(origin).set_scale(scale);
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let p = Vec2::new(x as f32, y as f32);
                    let value = ((sampler.sample(p) - 0.5) * 127f32 * strength) as i8;
                    data[i] = data[i].saturating_add_signed(value);
                }
            }
        }
        WorldModel::Globe => todo!(), // TODO
    }
}

fn sample_fill(data: &mut [u8], sampler: impl Sampler, model: WorldModel, world_size: [u32; 2])
{
    match model {
        WorldModel::Flat => {
            let width = world_size[0] as i32;
            let height = world_size[1] as i32;
            // NOTE: Not respected by FbmSampler.
            let origin = Vec2::new(width as f32 / 2.0, height as f32 / 2.0);
            // NOTE: Only respected by FbmSampler.
            let scale = f32::sqrt((width * height) as f32);
            let sampler = sampler.offset_origin(origin).set_scale(scale);
            for y in 0..height {
                for x in 0..width {
                    let i = (y * width + x) as usize;
                    let p = Vec2::new(x as f32, y as f32);
                    data[i] = (sampler.sample(p) * 255f32) as u8;
                }
            }
        }
        WorldModel::Globe => todo!(), // TODO
    }
}
