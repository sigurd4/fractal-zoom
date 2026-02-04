use num_complex::{Complex, ComplexFloat};
use num_traits::Zero;
use winit::dpi::PhysicalSize;

use crate::{f, MyFloat, app::InitView, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::henon;

#[derive(Clone, Copy)]
pub struct Henon
{
    a: f64,
    b: f64
}

impl Default for Henon
{
    fn default() -> Self
    {
        Self {
            a: 1.4,
            b: 0.3
        }
    }
}

impl<F> Fractal<F> for Henon
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "henon"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        let Self { a, b } = self;
        InitView {
            shift: Complex::new(f!(*a), f!(*b)),
            exp: Complex::new(f!(2.0), f!(0.0)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = henon::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = henon::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = henon::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: henon::vertex_state(&shader, &vertex_entry),
            fragment: Some(henon::fragment_state(&shader, &henon::fs_main_entry([
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