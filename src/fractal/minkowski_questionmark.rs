use num_complex::Complex;
use num_traits::{Float, Zero};
use winit::dpi::PhysicalSize;

use crate::{f, MyFloat, app::InitView, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::minkowski_questionmark;

#[derive(Clone, Copy)]
pub struct MinkowskiQuestionmark;

impl<F> Fractal<F> for MinkowskiQuestionmark
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "minkowski_questionmark"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        InitView {
            exp: Complex::from(f!(2.0)),
            shift: Complex::from(f!(2.0)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = minkowski_questionmark::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = minkowski_questionmark::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = minkowski_questionmark::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: minkowski_questionmark::vertex_state(&shader, &vertex_entry),
            fragment: Some(minkowski_questionmark::fragment_state(&shader, &minkowski_questionmark::fs_main_entry([
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
