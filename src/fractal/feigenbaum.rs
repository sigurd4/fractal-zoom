use num_complex::Complex;
use num_traits::Zero;
use winit::dpi::PhysicalSize;

use crate::{MyFloat, app::InitView, f, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::feigenbaum;

#[derive(Clone, Copy)]
pub struct Feigenbaum;

impl Fractal for Feigenbaum
{
    const LABEL: &str = "feigenbaum";

    fn init_view<F>(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    where
        F: MyFloat
    {
        InitView {
            exp: Complex::new(f!(2.0), Zero::zero()),
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
            label: Some(Self::LABEL),
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