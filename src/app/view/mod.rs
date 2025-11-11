use core::time::Duration;
use std::time::SystemTime;

use num_complex::Complex;
use num_traits::Float;
use rand::distr::{Distribution, Uniform};
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{DONUT, MAX_ITERATIONS, MyFloat, START_ZOOM, f, fractal::{self, Fractal, GlobalUniforms}};

moddef::moddef!(
    flat(pub) mod {
        view_control,
        move_direction,
        rotate_direction,
        zoom_direction,
    }
);

#[derive(Debug, Clone, Copy)]
pub struct View<F>
where
    F: MyFloat
{
    mouse_pos: Option<Complex<F>>,
    win_size: winit::dpi::PhysicalSize<u32>,
    win_center: Complex<F>,
    center: Complex<F>,
    zoom: F,
    rot: F,
    exp: Complex<F>,
    shift: Complex<F>,
    t0: SystemTime
}

impl<F> View<F>
where
    F: MyFloat
{
    pub fn new<T>(fractal: &T, win_size: PhysicalSize<u32>) -> Self
    where
        T: Fractal
    {
        let zoom = f!(START_ZOOM);
        let exp = Complex::new(Float::atan(f!(2.0)), Float::atan(f!(0.0)));
        let shift = Complex::new(Float::atan(f!(0.0)), Float::atan(f!(0.0)));
        let center = Complex::new(f!(0.0), f!(0.0));
        Self {
            mouse_pos: None,
            win_center: Complex { re: f!(0.0), im: f!(0.0) },
            win_size,
            zoom,
            center,
            rot: F::zero(),
            exp,
            shift,
            t0: SystemTime::now()
        }
    }

    pub fn uniforms(&self) -> GlobalUniforms
    {
        GlobalUniforms {
            time: SystemTime::now().duration_since(self.t0).unwrap().as_secs_f32(),
            _pad_time: [0; _],
            max_iterations: MAX_ITERATIONS,
            _pad_max_iterations: [0; _],
            window_size: glam::uvec2(self.win_size.width, self.win_size.height),
            center: glam::vec2(self.center.re.to_f32().unwrap(), self.center.im.to_f32().unwrap()),
            zoom: self.zoom.to_f32().unwrap(),
            rot: self.rot.to_f32().unwrap(),
            exp: glam::vec2(Float::tan(self.exp.re).to_f32().unwrap(), Float::tan(self.exp.im).to_f32().unwrap()),
            shift: glam::vec2(Float::tan(self.shift.re).to_f32().unwrap(), Float::tan(self.shift.im).to_f32().unwrap())
        }
    }
    
    pub fn update_mouse_pos(&mut self, mouse_pos: PhysicalPosition<f64>)
    {
        if self.win_size == PhysicalSize::new(0, 0)
        {
            self.mouse_pos = None
        }
        else
        {
            let mouse_pos = Complex::new(
                f!(mouse_pos.x - self.win_size.width as f64/2.0),
                f!(mouse_pos.y - self.win_size.height as f64/2.0)
            );
            self.mouse_pos = Some(mouse_pos);
            //println!("Mouse pos: x = {}, y = {}", mouse_pos.re, mouse_pos.im);
        }
    }

    pub fn recenter(&mut self)
    {
        if let Some(mouse_pos) = self.mouse_pos
        {
            self.win_center = mouse_pos
        }
    }

    pub fn resize(&mut self, win_size: PhysicalSize<u32>)
    {
        self.win_size = win_size
    }

    pub fn reset<T>(&mut self, fractal: &T)
    where
        T: Fractal
    {
        *self = View::new(fractal, self.win_size)
    }

    pub fn reset_time(&mut self)
    {
        self.t0 = SystemTime::now();
    }
    
    pub fn win_size(&self) -> PhysicalSize<u32>
    {
        self.win_size
    }
}