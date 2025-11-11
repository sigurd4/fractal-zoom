const RECURSION_LIMIT: usize = 256;

moddef::moddef!(
    mod {
        wgsl_bindgen
    },
    flat(pub) mod {
        blancmange,
        cantor,
        feigenbaum,
        julia,
        mandelbrot,
        pendulum
    }
);

use core::ops::Range;

use num_complex::Complex;
use num_complex::ComplexFloat;
use num_traits::One;
use num_traits::Zero;
use wgsl_bindgen::global_bindings;

pub use global_bindings::GlobalUniforms;
pub use global_bindings::WgpuBindGroup0;
pub use global_bindings::WgpuBindGroup0Entries;
pub use global_bindings::WgpuBindGroup0EntriesParams;
pub use global_bindings::VertexInput;

use crate::{f, MyFloat, NEWTON_N, NEWTON_MU};

pub trait Fractal
{
    const LABEL: &str;

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