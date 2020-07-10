#![deny(clippy::all)]
// A lot of loader functions etc, suppresses some warning noise
#![allow(dead_code)]

// External imports
use chrono::Utc;
use clap::Clap;
use humantime::format_duration;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::{error::Error, fs, sync::Arc, time::Instant};

// Internal imports
use clovers::*;
mod draw;
use draw::draw;
#[cfg(feature = "gui")]
mod draw_gui;
use camera::{Camera, CameraInit};
use color::Color;
#[cfg(feature = "gui")]
use draw_gui::draw_gui;
use hitable::{Hitable, HitableList};
use materials::{Dielectric, DiffuseLight};
use objects::{Boxy, BoxyInit, FlipFace, RotateY, Sphere, Translate, XYRect, XZRect, YZRect};
use scenes::Scene;
use textures::{SolidColor, Texture};

// Configure CLI parameters
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Walther")]
struct Opts {
    /// Output filename / location. [default: renders/timestamp.png]
    #[clap(short, long)]
    output: Option<String>,
    /// Width of the image in pixels
    #[clap(short, long, default_value = "1024")]
    width: u32,
    /// Height of the image in pixels
    #[clap(short, long, default_value = "1024")]
    height: u32,
    /// Number of samples to generate per each pixel
    #[clap(short, long, default_value = "100")]
    samples: u32,
    /// Maximum evaluated bounce depth for each ray
    #[clap(short, long, default_value = "100")]
    max_depth: u32,
    /// Gamma correction value
    #[clap(short, long, default_value = "2.0")]
    gamma: Float,
    /// Optional GUI with iterative rendering
    #[clap(long)]
    gui: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "cli")]
    let opts: Opts = Opts::parse();

    println!("clovers 🍀    ray tracing in rust 🦀");
    println!("width:        {}", opts.width);
    println!("height:       {}", opts.height);
    println!("samples:      {}", opts.samples);
    println!("max depth:    {}", opts.max_depth);
    let rays: u64 =
        opts.width as u64 * opts.height as u64 * opts.samples as u64 * opts.max_depth as u64;
    println!("approx. rays: {}", rays);
    println!(); // Empty line before progress bar

    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    enum ObjectLiteral {
        XZRect(XZRect),
        XYRect(XYRect),
        YZRect(YZRect),
        Sphere(Sphere),
        Boxy(BoxyInit),
    }

    impl From<ObjectLiteral> for Hitable {
        fn from(obj: ObjectLiteral) -> Hitable {
            match obj {
                ObjectLiteral::XZRect(x) => Hitable::XZRect(x),
                ObjectLiteral::XYRect(x) => Hitable::XYRect(x),
                ObjectLiteral::YZRect(x) => Hitable::YZRect(x),
                ObjectLiteral::Sphere(x) => Hitable::Sphere(x),
                ObjectLiteral::Boxy(x) => Boxy::new(x.corner_0, x.corner_1, x.material),
            }
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    enum MetaObject {
        FlipFace(ObjectLiteral),
        RotateY(RotateInit),
        Translate(TranslateInit),
    }

    impl From<MetaObject> for Hitable {
        fn from(obj: MetaObject) -> Hitable {
            match obj {
                MetaObject::FlipFace(x) => FlipFace::new(x.into()),
                MetaObject::RotateY(x) => {
                    let obj = *x.object;
                    let obj: Hitable = obj.into();
                    RotateY::new(Arc::new(obj), x.angle)
                }
                MetaObject::Translate(x) => {
                    let obj = *x.object;
                    let obj: Hitable = obj.into();
                    Translate::new(Arc::new(obj), x.offset)
                }
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RotateInit {
        object: Box<Object>,
        angle: Float,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TranslateInit {
        object: Box<Object>,
        offset: Vec3,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    enum Object {
        ObjectLiteral(ObjectLiteral),
        MetaObject(MetaObject),
    }

    impl From<Object> for Hitable {
        fn from(obj: Object) -> Hitable {
            match obj {
                Object::ObjectLiteral(obj) => obj.into(),
                Object::MetaObject(obj) => obj.into(),
            }
        }
    }

    // TODO: temporary let's try this serde thing out
    #[derive(Serialize, Deserialize, Debug)]
    struct SceneFile {
        time_0: Float,
        time_1: Float,
        background_color: Color,
        camera: CameraInit,
        objects: Vec<Object>,
        priority_objects: Vec<Object>,
    }
    let mut file = File::open("scenes/scene.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let scene_file: SceneFile = serde_json::from_str(&contents)?;
    let time_0 = scene_file.time_0;
    let time_1 = scene_file.time_1;
    let rng = rand::thread_rng();
    let background_color = scene_file.background_color;
    let camera = Camera::new(
        scene_file.camera.look_from,
        scene_file.camera.look_at,
        scene_file.camera.up,
        scene_file.camera.vertical_fov,
        opts.width as Float / opts.height as Float,
        scene_file.camera.aperture,
        scene_file.camera.focus_distance,
        time_0,
        time_1,
    );
    let mut hitables = HitableList::new();
    for obj in scene_file.objects {
        hitables.add(obj.into());
    }
    let scene = Scene::new(hitables, camera, time_0, time_1, background_color, rng);

    let mut priority_objects = HitableList::new();
    for obj in scene_file.priority_objects {
        priority_objects.add(obj.into());
    }
    let priority_objects = priority_objects.into_hitable();
    let priority_objects = Arc::new(priority_objects);

    // gui version
    if opts.gui {
        if cfg!(feature = "gui") {
            #[cfg(feature = "gui")]
            let _result = draw_gui(
                opts.width,
                opts.height,
                opts.samples,
                opts.max_depth,
                opts.gamma,
                scene,
                priority_objects,
            );
            return Ok(());
        } else {
            println!("clovers not built with feature 'gui' enabled");
            return Ok(());
        }
    }

    // cli version
    let start = Instant::now();
    let img = draw(
        opts.width,
        opts.height,
        opts.samples,
        opts.max_depth,
        opts.gamma,
        scene,
        priority_objects,
    )?; // Note: live progress bar printed within draw
    let duration = Instant::now() - start;

    println!(); // Empty line after progress bar
    println!("finished render in {}", format_duration(duration));

    // Write
    let target: String;
    match opts.output {
        Some(filename) => {
            target = filename;
        }
        None => {
            // Default to using a timestamp & `renders/` directory
            let timestamp = Utc::now().timestamp();
            fs::create_dir_all("renders")?;
            target = format!("renders/{}.png", timestamp);
        }
    };
    img.save(format!("{}", target))?;
    println!("output saved: {}", target);

    Ok(())
}
