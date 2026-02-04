use num_complex::Complex;
use num_traits::{One, Zero};
use winit::dpi::PhysicalSize;

use crate::{MyFloat, app::InitView, f, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::blancmange;

#[derive(Clone, Copy)]
pub struct Blancmange
{
    w: Complex<f64>
}

impl Default for Blancmange
{
    fn default() -> Self
    {
        Self {
            w: Complex::new(0.5, 0.0)
        }
    }
}

impl<F> Fractal<F> for Blancmange
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "blancmange"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        let Self { w } = self;
        InitView {
            exp: Complex::new(f!(w.re), f!(w.im)),
            shift: Complex::one(),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = blancmange::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = blancmange::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = blancmange::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
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
            multiview_mask: None,
            cache: None
            // ... other pipeline state
        })
    }
}