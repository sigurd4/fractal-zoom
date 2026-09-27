use std::f64::consts::FRAC_1_SQRT_2;

use num_complex::{Complex, ComplexFloat};
use num_traits::Zero;
use winit::dpi::PhysicalSize;

use crate::{f, MyFloat, app::InitView, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::weierstrass;

#[derive(Clone, Copy)]
pub struct Weierstrass
{
    a: Complex<f64>,
    b: Complex<f64>
}

impl Default for Weierstrass
{
    fn default() -> Self {
        Self {
            a: Complex::new(FRAC_1_SQRT_2, 0.0),
            b: Complex::new(2.0, 0.0)
        }
    }
}

impl<F> Fractal<F> for Weierstrass
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "weierstrass"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        InitView {
            shift: Complex::new(f!(self.a.re), f!(self.a.im)),
            exp: Complex::new(f!(self.b.re), f!(self.b.im)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = weierstrass::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = weierstrass::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = weierstrass::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: weierstrass::vertex_state(&shader, &vertex_entry),
            fragment: Some(weierstrass::fragment_state(&shader, &weierstrass::fs_main_entry([
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
