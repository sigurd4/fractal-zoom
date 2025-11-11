use num_complex::Complex;

use crate::{MyFloat, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::blancmange;

#[derive(Clone, Copy)]
pub struct Blancmange;

impl Fractal for Blancmange
{
    const LABEL: &str = "blancmange";

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = blancmange::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = blancmange::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = blancmange::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Self::LABEL),
            layout: Some(&pipeline_layout),
            vertex: blancmange::vertex_state(&shader, &vertex_entry),
            fragment: Some(blancmange::fragment_state(&shader, &blancmange::fs_main_entry([
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