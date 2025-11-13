use core::f64::consts::{FRAC_PI_2, PI};

use num_complex::Complex;
use num_traits::Float;
use winit::dpi::PhysicalSize;

use crate::{MyFloat, app::InitView, f, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::cantor;

#[derive(Clone, Copy)]
pub struct Cantor;

impl Fractal for Cantor
{
    const LABEL: &str = "cantor";

    fn init_view<F>(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    where
        F: MyFloat
    {
        let wrapping = |x| Float::tan(x*F::PI() - F::FRAC_PI_2());
        let gamma = f!(1.0/3.0);
        let phi = f!(0.5);
        InitView {
            exp: Complex::new(wrapping(phi), wrapping(phi)),
            shift: Complex::new(wrapping(gamma), wrapping(gamma)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = cantor::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = cantor::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = cantor::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Self::LABEL),
            layout: Some(&pipeline_layout),
            vertex: cantor::vertex_state(&shader, &vertex_entry),
            fragment: Some(cantor::fragment_state(&shader, &cantor::fs_main_entry([
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