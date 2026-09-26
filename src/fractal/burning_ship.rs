use num_complex::Complex;
use num_traits::{Float, Zero};
use winit::dpi::PhysicalSize;

use crate::{f, MyFloat, app::InitView, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::burning_ship;

#[derive(Clone, Copy)]
pub struct BurningShip;

impl<F> Fractal<F> for BurningShip
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "burning_ship"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        InitView {
            exp: Complex::new(f!(2.0), F::zero()),
            shift: Complex::from(f!(1.0)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = burning_ship::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = burning_ship::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = burning_ship::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: burning_ship::vertex_state(&shader, &vertex_entry),
            fragment: Some(burning_ship::fragment_state(&shader, &burning_ship::fs_main_entry([
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