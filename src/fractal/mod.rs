const RECURSION_LIMIT: usize = 256;

moddef::moddef!(
    mod {
        wgsl_bindgen
    },
    flat(pub) mod {
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

    fn f<F>(&self, c: Complex<F>, z: Option<Complex<F>>, phi: Complex<F>) -> Complex<F>
    where
        F: MyFloat;

    fn f_newton<'a, F>(&'a self, c: Complex<F>, z: Option<Complex<F>>, phi: Complex<F>) -> impl Iterator<Item = Complex<F>> + 'a
    where
        F: MyFloat + 'a;

    fn dc_newton<F>(&self, c: Complex<F>, phi: Complex<F>) -> Option<Complex<F>>
    where
        F: MyFloat
    {
        let [f, dfdz, d2fdz2] = self.f_newton(c, None, phi)
            .next_chunk()
            .unwrap();

        let g = -dfdz/f + F::one();
        let dgdz = -(d2fdz2 - dfdz/f)/f;
        let dc = g/dgdz;
        //let dc = (dfdz - f)/(d2fdz2 - dfdz/f + F::one());
        //let dc = d2fdz2/dfdz;
        //let dc = (d2fdz2/dfdz - f.recip()).recip();
        match dc.is_finite()
        {
            true => Some(dc),
            false => None
        }
    }

    fn c_initial<F>(&self, r: Range<F>, phi: Complex<F>) -> Complex<F>
    where
        F: MyFloat
    {
        let mu = f!(NEWTON_MU);
        let mut c = crate::random_donut(r);
        let mut n = 0;
        while n < NEWTON_N && let Some(dz) = self.dc_newton(c, phi)
        {
            c = dz*mu;
            n += 1;
        }
        c
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