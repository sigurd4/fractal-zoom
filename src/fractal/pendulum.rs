use num_complex::Complex;
use num_traits::Zero;
use winit::dpi::PhysicalSize;

use crate::{MyFloat, app::InitView, fractal::Fractal};

use super::wgsl_bindgen::pendulum;

#[derive(Clone, Copy)]
pub struct Pendulum;

impl<F> Fractal<F> for Pendulum
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "pendulum"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        InitView {
            exp: Complex::zero(),
            shift: Complex::zero(),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = pendulum::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = pendulum::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = pendulum::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: pendulum::vertex_state(&shader, &vertex_entry),
            fragment: Some(pendulum::fragment_state(&shader, &pendulum::fs_main_entry([
                Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::COLOR,
                })
            ]))),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None
            // ... other pipeline state
        })
    }
}