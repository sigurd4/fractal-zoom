use core::fmt::Display;

use num_complex::Complex;
use num_traits::{Float, NumAssignOps};
use rand::{Rng, distr::{Distribution, Uniform, uniform::SampleUniform}};
use winit::dpi::PhysicalSize;

use crate::{START_ZOOM, f, fractal::GlobalUniforms};

moddef::moddef!(
    flat(pub) mod {
        view_control,
        move_direction,
        rotate_direction,
    }
);

#[derive(Debug, Clone, Copy)]
pub struct View<F>
where
    F: Float
{
    center: Complex<F>,
    zoom: F,
    rot: F,
    exp: Complex<F>
}

impl<F> Default for View<F>
where
    F: Float + SampleUniform + Display
{
    fn default() -> Self
    {
        Self::new(&mut rand::rng())
    }
}

impl<F> View<F>
where
    F: Float + Display
{
    fn new(rng: &mut impl Rng) -> Self
    where
        F: SampleUniform
    {
        Self {
            zoom: f!(START_ZOOM),
            center: Complex::new(Uniform::new(f!(1.5), f!(2)).unwrap().sample(rng), F::zero()),
            rot: F::zero(),
            exp: Complex { re: f!(2.0), im: f!(0.0) }
        }
    }

    pub fn uniforms(&self, size: PhysicalSize<u32>) -> GlobalUniforms
    {
        GlobalUniforms {
            window_size: glam::uvec2(size.width, size.height),
            center: glam::vec2(self.center.re.to_f32().unwrap(), self.center.im.to_f32().unwrap()),
            zoom: self.zoom.to_f32().unwrap(),
            rot: self.rot.to_f32().unwrap(),
            exp: glam::vec2(self.exp.re.to_f32().unwrap(), self.exp.im.to_f32().unwrap())
        }
    }

    pub fn update(&mut self, control: ViewControl)
    where
        F: NumAssignOps
    {
        control.update_view(self);
    }

    pub fn transform_3x3(&self) -> glam::Mat3A
    {
        let rot = Complex::from_polar(self.zoom.recip(), self.rot);
        let rot = Complex::new(
            rot.re.to_f32().unwrap(),
            rot.im.to_f32().unwrap()
        );
        let center = Complex::new(
            self.center.re.to_f32().unwrap(),
            self.center.im.to_f32().unwrap()
        );
        glam::mat3a(
            glam::vec3a(rot.re, -rot.im, -center.re),
            glam::vec3a(-rot.im, rot.re, -center.im),
            glam::vec3a(0.0, 0.0, 1.0)
        )
    }

    pub fn exp_vec2(&self) -> glam::Vec2
    {
        let exp = Complex::new(
            self.exp.re.to_f32().unwrap(),
            self.exp.im.to_f32().unwrap()
        );

        glam::vec2(exp.re, exp.im)
    }
}