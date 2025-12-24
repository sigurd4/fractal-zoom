const RECURSION_LIMIT: usize = 256;

moddef::moddef!(
    mod {
        wgsl_bindgen
    },
    flat(pub) mod {
        blancmange,
        cantor,
        feigenbaum,
        fibonacci_hamiltonian,
        julia,
        mandelbrot,
        pendulum
    }
);

use num_complex::Complex;
use num_traits::One;
use num_traits::Zero;
use wgsl_bindgen::global_bindings;

pub use global_bindings::GlobalUniforms;
pub use global_bindings::WgpuBindGroup0;
pub use global_bindings::WgpuBindGroup0Entries;
pub use global_bindings::WgpuBindGroup0EntriesParams;
pub use global_bindings::VertexInput;
use winit::dpi::PhysicalSize;

use crate::app::InitView;
use crate::MyFloat;

pub trait Fractal
{
    const LABEL: &str;

    fn init_view<F>(&self, zoom: F, win_size: PhysicalSize<u32>) -> InitView<F>
    where
        F: MyFloat;

    fn setup_render_pipeline(
        &self,
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat
    ) -> wgpu::RenderPipeline;
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