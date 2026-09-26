use core::f64::consts::SQRT_3;

use num_complex::{Complex, ComplexFloat};
use num_traits::Zero;
use winit::dpi::PhysicalSize;

use crate::{f, MyFloat, app::InitView, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::koch_peano;

#[derive(Clone, Copy)]
pub struct KochPeano
{
    a: Complex<f64>
}

impl KochPeano
{
    pub fn koch() -> Self
    {
        Self {
            a: Complex { re: 0.5, im: SQRT_3/6.0 }
        }
    }

    pub fn peano() -> Self
    {
        Self {
            a: Complex { re: 0.5, im: 0.5 }
        }
    }

    pub fn osgood(b: f64) -> Self
    {
        Self {
            a: Complex { re: 0.5, im: b*0.5 }
        }
    }
}

impl<F> Fractal<F> for KochPeano
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "koch_peano"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        InitView {
            shift: Complex::new(f!(1.0), f!(0.0)),
            exp: Complex::new(f!(self.a.re), f!(self.a.im)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = koch_peano::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = koch_peano::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = koch_peano::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: koch_peano::vertex_state(&shader, &vertex_entry),
            fragment: Some(koch_peano::fragment_state(&shader, &koch_peano::fs_main_entry([
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