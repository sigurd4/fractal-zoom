const RECURSION_LIMIT: usize = 256;

moddef::moddef!(
    mod {
        wgsl_bindgen
    },
    flat(pub) mod {
        blancmange,
        burning_ship,
        cantor,
        cesaro,
        feigenbaum,
        fibonacci_hamiltonian_julia,
        fibonacci_hamiltonian_mandelbrot,
        julia,
        henon,
        ikeda,
        roessler,
        koch_peano,
        mandelbrot,
        heighway_dragon,
        pendulum,
        rauzy,
        supergolden_julia,
        supergolden_mandelbrot,
        fibonacci_snowflake,
        weierstrass
    }
);

use core::ops::Deref;
use std::sync::Arc;

use num_complex::Complex;
use num_traits::One;
use num_traits::Zero;
use wgsl_bindgen::global_bindings;

pub use global_bindings::GlobalUniforms;
pub use global_bindings::VertexInput;
pub use global_bindings::WgpuBindGroup0;
pub use global_bindings::WgpuBindGroup0Entries;
pub use global_bindings::WgpuBindGroup0EntriesParams;
use winit::dpi::PhysicalSize;

use crate::MyFloat;
use crate::app::InitView;

pub trait Fractal<F>
where
    F: MyFloat
{
    fn label(&self) -> &'static str;

    fn init_view(&self, zoom: F, win_size: PhysicalSize<u32>) -> InitView<F>;

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline;
}

impl<F, T> Fractal<F> for Box<T>
where
    T: Fractal<F> + ?Sized,
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        self.deref().label()
    }

    fn init_view(&self, zoom: F, win_size: PhysicalSize<u32>) -> InitView<F>
    {
        self.deref().init_view(zoom, win_size)
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        self.deref().setup_render_pipeline(device, surface_format)
    }
}

impl<F, T> Fractal<F> for Arc<T>
where
    T: Fractal<F> + ?Sized,
    F: MyFloat
{
    fn label(&self) -> &'static str
    {
        self.deref().label()
    }

    fn init_view(&self, zoom: F, win_size: PhysicalSize<u32>) -> InitView<F>
    {
        self.deref().init_view(zoom, win_size)
    }

    fn setup_render_pipeline(&self, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline
    {
        self.deref().setup_render_pipeline(device, surface_format)
    }
}

fn dcdz<F, T>(z: Option<Complex<F>>) -> T
where
    F: MyFloat,
    T: One + Zero
{
    match z
    {
        Some(_) => T::zero(),
        None => T::one()
    }
}
