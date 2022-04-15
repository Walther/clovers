use std::{
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

use clovers::{
    scenes::{self, Scene, SceneFile},
    RenderOpts,
};
use eframe::{egui, epi};
use poll_promise::Promise;
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
    texture: Option<egui::TextureHandle>,
    /// Current rendering progress: `(current,total)`
    progress: Arc<Mutex<u32>>,
    /// Thread handler for work outside the GUI thread
    promise: Option<Promise<Vec<u8>>>,
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
            progress: Arc::new(Mutex::new(0)),
            promise: None,
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
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Not using persistence for now
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            input,
            width,
            height,
            samples,
            max_depth,
            gamma,
            gpu,
            normalmap,
            texture: _,
            progress: _,
            promise: _,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
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

            ui.heading("Placeholder options - currently implemented in CLI only");
            ui.add(egui::Checkbox::new(gpu, "use gpu rendering"));
            ui.add(egui::Checkbox::new(normalmap, "only render a normal map"));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                // Progress bar
                let p = self.progress.lock().unwrap();
                let bar = *p as f32 / *samples as f32;
                drop(p);
                ui.add(egui::ProgressBar::new(bar));

                // Render button
                if ui.button("Render!").clicked() {
                    // TODO: error handling

                    // Clear previous image
                    self.texture = None;
                    let progress = Arc::clone(&self.progress);
                    let mut p = progress.lock().unwrap();
                    *p = 0;
                    drop(p);

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

                    // TODO: why are these manual clones needed? closure ownership is confusing
                    let s = samples.clone();
                    let w = width.clone() as usize;
                    let h = height.clone() as usize;

                    self.promise = Some(Promise::spawn_thread("renderer", move || {
                        info!("Creating the renderer");
                        let mut renderer = draw_gui::Renderer::new(scene, renderopts);
                        let mut pixelbuffer = vec![0; 4 * w * h];
                        info!("Calling draw()");
                        for frame_number in 1..=s {
                            info!("Rendering sample {} of {}", &frame_number, s);
                            let mut p = progress.lock().unwrap();
                            *p = frame_number;
                            renderer.draw(&mut pixelbuffer, frame_number);
                            drop(p);
                        }
                        pixelbuffer
                    }));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let w = self.width.clone() as usize;
            let h = self.height.clone() as usize;

            // Are we currently rendering?
            if let Some(promise) = &self.promise {
                if let Some(result) = promise.ready() {
                    // Use/show result
                    info!("Creating the texture");
                    let image = egui::ColorImage::from_rgba_unmultiplied([w, h], &result);
                    let _texture_id = self.texture.get_or_insert_with(|| {
                        // Load the texture only once.
                        ui.ctx().load_texture("rendered_image", image)
                    });
                    ctx.request_repaint();
                    self.promise = None;
                } else {
                    let p = self.progress.lock().unwrap();
                    ui.heading(format!("Rendering progress: {} of {}", *p, self.samples));
                    drop(p);
                    ctx.request_repaint();
                }
            }

            // If we have a render result in the texture, show it
            if let Some(texture) = &self.texture {
                ui.heading("Render result");
                ui.image(
                    texture,
                    egui::Vec2::new(self.width as f32, self.height as f32),
                );
            }

            // If we are in a fresh window, show instructions
            if self.promise.is_none() && self.texture.is_none() {
                // Fresh window; show a default heading
                ui.heading("Select your rendering options & press Render!");
            }

            egui::warn_if_debug_build(ui);
        });
    }
}
