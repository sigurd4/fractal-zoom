use num_complex::{Complex, ComplexFloat};
use num_traits::Zero;
use winit::dpi::PhysicalSize;

use crate::{f, MyFloat, app::InitView, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::ikeda;

#[derive(Clone, Copy)]
pub struct Ikeda
{
    a: f64,
    b: f64,
    k: f64,
    p: f64
}

impl Default for Ikeda
{
    fn default() -> Self
    {
        Self {
            a: 1.0,
            b: 0.9,
            k: 0.4,
            p: 6.0
        }
    }
}

impl<F> Fractal<F> for Ikeda
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "ikeda"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        let Self { a, b, k, p } = *self;
        InitView {
            shift: Complex::new(f!(a), f!(k)),
            exp: Complex::new(f!(b), f!(p)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = ikeda::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = ikeda::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = ikeda::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: ikeda::vertex_state(&shader, &vertex_entry),
            fragment: Some(ikeda::fragment_state(&shader, &ikeda::fs_main_entry([
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
