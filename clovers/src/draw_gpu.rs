#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
// spirv errors
#![deny(warnings)]
// TODO: temporary during development
#![allow(dead_code)]

use std::{borrow::Cow, path::PathBuf};

use clovers::{color::Color, scenes::Scene, Float};
use spirv_std::glam::{vec4, Vec4};

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

#[spirv(fragment)]
pub fn main_fs(output: &mut Vec4) {
    *output = vec4(1.0, 0.0, 0.0, 1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    *out_pos = vec4(
        (vert_id - 1) as f32,
        ((vert_id & 1) * 2 - 1) as f32,
        0.0,
        1.0,
    );
}

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
mod shaders {
    #[allow(non_upper_case_globals)]
    pub const main_fs: &str = "main_fs";
    #[allow(non_upper_case_globals)]
    pub const main_vs: &str = "main_vs";
}

// TODO: borrowed from rust-gpu
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

use bytemuck::{Pod, Zeroable};
#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub max_depth: u32,
    pub time: f32,
}

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub async fn draw(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
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
    let (device, queue) = adapter
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
    let swapchain_format = wgpu::TextureFormat::Rgba32Float;

    // TODO: this build step seems fairly messy, clean up?
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let crate_path = [manifest_dir, "src"].iter().copied().collect::<PathBuf>();
    let builder = SpirvBuilder::new(crate_path, "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::None);

    use spirv_builder::{CompileResult, MetadataPrintout, SpirvBuilder};
    let initial_result = builder.build().unwrap();

    fn handle_compile_result(
        compile_result: CompileResult,
    ) -> wgpu::ShaderModuleDescriptor<'static> {
        let module_path = compile_result.module.unwrap_single();
        let data = std::fs::read(module_path).unwrap();
        let spirv = wgpu::util::make_spirv(&data);
        let spirv = match spirv {
            wgpu::ShaderSource::Wgsl(cow) => wgpu::ShaderSource::Wgsl(Cow::Owned(cow.into_owned())),
            wgpu::ShaderSource::SpirV(cow) => {
                wgpu::ShaderSource::SpirV(Cow::Owned(cow.into_owned()))
            }
        };
        wgpu::ShaderModuleDescriptor {
            label: None,
            source: spirv,
            flags: wgpu::ShaderFlags::default(),
        }
    }
    let shader_binary = handle_compile_result(initial_result);

    // TODO: what do we need for actually running the shader?
    let mut render_pipeline =
        create_pipeline(&device, &pipeline_layout, swapchain_format, shader_binary);
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: width,
        height: height,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    // TODO: i do not have a window and don't want one here
    let mut surface = unsafe { instance.create_surface(&window) };
    let swap_chain = device.create_swap_chain(&surface, &sc_desc);
    let frame = swap_chain
        .get_current_frame()
        .expect("Failed to acquire next swap chain texture")
        .output;
    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view: &frame.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });

    // TODO: placeholder constants
    let time = 0.0;
    let push_constants = ShaderConstants {
        width,
        height,
        samples,
        max_depth,
        time,
    };

    rpass.set_pipeline(&render_pipeline);
    rpass.set_push_constants(
        wgpu::ShaderStage::all(),
        0,
        bytemuck::bytes_of(&push_constants),
    );
    rpass.draw(0..3, 0..1);
    // TODO: return the actual results

    // TODO: placeholder return
    let pixels = (width * height) as u64;
    let black = Color::new(0.0, 0.0, 0.0);
    let pixelbuffer: Vec<Color> = vec![black; pixels as usize];
    pixelbuffer
}
