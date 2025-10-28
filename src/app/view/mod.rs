use num_complex::Complex;
use num_traits::Float;
use rand::distr::{Distribution, Uniform};
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{MAX_ITERATIONS, MyFloat, START_ZOOM, f, fractal::GlobalUniforms};

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
    phi: Complex<F>
}

impl<F> View<F>
where
    F: MyFloat
{
    pub fn new(win_size: PhysicalSize<u32>) -> Self
    {
        Self {
            mouse_pos: None,
            win_center: Complex { re: f!(0.0), im: f!(0.0) },
            win_size,
            zoom: f!(START_ZOOM),
            center: Complex::new(Uniform::new(f!(1.5), f!(2)).unwrap().sample(&mut rand::rng()), F::zero()),
            rot: F::zero(),
            phi: Complex { re: Float::atan(f!(2.0)), im: Float::atan(f!(0.0)) }
        }
    }

    pub fn uniforms(&self) -> GlobalUniforms
    {
        GlobalUniforms {
            max_iterations: MAX_ITERATIONS,
            _pad_max_iterations: [0; _],
            window_size: glam::uvec2(self.win_size.width, self.win_size.height),
            center: glam::vec2(self.center.re.to_f32().unwrap(), self.center.im.to_f32().unwrap()),
            zoom: self.zoom.to_f32().unwrap(),
            rot: self.rot.to_f32().unwrap(),
            exp: glam::vec2(Float::tan(self.phi.re).to_f32().unwrap(), Float::tan(self.phi.im).to_f32().unwrap())
        }
    }

    pub fn update(&mut self, control: ViewControl<F>)
    {
        control.update_view(self);
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
            println!("Mouse pos: x = {}, y = {}", mouse_pos.re, mouse_pos.im);
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

    pub fn reset(&mut self)
    {
        *self = View::new(self.win_size)
    }
    
    pub fn win_size(&self) -> PhysicalSize<u32>
    {
        self.win_size
    }
}