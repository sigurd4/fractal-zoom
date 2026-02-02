use num_complex::Complex;
use num_traits::{Float, Zero};
use winit::dpi::PhysicalSize;

use crate::{f, MyFloat, app::InitView, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::mandelbrot;

#[derive(Clone, Copy)]
pub struct Mandelbrot;

impl<F> Fractal<F> for Mandelbrot
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "mandelbrot"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        InitView {
            exp: Complex::new(f!(2.0), F::zero()),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = mandelbrot::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = mandelbrot::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = mandelbrot::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: mandelbrot::vertex_state(&shader, &vertex_entry),
            fragment: Some(mandelbrot::fragment_state(&shader, &mandelbrot::fs_main_entry([
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