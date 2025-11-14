use num_complex::{Complex, ComplexFloat};
use num_traits::Zero;
use winit::dpi::PhysicalSize;

use crate::{f, MyFloat, app::InitView, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::julia;

#[derive(Clone, Copy)]
pub struct Julia;

pub enum JuliaVariant
{
    Dendrite,
    /// Unofficial name
    Clover
}

impl Fractal for Julia
{
    const LABEL: &str = "julia";

    fn init_view<F>(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    where
        F: MyFloat
    {
        let shift = match JuliaVariant::Dendrite
        {
            JuliaVariant::Dendrite => Complex::i(),
            JuliaVariant::Clover => f!(1.0/4.0).into()
        };
        InitView {
            shift,
            exp: Complex::new(f!(2.0), F::zero()),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = julia::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = julia::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = julia::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Self::LABEL),
            layout: Some(&pipeline_layout),
            vertex: julia::vertex_state(&shader, &vertex_entry),
            fragment: Some(julia::fragment_state(&shader, &julia::fs_main_entry([
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