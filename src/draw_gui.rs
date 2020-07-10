use crate::{
    color::Color,
    colorize::colorize,
    hitable::{Hitable, HitableList},
    materials::DiffuseLight,
    objects::XZRect,
    scenes::{self, Scene},
    textures::SolidColor,
    Float,
};

use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

#[cfg(feature = "gui")]
use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use std::sync::Arc;
#[cfg(feature = "gui")]
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
#[cfg(feature = "gui")]
use winit_input_helper::WinitInputHelper;

#[cfg(feature = "gui")]
pub fn draw_gui(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    gamma: Float,
    scene: Scene,
    lights: Arc<Hitable>,
) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("clovers 🍀 ray tracing in rust 🦀")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut hidpi_factor = window.scale_factor();
    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(width, height, surface);
        Pixels::new(width, height, surface_texture)?
    };

    let mut world = World::new(width, height, samples, max_depth, gamma, scene, lights);
    let mut frame_num = 0;

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            frame_num += 1;
            world.draw(pixels.get_frame(), frame_num);
            if pixels
                .render()
                .map_err(|e| println!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                hidpi_factor = factor;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

struct World {
    width: u32,
    height: u32,
    scene: Scene,
    lights: Arc<Hitable>, // TODO: fix / improve
    float_buffer: Vec<Float>,
    bar: ProgressBar,
    samples: u32,
    max_depth: u32,
    gamma: Float,
}

impl World {
    fn new(
        width: u32,
        height: u32,
        samples: u32,
        max_depth: u32,
        gamma: Float,
        scene: Scene,
        lights: Arc<Hitable>,
    ) -> Self {
        // Progress bar
        let bar = ProgressBar::new(samples as u64);
        bar.set_style(ProgressStyle::default_bar().template(
            "Elapsed: {elapsed_precise}\nSamples:  {bar} {pos}/{len}\nETA:     {eta_precise}",
        ));

        World {
            width,
            height,
            scene,
            lights,
            float_buffer: vec![0.0; 4 * width as usize * height as usize], // rgba
            bar,
            samples,
            max_depth,
            gamma,
        }
    }

    // Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn draw(&mut self, frame: &mut [u8], frame_num: u32) {
        let background_color = self.scene.background;

        // Stop iterating
        if frame_num > self.samples {
            return;
        }
        let width = self.width as usize;
        let height = self.height as usize;
        let camera = &self.scene.camera;
        let world = &self.scene.world;
        let lights = &self.lights;
        let d = self.max_depth.clone(); // TODO: cleanup, silly

        // Update internal float-based pixel buffer with new samples
        self.float_buffer
            .par_chunks_exact_mut(4)
            .enumerate()
            .for_each(|(i, pixel)| {
                let x = (i % width) as i16;
                let y = height as i16 - (i / width) as i16; // flip y-axis

                let mut rng = rand::thread_rng();
                let mut color: Color = Color::new(0.0, 0.0, 0.0);

                let u = (x as Float + rng.gen::<Float>()) / width as Float;
                let v = (y as Float + rng.gen::<Float>()) / height as Float;
                let ray = camera.get_ray(u, v, rng);
                color += colorize(
                    &ray,
                    background_color,
                    world,
                    Arc::clone(&lights), // TODO: Fixme, ridiculous and unusable
                    0,
                    d,
                    rng,
                );

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
                // weight the pixel down based on frame number
                let rgba = [rgb[0], rgb[1], rgb[2], 0xFF]; //TODO: alpha in color?

                pixel.copy_from_slice(&rgba);
            });
        self.bar.inc(1);
    }

    fn update(&self) {
        // TODO
    }
}
