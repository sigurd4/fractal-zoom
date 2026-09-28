use core::{
    f64::consts::{FRAC_PI_2, PI, SQRT_2},
    ops::Range
};

use num_complex::Complex;
use num_traits::{Float, One, Zero};
use winit::dpi::PhysicalSize;

use crate::{
    MyFloat,
    app::InitView,
    f,
    fractal::{Fractal, dcdz}
};

use super::wgsl_bindgen::heighway_dragon;

#[derive(Clone)]
pub struct HeighwayDragon
{
    a: Complex<f64>,
    c: Complex<f64>
}

impl Default for HeighwayDragon
{
    fn default() -> Self
    {
        Self {
            a: Complex { re: 0.5, im: 0.5 },
            c: Complex { re: 1.0, im: 0.0 }
        }
    }
}

impl<F> Fractal<F> for HeighwayDragon
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "heighway_dragon"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        InitView {
            shift: Complex::new(f!(self.c.re), f!(self.c.im)),
            exp: Complex::new(f!(self.a.re), f!(self.a.im)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = heighway_dragon::create_shader_module_embed_source(device);

        // Use generated pipeline layout
        let pipeline_layout = heighway_dragon::create_pipeline_layout(device);

        // Use generated vertex entry with proper buffer layout
        let vertex_entry = heighway_dragon::vs_main_entry(wgpu::VertexStepMode::Vertex);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: heighway_dragon::vertex_state(&shader, &vertex_entry),
            fragment: Some(heighway_dragon::fragment_state(
                &shader,
                &heighway_dragon::fs_main_entry([Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::COLOR
                })])
            )),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None // ... other pipeline state
        })
    }
}
