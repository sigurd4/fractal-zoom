use core::{f64::consts::{FRAC_PI_2, PI, SQRT_2}, ops::Range};

use num_complex::Complex;
use num_traits::{Float, One, Zero};
use winit::dpi::PhysicalSize;

use crate::{MyFloat, app::InitView, f, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::fibonacci_snowflake;

#[derive(Clone)]
pub struct FibonacciSnowlake;

impl Default for FibonacciSnowlake
{
    fn default() -> Self
    {
        Self
    }
}

impl<F> Fractal<F> for FibonacciSnowlake
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "fibonacci_snowflake"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        InitView {
            exp: Complex::new(Zero::zero(), One::one()),
            center: Complex::zero(),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = fibonacci_snowflake::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = fibonacci_snowflake::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = fibonacci_snowflake::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: fibonacci_snowflake::vertex_state(&shader, &vertex_entry),
            fragment: Some(fibonacci_snowflake::fragment_state(&shader, &fibonacci_snowflake::fs_main_entry([
                Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::COLOR,
                })
            ]))),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None
            // ... other pipeline state
        })
    }
}