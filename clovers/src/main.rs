#![deny(clippy::all)]

// External imports
use chrono::Utc;
use clap::Clap;
use clovers::color::Color;
use humantime::format_duration;
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::File;
use std::{error::Error, fs, time::Instant};
use tracing::*;
use tracing_timing::{Builder, Histogram};

// Internal imports
use clovers::*;
mod draw;
use draw::draw;
use scenes::Scene;

// Configure CLI parameters
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Walther")]
struct Opts {
    /// Input filename / location
    #[clap(short, long)]
    input: String,
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
}

fn main() -> Result<(), Box<dyn Error>> {
    // Tracing
    let s = Builder::default().build(|| Histogram::new_with_bounds(10_000, 1_000_000, 3).unwrap());
    let sid = s.downcaster();

    let d = Dispatch::new(s);
    let d2 = d.clone();

    let _dispatcher_result = dispatcher::set_global_default(d2);

    // Tracing debug: uncommenting this makes the hashmap assert pass
    // trace_span!("draw").in_scope(|| {
    //     trace!("ray_color");
    //     trace!("ray_none");
    // });

    // CLI
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

    // Read the given scene file
    let file = File::open(&opts.input)?;
    let scene: Scene = scenes::initialize(file, opts.width, opts.height)?;

    // Note: live progress bar printed within draw
    let start = Instant::now();

    let pixelbuffer = draw(
        opts.width,
        opts.height,
        opts.samples,
        opts.max_depth,
        opts.gamma,
        scene,
    );

    // Translate our internal pixelbuffer into an Image buffer
    let width = opts.width;
    let height = opts.height;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = y * width + x;
        *pixel = Rgb(pixelbuffer[index as usize].to_rgb_u8());
    });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    // Our coordinate system is weird in general, try flipping this way too.
    image::imageops::flip_horizontal_in_place(&mut img);
    // TODO: fix the coordinate system

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

    // prettyprint a histogram
    let timings = sid.downcast(&d).unwrap();
    // Now that we are done rendering, ensure all metrics are up to date.
    timings.force_synchronize();
    timings.with_histograms(|hs| {
        for (span_group, hs) in hs {
            for (event_group, h) in hs {
                // make sure we see the latest samples:
                h.refresh_timeout(std::time::Duration::from_secs(10));
                println!("\n{} -> {}:", span_group, event_group,);
                println!(
                    " | mean: {:.1}µs, p50: {}µs, p90: {}µs, p99: {}µs, p999: {}µs, max: {}µs\n | ",
                    h.mean() / 1000.0,
                    h.value_at_quantile(0.5) / 1_000,
                    h.value_at_quantile(0.9) / 1_000,
                    h.value_at_quantile(0.99) / 1_000,
                    h.value_at_quantile(0.999) / 1_000,
                    h.max() / 1_000,
                );
                for v in break_once(
                    h.iter_linear(25_000).skip_while(|v| v.quantile() < 0.01),
                    |v| v.quantile() > 0.95,
                ) {
                    println!(
                        " | {:4}µs | {:40} | {:4.1}th %-ile",
                        (v.value_iterated_to() + 1) / 1_000,
                        "*".repeat(
                            (v.count_since_last_iteration() as f64 * 40.0 / h.len() as f64).ceil()
                                as usize
                        ),
                        v.percentile(),
                    );
                }
            }
        }
    });

    Ok(())
}

// https://github.com/jonhoo/tracing-timing/blob/master/examples/pretty.rs
// until we have https://github.com/rust-lang/rust/issues/62208
fn break_once<I, F>(it: I, mut f: F) -> impl Iterator<Item = I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> bool,
{
    let mut got_true = false;
    it.into_iter().take_while(move |i| {
        if got_true {
            // we've already yielded when f was true
            return false;
        }
        if f(i) {
            // this must be the first time f returns true
            // we should yield i, and then no more
            got_true = true;
        }
        // f returned false, so we should keep yielding
        true
    })
}
