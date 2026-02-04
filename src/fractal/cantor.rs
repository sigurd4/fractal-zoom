use core::{f64::consts::{FRAC_PI_2, PI}, ops::Range};

use num_complex::Complex;
use num_traits::{Float, One, Zero};
use winit::dpi::PhysicalSize;

use crate::{MyFloat, app::InitView, f, fractal::{Fractal, dcdz}};

use super::wgsl_bindgen::cantor;

/// K = 1 + nλ
/// r = φ₂ - φ₁
/// C := C/((k + (φ₂ + φ₁ - rᴷ)/2)rᴷⁿ, (k + (φ₂ + φ₁ + rᴷ)/2)rᴷⁿ)
/// rᴷⁿ_prev/rᴷⁿ = r^(1 + (2n - 1)λ)
#[derive(Clone)]
pub struct Cantor
{
    pub phi: Range<f64>,
    pub lambda: Complex<f64>
}

impl Cantor
{
    /// γ = 1/3
    /// C := C/((k + (1 - γ)/2)γⁿ, (k + (1 + γ)/2)γⁿ)
    pub fn cantor() -> Self
    {
        Self::symmetric(1.0/3.0)
    }
    pub fn sierpinski(self) -> Self
    {
        let Self { phi, lambda } = self;
        Self {
            phi: phi.end..phi.start,
            lambda
        }
    }
    /// C := C/((k + (1 - γ)/2)γⁿ, (k + (1 + γ)/2)γⁿ)
    pub fn symmetric(gamma: f64) -> Self
    {
        Self::assymetric(Self::symmetric_range(gamma))
    }
    /// γ = φ₂ - φ₁
    /// C := C/((k + φ₁)(φ₂ - φ₁)ⁿ, (k + φ₂)(φ₂ - φ₁)ⁿ)
    pub fn assymetric(gamma: Range<f64>) -> Self
    {
        Self {
            phi: gamma,
            lambda: Complex::zero()
        }
    }
    /// r = 1/4
    /// C := C/((k + (1 - rⁿ)/2)rⁿⁿ, (k + (1 + rⁿ)/2)rⁿⁿ)
    pub fn smith_volterra() -> Self
    {
        Self::fat(1.0/4.0)
    }
    /// C := C/((k + (1 - rⁿ)/2)rⁿⁿ, (k + (1 + rⁿ)/2)rⁿⁿ)
    pub fn fat(r: f64) -> Self
    {
        Self::fat_assymetric(Self::symmetric_range(r))
    }
    /// r = φ₂ - φ₁
    /// C := C/((k + (φ₂ + φ₁ - rⁿ)/2)rⁿⁿ, (k + (φ₂ + φ₁ + rⁿ)/2)rⁿⁿ)
    pub fn fat_assymetric(r: Range<f64>) -> Self
    {
        Self {
            phi: r,
            lambda: Complex::from(1.0)
        }
    }

    fn symmetric_range(mut phi: f64) -> Range<f64>
    {
        phi = phi % 1.0;
        (1.0 - phi)/2.0..(1.0 + phi)/2.0
    }
}

impl<F> Fractal<F> for Cantor
where
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        "cantor"
    }

    fn init_view(&self, _zoom: F, _win_size: PhysicalSize<u32>) -> InitView<F>
    {
        let Self { phi, lambda } = self;

        InitView {
            exp: Complex::new(f!(lambda.re), f!(lambda.im)),
            shift: Complex::new(f!(phi.start), f!(phi.end)),
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
            label: Some(Fractal::<F>::label(self)),
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
            multiview_mask: None,
            cache: None
            // ... other pipeline state
        })
    }
}