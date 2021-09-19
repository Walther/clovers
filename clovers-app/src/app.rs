use std::{fs::File, io::Read};

use clovers::{
    scenes::{self, Scene, SceneFile},
    RenderOpts,
};
use eframe::{egui, epi};
use tracing::info;

use crate::draw_gui;

pub struct CloversApp {
    /// Input filename / location
    input: String,
    /// Width of the image in pixels
    width: u32,
    /// Height of the image in pixels
    height: u32,
    /// Number of samples to generate per each pixel
    samples: u32,
    /// Maximum evaluated bounce depth for each ray
    max_depth: u32,
    /// Gamma correction value
    gamma: f32,
    /// Use the GPU draw process instead of CPU
    gpu: bool,
    /// Render a normal map only. Experimental feature.
    normalmap: bool,
    /// Texture to render the image to
    texture: Option<egui::TextureId>,
}

impl Default for CloversApp {
    fn default() -> Self {
        Self {
            input: "scenes/scene.json".to_owned(),
            width: 512,
            height: 512,
            samples: 100,
            max_depth: 100,
            gamma: 2.0,
            gpu: false, // TODO: gpu rendering by default <3
            normalmap: false,
            texture: None,
        }
    }
}

impl epi::App for CloversApp {
    fn name(&self) -> &str {
        "clovers 🍀 ray tracing in rust 🦀"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Not using persistence for now
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            input,
            width,
            height,
            samples,
            max_depth,
            gamma,
            gpu,
            normalmap,
            texture,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Rendering options");
            ui.add(egui::Slider::new(width, 0..=4096).text("width"));
            ui.add(egui::Slider::new(height, 0..=4096).text("height"));
            ui.add(egui::Slider::new(samples, 0..=10_000).text("samples per pixel"));
            ui.add(egui::Slider::new(max_depth, 0..=1000).text("max ray bounce depth"));
            ui.add(egui::Slider::new(gamma, 0.0..=10.0).text("gamma"));

            ui.heading("File to render");
            ui.add(egui::TextEdit::singleline(input));

            ui.heading("Experimental options");
            ui.add(egui::Checkbox::new(gpu, "use gpu rendering"));
            ui.add(egui::Checkbox::new(normalmap, "only render a normal map"));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("Render!").clicked() {
                    // TODO: error handling

                    // Read the given scene file
                    info!("Reading the scene file");
                    let mut file = File::open(input.clone()).unwrap();
                    let mut contents: String = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    info!("Parsing the scene file");
                    let scene_file: SceneFile = serde_json::from_str(&contents).unwrap();
                    info!("Initializing the scene");
                    let scene: Scene = scenes::initialize(scene_file, *width, *height);

                    let renderopts: RenderOpts = RenderOpts {
                        width: *width,
                        height: *height,
                        samples: *samples,
                        max_depth: *max_depth,
                        gamma: *gamma,
                        quiet: true,
                        normalmap: *normalmap,
                    };

                    info!("Creating the renderer");
                    let mut renderer = draw_gui::Renderer::new(scene, renderopts);
                    let mut pixelbuffer = vec![0; 4 * *width as usize * *height as usize];
                    info!("Calling draw()");
                    for frame_number in 1..=*samples {
                        renderer.draw(&mut pixelbuffer, frame_number);
                    }

                    info!("Collecting the pixelbuffer");
                    let pixels: Vec<_> = pixelbuffer
                        .chunks_exact(4)
                        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                        .collect();

                    info!("Creating the texture");
                    let texture_id = frame
                        .tex_allocator()
                        .alloc_srgba_premultiplied((*width as usize, *height as usize), &pixels);
                    info!("Setting the texture to the state");
                    *texture = Some(texture_id);
                    info!("{:?}", texture_id);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Render result");
            if let Some(texture) = self.texture {
                ui.image(
                    texture,
                    egui::Vec2::new(self.width as f32, self.height as f32),
                );
            }
            egui::warn_if_debug_build(ui);
        });
    }
}
