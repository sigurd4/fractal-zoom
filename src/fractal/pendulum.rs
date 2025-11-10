use num_complex::Complex;

use crate::{MyFloat, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::pendulum;

#[derive(Clone, Copy)]
pub struct Pendulum;

impl Fractal for Pendulum
{
    const LABEL: &str = "pendulum";

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = pendulum::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = pendulum::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = pendulum::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Self::LABEL),
            layout: Some(&pipeline_layout),
            vertex: pendulum::vertex_state(&shader, &vertex_entry),
            fragment: Some(pendulum::fragment_state(&shader, &pendulum::fs_main_entry([
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

    fn f<F>(&self, c: Complex<F>, z: Option<Complex<F>>, phi: Complex<F>) -> Complex<F>
    where
        F: MyFloat
    {
        let exp = phi.tan();
        z.unwrap_or(c).powc(exp) + c
    }

    fn f_newton<'a, F>(&'a self, c: Complex<F>, z: Option<Complex<F>>, phi: Complex<F>) -> impl Iterator<Item = Complex<F>> + 'a
    where
        F: MyFloat + 'a
    {
        let (mut c, z) = ([c, dcdz(z)].into_iter(), z.unwrap_or(c));

        let mut exp = phi.tan();
        let mut fmc = z.powc(exp);

        core::iter::repeat_with(move || {
            let mut f = fmc;
            if let Some(c) = c.next()
            {
                f += c
            }
            fmc *= exp/z;
            exp -= F::one();
            f
        })
    }
}