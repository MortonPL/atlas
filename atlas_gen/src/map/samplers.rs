use bevy::{math::Vec2, utils::petgraph::matrix_graph::Zero};
use bevy_egui::egui::lerp;
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex, Perlin, SuperSimplex};

use crate::config::{
    CircleSamplerConfig, FbmConfig, InfluenceShape, NoiseAlgorithm, StripSamplerConfig, WorldModel,
};

trait Sampler {
    fn offset_origin(self, offset: Vec2) -> Self;
    fn set_scale(self, scale: f32) -> Self;
    fn sample(&self, p: Vec2) -> f32;
}

struct CircleSampler {
    offset: Vec2,
    radius: f32,
    midpoint: f32,
    midpoint_value: f32,
}

impl CircleSampler {
    fn new(config: &CircleSamplerConfig) -> Self {
        let offset = Vec2::new(config.offset[0] as f32, config.offset[1] as f32);
        Self {
            offset,
            radius: config.radius as f32,
            midpoint: config.midpoint,
            midpoint_value: config.midpoint_value,
        }
    }
}

impl Sampler for CircleSampler {
    fn sample(&self, p: Vec2) -> f32 {
        // Calculate the distance from circle center.
        let len = p.distance(self.offset);
        // Transform the distance as a fraction of radius.
        let norm = (len / self.radius).clamp(0.0, 1.0);
        // Interpolate value using the midpoint and midpoint value.
        if norm <= self.midpoint {
            lerp(1.0..=self.midpoint_value, norm / self.midpoint)
        } else {
            lerp(
                self.midpoint_value..=0.0,
                (norm - self.midpoint) / (1.0 - self.midpoint),
            )
        }
    }

    fn offset_origin(mut self, offset: Vec2) -> Self {
        self.offset += offset;
        self
    }

    fn set_scale(self, _scale: f32) -> Self {
        self
    }
}

struct StripSampler {
    offset: Vec2,
    start: Vec2,
    end: Vec2,
    length: f32,
    thickness: f32,
    slope_a: f32,
    midpoint: f32,
    midpoint_value: f32,
}

impl StripSampler {
    fn new(config: &StripSamplerConfig) -> Self {
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
            midpoint: config.midpoint,
            midpoint_value: config.midpoint_value,
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
        // Interpolate value using the midpoint and midpoint value.
        if norm <= self.midpoint {
            lerp(1.0..=self.midpoint_value, norm / self.midpoint)
        } else {
            lerp(
                self.midpoint_value..=0.0,
                (norm - self.midpoint) / (1.0 - self.midpoint),
            )
        }
    }

    fn offset_origin(mut self, offset: Vec2) -> Self {
        self.offset += offset;
        self
    }

    fn set_scale(self, _scale: f32) -> Self {
        self
    }
}

struct FbmSampler<N> {
    origin: Vec2,
    scale: f32,
    noise: Fbm<N>,
    bias: f32,
    range: f32,
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
            range: config.range,
        }
    }
}

impl<N> Sampler for FbmSampler<N>
where
    N: NoiseFn<f64, 2>,
{
    fn sample(&self, p: Vec2) -> f32 {
        let xy = p / self.scale + self.origin;
        let sample = (self.noise.get([xy.x as f64, xy.y as f64]) + 1.0) / 2.0;
        ((sample as f32 + self.bias).clamp(0.0, 1.0) * self.range).clamp(0.0, 1.0)
    }

    fn offset_origin(self, _offset: Vec2) -> Self {
        self
    }

    fn set_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

/// Fill a data layer with specified noise algorithm.
pub fn fill_noise(data: &mut [u8], config: &FbmConfig, model: &WorldModel, algorithm: NoiseAlgorithm) {
    match algorithm {
        NoiseAlgorithm::Perlin => sample_fill(data, FbmSampler::<Perlin>::new(config), model),
        NoiseAlgorithm::OpenSimplex => sample_fill(data, FbmSampler::<OpenSimplex>::new(config), model),
        NoiseAlgorithm::SuperSimplex => sample_fill(data, FbmSampler::<SuperSimplex>::new(config), model),
        NoiseAlgorithm::FromImage => { /* Do nothing. */ }
    }
}

/// Fill an influence layer with specified shape (or noise algorithm).
pub fn fill_influence(data: &mut [u8], shape: &InfluenceShape, model: &WorldModel) {
    match shape {
        InfluenceShape::None(_) => unreachable!(),
        InfluenceShape::FromImage(_) => unreachable!(),
        InfluenceShape::Circle(x) => sample_fill(data, CircleSampler::new(x), model),
        InfluenceShape::Strip(x) => sample_fill(data, StripSampler::new(x), model),
        InfluenceShape::Fbm(x) => fill_noise(data, &x.config, model, x.algorithm),
    }
}

/// Apply influence data to real data with given influence strength.
/// Real data is multiplied by ratio of influence data to 255.
/// Strength == 0.0 means no effect, strength == 1.0 means max effect.
pub fn apply_influence(data: &mut [u8], influence: &[u8], strength: f32) {
    let strength = strength.clamp(0.0, 1.0);
    if strength.is_zero() {
        return;
    }
    for i in 0..data.len() {
        let inf = 1.0 - (1.0 - influence[i] as f32 / 255.0) * strength;
        data[i] = (data[i] as f32 * inf) as u8;
    }
}

fn sample_fill<T>(data: &mut [u8], sampler: T, model: &WorldModel)
where
    T: Sampler,
{
    match model {
        WorldModel::Flat(flat) => {
            let width = flat.world_size[0] as i32;
            let height = flat.world_size[1] as i32;
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
        WorldModel::Globe(_) => todo!(), // TODO
    }
}
