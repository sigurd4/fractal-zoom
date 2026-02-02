use std::time::SystemTime;

use num_complex::Complex;
use num_traits::{Float, Zero};
use winit::{dpi::{PhysicalPosition, PhysicalSize}, event::ElementState};

use crate::{EXP_ZOOM_VARIANCE, MAX_ITERATIONS, MOVE_CENTER_SPEED, MOVE_EXP_SPEED, MOVE_SHIFT_SPEED, MyFloat, ROT_SPEED, SHIFT_ZOOM_VARIANCE, START_ZOOM, ZOOM_MUL, f, fractal::{Fractal, GlobalUniforms}};

moddef::moddef!(
    flat(pub) mod {
        coord_control,
        rot_control,
        zoom_control,
        move_direction,
        rotate_direction,
        zoom_direction,
    }
);

#[derive(Debug, Clone, Copy)]
pub struct InitView<F>
where
    F: MyFloat
{
    pub win_center: Complex<F>,
    pub center: Complex<F>,
    pub shift: Complex<F>,
    pub exp: Complex<F>
}

impl<F> Default for InitView<F>
where
    F: MyFloat
{
    fn default() -> Self
    {
        Self {
            win_center: Complex::zero(),
            center: Complex::zero(),
            shift: Complex::zero(),
            exp: Complex::zero(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct View<F>
where
    F: MyFloat
{
    mouse_pos: Option<Complex<F>>,
    win_size: winit::dpi::PhysicalSize<u32>,
    win_center: Complex<F>,
    pub center: CoordControl<F>,
    pub shift: CoordControl<F>,
    pub exp: CoordControl<F>,
    pub zoom: ZoomControl<F>,
    pub rot: RotControl<F>,
    pub reverse: bool,
    t0: SystemTime
}

impl<F> View<F>
where
    F: MyFloat
{
    pub fn new<T>(fractal: &T, win_size: PhysicalSize<u32>) -> Self
    where
        T: Fractal<F>
    {
        let zoom = f!(START_ZOOM);
        let InitView { win_center, center, shift, exp } = fractal.init_view(zoom, win_size);
        Self {
            mouse_pos: None,
            win_center,
            win_size,
            center: CoordControl::from(center),
            shift: CoordControl::from(Complex::new(Float::atan(shift.re), Float::atan(shift.im))),
            exp: CoordControl::from(Complex::new(Float::atan(exp.re), Float::atan(exp.im))),
            zoom: ZoomControl::from(zoom),
            rot: RotControl::default(),
            reverse: false,
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
    
    pub fn reverse(&mut self, button_state: ElementState)
    {
        self.reverse = match button_state
        {
            ElementState::Pressed => !self.reverse,
            ElementState::Released => self.reverse
        }
    }

    pub fn reset<T>(&mut self, fractal: &T)
    where
        T: Fractal<F>
    {
        *self = View::new(fractal, self.win_size)
    }
    pub fn reset_time(&mut self)
    {
        self.t0 = SystemTime::now();
    }

    pub fn reset_view<T>(&mut self, fractal: &T)
    where
        T: Fractal<F>
    {
        let zoom = f!(START_ZOOM);
        let InitView { win_center, center, shift: _, exp: _ } = fractal.init_view(zoom, self.win_size);
        self.win_center = win_center;
        *self.center = center;
    }
    
    pub fn win_size(&self) -> PhysicalSize<u32>
    {
        self.win_size
    }

    pub fn update(&mut self) -> anyhow::Result<()>
    {
        self.center.update(Complex::from_polar(f!(MOVE_CENTER_SPEED)/(*self.zoom), *self.rot), f!(ROT_SPEED), self.reverse)?;
        self.shift.update(Complex::from(f!(MOVE_SHIFT_SPEED))*Float::powf(Float::recip(*self.zoom), f!(SHIFT_ZOOM_VARIANCE)), f!(ROT_SPEED), self.reverse)?;
        self.exp.update(Complex::from(f!(MOVE_EXP_SPEED))*Float::powf(Float::recip(*self.zoom), f!(EXP_ZOOM_VARIANCE)), f!(ROT_SPEED), self.reverse)?;
        self.zoom.update(f!(ZOOM_MUL), self.reverse, &mut self.center, self.win_center, *self.rot)?;
        self.rot.update(f!(ROT_SPEED), self.reverse, *self.center, &mut self.win_center, *self.zoom)?;
        Ok(())
    }
}