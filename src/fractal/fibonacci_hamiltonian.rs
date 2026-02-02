use core::{f64::consts::{FRAC_PI_2, PI, SQRT_2}, ops::Range};

use num_complex::Complex;
use num_traits::{Float, One, Zero};
use winit::dpi::PhysicalSize;

use crate::{MyFloat, app::InitView, f, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::fibonacci_hamiltonian;

#[derive(Clone)]
pub struct FibonacciHamiltonian
{
    pub f: Complex<f64>,
    pub lambda: Complex<f64>
}

impl Default for FibonacciHamiltonian
{
    fn default() -> Self
    {
        let phi = (5.0f64.sqrt() + 1.0)/2.0;
        Self {
            f: Complex::from((1.0 + SQRT_2).ln()/phi.ln()),
            lambda: Complex::zero()
        }
    }
}

impl<F> Fractal<F> for FibonacciHamiltonian
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "fibonacci_hamiltonian"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        let Self { f, lambda } = self;

        InitView {
            exp: Complex::new(f!(lambda.re), f!(lambda.im)),
            shift: Complex::new(f!(f.re), f!(f.im)),
            ..Default::default()
        }
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        // Create shader module from generated code
        let shader = fibonacci_hamiltonian::create_shader_module_embed_source(device);
        
        // Use generated pipeline layout
        let pipeline_layout = fibonacci_hamiltonian::create_pipeline_layout(device);
        
        // Use generated vertex entry with proper buffer layout
        let vertex_entry = fibonacci_hamiltonian::vs_main_entry(wgpu::VertexStepMode::Vertex);
     
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(Fractal::<F>::label(self)),
            layout: Some(&pipeline_layout),
            vertex: fibonacci_hamiltonian::vertex_state(&shader, &vertex_entry),
            fragment: Some(fibonacci_hamiltonian::fragment_state(&shader, &fibonacci_hamiltonian::fs_main_entry([
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