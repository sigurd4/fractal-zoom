use num_complex::{Complex, ComplexFloat};
use num_traits::Zero;

use crate::{MyFloat, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::julia;

#[derive(Clone, Copy)]
pub struct Julia;

impl Fractal for Julia
{
    const LABEL: &str = "julia";

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

    fn f<F>(&self, c: Complex<F>, z: Option<Complex<F>>, phi: Complex<F>) -> Complex<F>
    where
        F: MyFloat
    {
        let exp = phi.tan();
        (z.unwrap_or_else(Complex::zero) - c).powc(exp.recip())
    }

    fn f_newton<'a, F>(&'a self, c: Complex<F>, z: Option<Complex<F>>, phi: Complex<F>) -> impl Iterator<Item = Complex<F>> + 'a
    where
        F: MyFloat + 'a
    {
        let dcdz: Complex<F> = dcdz(z);
        let z = z.unwrap_or_else(Complex::zero);

        let mut exp = phi.tan().recip();
        let zc = z - c;
        let dzcdz = -dcdz + F::one();
        let d = dzcdz/zc;
        let mut a = zc.powc(exp);

        core::iter::repeat_with(move || {
            let f = a;
            exp -= F::one();
            a *= d*exp;
            f
        })
    }
}