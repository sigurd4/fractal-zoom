use nalgebra::ArrayStorage;
use num_complex::Complex;
use num_traits::{Float, NumAssignOps};
use rand::{Rng, distr::{Distribution, Uniform, uniform::SampleUniform}};

use crate::{START_ZOOM, f};

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
    F: Float + SampleUniform
{
    fn default() -> Self
    {
        Self::new(&mut rand::rng())
    }
}

impl<F> View<F>
where
    F: Float
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

    pub fn update(&mut self, control: ViewControl)
    where
        F: NumAssignOps
    {
        control.update_view(self);
    }

    pub fn transform_3x3(&self) -> nalgebra::Matrix3x2<f32>
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
        nalgebra::Matrix::from_array_storage(ArrayStorage([
            [rot.re, -rot.im, -center.re],
            [-rot.im, rot.re, -center.im]
        ]))
    }

    pub fn exp_vec2(&self) -> nalgebra::Vector2<f32>
    {
        let exp = Complex::new(
            self.exp.re.to_f32().unwrap(),
            self.exp.im.to_f32().unwrap()
        );
        nalgebra::Vector::from_array_storage(ArrayStorage([
            [exp.re, exp.im]
        ]))
    }
}