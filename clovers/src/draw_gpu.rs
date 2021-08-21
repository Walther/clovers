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
    // The usual usecase of code generation is always building in build.rs, and so the codegen
    // always happens. However, we want to both test code generation (on android) and runtime
    // compilation (on desktop), so manually fill in what would have been codegenned for desktop.
    #[allow(non_upper_case_globals)]
    pub const main_fs: &str = "main_fs";
    #[allow(non_upper_case_globals)]
    pub const main_vs: &str = "main_vs";
}

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

// use bytemuck::{Pod, Zeroable};
// #[derive(Copy, Clone, Pod, Zeroable)]
#[derive(Copy, Clone)]
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

    // TODO:

    let pixels = (width * height) as u64;
    let black = Color::new(0.0, 0.0, 0.0);
    let pixelbuffer: Vec<Color> = vec![black; pixels as usize];
    pixelbuffer
}
