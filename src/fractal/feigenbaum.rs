use num_complex::Complex;
use winit::dpi::PhysicalSize;

use crate::{MyFloat, app::InitView, f, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::feigenbaum;

/// z := rz(1 - z)
#[derive(Clone, Copy)]
pub struct Feigenbaum
{
    pub r: Complex<f64>
}

impl Default for Feigenbaum
{
    fn default() -> Self
    {
        Self {
            r: Complex::from(2.0)
        }
    }
}

impl<F> Fractal<F> for Feigenbaum
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "feigenbaum"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        let Self { r } = self;
        InitView {
            exp: Complex::new(f!(r.re), f!(r.im)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = feigenbaum::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = feigenbaum::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = feigenbaum::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: feigenbaum::vertex_state(&shader, &vertex_entry),
            fragment: Some(feigenbaum::fragment_state(&shader, &feigenbaum::fs_main_entry([
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