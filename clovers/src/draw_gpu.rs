use bytemuck::{Pod, Zeroable};
use clovers::{color::Color, scenes::Scene, Float};
use log::debug;
use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder};
use std::{borrow::Cow, mem::size_of, path::PathBuf};
use wgpu::{Extent3d, TextureAspect, TextureViewDescriptor};

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
mod shaders {
    #[allow(non_upper_case_globals)]
    pub const main_fs: &str = "main_fs";
    #[allow(non_upper_case_globals)]
    pub const main_vs: &str = "main_vs";
}

// TODO: borrowed from rust-gpu
// TODO: is this needed? could this be improved?
fn create_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    swapchain_format: wgpu::TextureFormat,
    shader_binary: wgpu::ShaderModuleDescriptor<'_>,
) -> wgpu::RenderPipeline {
    let module = device.create_shader_module(&shader_binary);
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: shaders::main_vs,
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            clamp_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: shaders::main_fs,
            targets: &[wgpu::ColorTargetState {
                format: swapchain_format,
                blend: None,
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
    })
}
// END borrowed from rust-gpu

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct ShaderConstants {
    // TODO: actual shader constants
// pub width: u32,
// pub height: u32,
// pub samples: u32,
// pub max_depth: u32,
// pub time: f32,
}

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub async fn draw(
    width: u32,
    height: u32,
    _samples: u32,
    _max_depth: u32,
    _gamma: Float,
    _quiet: bool,
    _scene: Scene,
) -> Vec<Color> {
    // Initialize the GPU instance
    let instance = wgpu::Instance::new(wgpu::BackendBit::VULKAN | wgpu::BackendBit::METAL);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Do not request a drawing surface; headless mode
            compatible_surface: None,
        })
        .await
        .expect("Failed to find an appropriate adapter");
    let features = wgpu::Features::PUSH_CONSTANTS;
    let limits = wgpu::Limits {
        max_push_constant_size: 256,
        ..Default::default()
    };
    // Create the logical device and command queue
    let (device, _queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features,
                limits,
            },
            None,
        )
        .await
        .expect("Failed to create device");
    // Load the shaders from disk
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[wgpu::PushConstantRange {
            stages: wgpu::ShaderStage::all(),
            range: 0..std::mem::size_of::<ShaderConstants>() as u32,
        }],
    });

    // TODO: which format to use?
    let swapchain_format = wgpu::TextureFormat::Rgba8Sint; // TODO: rgbaf32 probably?

    // TODO: build shaders at build time, not at runtime
    let shader_mod_desc = load_shader_module_desc();
    debug!("Shader loaded");
    let _shader_mod = device.create_shader_module(&shader_mod_desc);
    debug!("Shader module created");

    // TODO: what do we need for actually running the shader?
    let render_pipeline =
        create_pipeline(&device, &pipeline_layout, swapchain_format, shader_mod_desc);
    debug!("Pipeline created");
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    debug!("Encoder created");

    // TODO: is this valid? mostly copypasted from docs https://docs.rs/wgpu-types/0.10.0/wgpu_types/struct.TextureDescriptor.html
    let texture_desc = wgpu::TextureDescriptor {
        label: None,
        size: Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D3,
        format: wgpu::TextureFormat::Rgba8Sint, // TODO: rgbaf32 probably?
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
    };
    let texture = device.create_texture(&texture_desc);
    debug!("Texture created");
    // TODO: is this valid? these are complete guesses based on above
    let texture_view_desc = TextureViewDescriptor {
        label: None,
        format: Some(wgpu::TextureFormat::Rgba8Sint), // TODO: rgbaf32 probably?
        dimension: Some(wgpu::TextureViewDimension::D3),
        aspect: TextureAspect::All,
        base_mip_level: Default::default(),
        mip_level_count: Default::default(),
        base_array_layer: Default::default(),
        array_layer_count: Default::default(),
    };
    let texture_view = texture.create_view(&texture_view_desc);
    debug!("Texture view created");
    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: Default::default(),
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });
    debug!("Render pass created");

    // TODO: use actual shader constants
    let push_constants = ShaderConstants {};

    rpass.set_pipeline(&render_pipeline);
    debug!("Render pipeline set");
    rpass.set_push_constants(
        wgpu::ShaderStage::all(),
        0,
        bytemuck::bytes_of(&push_constants),
    );
    debug!("Shader constants pushed");
    rpass.draw(0..3, 0..1);
    debug!("Draw called");

    // Start getting the results from the draw
    // Heavily based on https://github.com/gfx-rs/wgpu/blob/v0.9/wgpu/examples/capture/main.rs
    // TODO: simplify where possible

    let buffer_dimensions = BufferDimensions::new(width as usize, height as usize);
    // The output buffer lets us retrieve the data as an array
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height) as u64,
        usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });

    let texture_extent = wgpu::Extent3d {
        width: buffer_dimensions.width as u32,
        height: buffer_dimensions.height as u32,
        depth_or_array_layers: 1,
    };

    // The render pipeline renders data into this texture
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb, // TODO: rgbaf32 probably?
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::COPY_SRC,
        label: None,
    });

    // Drop the render pass to prevent double mutable borrow on encoder
    drop(rpass);

    // Copy the texture into a buffer, as they are separate concepts in wgpu
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(
                    std::num::NonZeroU32::new(buffer_dimensions.padded_bytes_per_row as u32)
                        .unwrap(),
                ),
                rows_per_image: None,
            },
        },
        texture_extent,
    );

    // Note that we're not calling `.await` here.
    let buffer_slice = output_buffer.slice(..);
    let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    let mut pixelbuffer: Vec<Color> = vec![];

    if let Ok(()) = buffer_future.await {
        debug!("Writing the pixelbuffer");
        let padded_buffer = buffer_slice.get_mapped_range();
        // from the padded_buffer we write just the unpadded bytes into the image
        for chunk in padded_buffer.chunks(buffer_dimensions.padded_bytes_per_row) {
            let row = &chunk[..buffer_dimensions.unpadded_bytes_per_row];
            // currently rgba8, we care about rgb only
            // TODO: use a f32 format instead so no need for conversions
            for pixel in row.chunks(4) {
                let r = u8_to_float(pixel[0]);
                let g = u8_to_float(pixel[1]);
                let b = u8_to_float(pixel[2]);
                let _a = u8_to_float(pixel[3]);
                let color = Color::new(r, g, b);
                pixelbuffer.push(color);
            }
        }
    }

    // Drop the GPU instance
    drop(instance);

    // TODO: placeholder return
    // let pixels = (width * height) as u64;
    // let black = Color::new(0.0, 0.0, 0.0);
    // let pixelbuffer: Vec<Color> = vec![black; pixels as usize];
    debug!("Returning pixelbuffer");
    pixelbuffer
}

fn u8_to_float(byte: u8) -> Float {
    // byte is 0-255
    // make it into a float 0.0-1.0
    let float = byte as Float / 255.0;
    float
}

// TODO: adapted from https://github.com/mitchmindtree/nannou-rustgpu-raytracer
// TODO: figure out if needed / could be simplified / etc
// TODO: compile shaders at build time, not at run time
fn load_shader_module_desc() -> wgpu::ShaderModuleDescriptor<'static> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let crate_path = [manifest_dir, "..", "clovers-gpu", "shaders"]
        .iter()
        .copied()
        .collect::<PathBuf>();
    let compile_result = SpirvBuilder::new(crate_path, "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::None)
        // Seems to be needed to handle conditions within functions?
        // Error was confusing but adding this worked.
        .capability(Capability::Int8)
        .build()
        .unwrap();
    let module_path = compile_result.module.unwrap_single();
    let data = std::fs::read(module_path).unwrap();
    let spirv = wgpu::util::make_spirv(&data);
    let spirv = match spirv {
        wgpu::ShaderSource::Wgsl(cow) => wgpu::ShaderSource::Wgsl(Cow::Owned(cow.into_owned())),
        wgpu::ShaderSource::SpirV(cow) => wgpu::ShaderSource::SpirV(Cow::Owned(cow.into_owned())),
    };
    wgpu::ShaderModuleDescriptor {
        label: Some("clovers-shader"),
        source: spirv,
        flags: wgpu::ShaderFlags::default(),
    }
}

// TODO: adapted from https://github.com/gfx-rs/wgpu/blob/v0.9/wgpu/examples/capture/main.rs
// TODO: figure out if needed etc
struct BufferDimensions {
    width: usize,
    height: usize,
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

impl BufferDimensions {
    fn new(width: usize, height: usize) -> Self {
        let bytes_per_pixel = size_of::<u32>();
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}
