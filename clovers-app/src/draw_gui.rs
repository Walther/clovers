use clovers::{color::Color, colorize::colorize, scenes::Scene, Float};

use clovers::RenderOpts;
// use indicatif::{ProgressBar, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

// TODO: clean up
pub struct Renderer {
    width: u32,
    height: u32,
    scene: Scene,
    float_buffer: Vec<Float>,
    // bar: ProgressBar,
    samples: u32,
    max_depth: u32,
    gamma: Float,
}

impl Renderer {
    pub fn new(scene: Scene, opts: RenderOpts) -> Self {
        // Progress bar
        // let bar = ProgressBar::new(opts.samples as u64);
        // bar.set_style(ProgressStyle::default_bar().template(
        //     "Elapsed: {elapsed_precise}\nSamples:  {bar} {pos}/{len}\nETA:     {eta_precise}",
        // ));

        Renderer {
            width: opts.width,
            height: opts.height,
            scene,
            float_buffer: vec![0.0; 4 * opts.width as usize * opts.height as usize], // rgba
            // bar,
            samples: opts.samples,
            max_depth: opts.max_depth,
            gamma: opts.gamma,
        }
    }

    // Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&mut self, frame: &mut [u8], frame_num: u32) {
        // Stop iterating
        if frame_num > self.samples {
            return;
        }
        let width = self.width as usize;
        let height = self.height as usize;
        let camera = &self.scene.camera;
        let scene = &self.scene;
        let max_depth = self.max_depth;

        // Update internal float-based pixel buffer with new samples
        self.float_buffer
            .par_chunks_exact_mut(4)
            .enumerate()
            .for_each(|(i, pixel)| {
                let x = (i % width) as i16;
                let y = height as i16 - (i / width) as i16; // flip y-axis

                let mut rng = SmallRng::from_entropy();
                let mut color: Color = Color::new(0.0, 0.0, 0.0);

                let u = (x as Float + rng.gen::<Float>()) / width as Float;
                let v = (y as Float + rng.gen::<Float>()) / height as Float;
                let ray = camera.get_ray(u, v, &mut rng);
                let new_color = colorize(&ray, scene, 0, max_depth, &mut rng);
                // skip NaN and Infinity
                if new_color.r.is_finite() && new_color.g.is_finite() && new_color.b.is_finite() {
                    color += new_color;
                }

                // sum to previous color; remember to divide in a consumer!
                let prev_color = Color::new(pixel[0], pixel[1], pixel[2]);
                color = prev_color + color;

                // write
                let rgba = &[color.r, color.g, color.b, 1.0];
                pixel.copy_from_slice(rgba);
            });

        // Write to actual framebuffer
        frame
            .par_chunks_exact_mut(4)
            .enumerate()
            .for_each(|(i, pixel)| {
                let r = self.float_buffer[i * 4];
                let g = self.float_buffer[i * 4 + 1];
                let b = self.float_buffer[i * 4 + 2];
                let _a = self.float_buffer[i * 4 + 3];
                // NOTE: divided because internal floatbuffer keeps summing values
                let color = Color::new(r, g, b) / frame_num as Float;
                // gamma correction
                let color = color.gamma_correction(self.gamma);
                let rgb = color.to_rgb_u8();
                let rgba = [rgb[0], rgb[1], rgb[2], 0xFF]; //TODO: alpha in color?

                pixel.copy_from_slice(&rgba);
            });
        // self.bar.inc(1);
    }
}
