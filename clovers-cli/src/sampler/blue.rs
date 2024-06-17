//! A sampler based on blue noise. Works especially well at low samples-per-pixel counts.
//!
//! Utilizes library code from <https://github.com/Jasper-Bekkers/blue-noise-sampler>.

use clovers::{wavelength::sample_wavelength, Float, Vec2, PI};

use super::*;

pub struct BlueSampler {
    get: fn(i32, i32, i32, SamplerDimension) -> Float,
}

impl BlueSampler {
    pub fn new(samples: u32) -> Self {
        let get = match samples {
            1 => blue_sample_spp1,
            2 => blue_sample_spp2,
            4 => blue_sample_spp4,
            8 => blue_sample_spp8,
            16 => blue_sample_spp16,
            32 => blue_sample_spp32,
            64 => blue_sample_spp64,
            128 => blue_sample_spp128,
            256 => blue_sample_spp256,
            _ => unimplemented!(
                "blue sampler only supports sample-per-pixel counts that are powers of two of and up to 256"
            ),
        };
        Self { get }
    }
}

impl<'scene> SamplerTrait<'scene> for BlueSampler {
    fn sample(&mut self, i: i32, j: i32, index: i32) -> Randomness {
        let pixel_offset = Vec2::new(
            (self.get)(i, j, index, SamplerDimension::PixelOffsetX),
            (self.get)(i, j, index, SamplerDimension::PixelOffsetY),
        );
        let lens_offset = in_unit_disk(
            (self.get)(i, j, index, SamplerDimension::LensOffsetX),
            (self.get)(i, j, index, SamplerDimension::LensOffsetY),
        );
        let time = (self.get)(i, j, index, SamplerDimension::Time);
        // TODO: verify uniformity & correctness for math?
        let wavelength = sample_wavelength((self.get)(i, j, index, SamplerDimension::Wavelength));

        Randomness {
            pixel_offset,
            lens_offset,
            time,
            wavelength,
        }
    }

    fn sample_dimension(
        &mut self,
        i: i32,
        j: i32,
        index: i32,
        dimension: SamplerDimension,
    ) -> Float {
        (self.get)(i, j, index, dimension)
    }
}

macro_rules! define_blue_sampler {
    ($spp:ident) => {
        ::paste::paste! {
            pub fn [<blue_sample_ $spp>](
                mut pixel_i: i32,
                mut pixel_j: i32,
                mut sample_index: i32,
                sample_dimension: SamplerDimension) -> Float {
                    let mut sample_dimension = sample_dimension as i32;

                    use blue_noise_sampler::$spp::*;

                    // Adapted from <https://dl.acm.org/doi/10.1145/3306307.3328191> and <https://github.com/Jasper-Bekkers/blue-noise-sampler>

                    // wrap arguments
                    pixel_i &= 127;
                    pixel_j &= 127;
                    sample_index &= 255;
                    sample_dimension &= 255;

                    // xor index based on optimized ranking
                    // jb: 1spp blue noise has all 0 in g_blueNoiseRankingTile so we can skip the load
                    let index = sample_dimension + (pixel_i + pixel_j * 128) * 8;
                    let index = index as usize;
                    let ranked_sample_index = sample_index ^ RANKING_TILE[index];

                    // fetch value in sequence
                    let index = sample_dimension + ranked_sample_index * 256;
                    let index = index as usize;
                    let value = SOBOL[index];

                    // If the dimension is optimized, xor sequence value based on optimized scrambling
                    let index = (sample_dimension % 8) + (pixel_i + pixel_j * 128) * 8;
                    let index = index as usize;
                    let value = value ^ SCRAMBLING_TILE[index];

                    // convert to float and return
                    let v: Float = (0.5 + value as Float) / 256.0;
                    v
            }
        }
    };
}

define_blue_sampler!(spp1);
define_blue_sampler!(spp2);
define_blue_sampler!(spp4);
define_blue_sampler!(spp8);
define_blue_sampler!(spp16);
define_blue_sampler!(spp32);
define_blue_sampler!(spp64);
define_blue_sampler!(spp128);
define_blue_sampler!(spp256);

/// Given two samples in range `[0..1]`, return a sample within `[-0.5 .. 0.5]` unit disk.
/// Based on <https://stackoverflow.com/a/50746409>
fn in_unit_disk(x: Float, y: Float) -> Vec2 {
    // Polar coordinates + correcting for the distribution using sqrt
    let r = x.sqrt();
    let theta = y * 2.0 * PI;
    // Conversion to Cartesian coordinates
    let x = r * theta.cos();
    let y = r * theta.sin();
    Vec2::new(x, y)
}
