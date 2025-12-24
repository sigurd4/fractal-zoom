use core::{cmp::Ordering, ops::Deref};
use std::time::SystemTime;

use num_complex::Complex;
use num_traits::{Float, One, Zero};
use winit::event::ElementState;

use crate::{MyFloat, ZOOM_BASE, ZOOM_MUL, ZOOM_RANGE, app::{CoordControl, RotateDirection, ZoomDirection}, f};

#[derive(Debug, Clone, Copy)]
pub struct ZoomControl<F>
where
    F: MyFloat
{
    t: SystemTime,
    pos: F,
    vel: F,
    acc: F,
    brk: F,
    mov: Option<bool>
}

impl<F> From<F> for ZoomControl<F>
where
    F: MyFloat
{
    fn from(pos: F) -> Self
    {
        Self {
            pos,
            ..Default::default()
        }
    }
}
impl<F> Deref for ZoomControl<F>
where
    F: MyFloat
{
    type Target = F;

    fn deref(&self) -> &Self::Target
    {
        &self.pos
    }
}
impl<F> Default for ZoomControl<F>
where
    F: MyFloat
{
    fn default() -> Self
    {
        Self {
            t: SystemTime::now(),
            pos: One::one(),
            vel: One::one(),
            acc: Zero::zero(),
            brk: Zero::zero(),
            mov: None
        }
    }
}

impl<F> ZoomControl<F>
where
    F: MyFloat
{
    pub fn new(pos: F, vel: F) -> Self
    {
        Self {
            pos,
            vel,
            ..Default::default()
        }
    }

    pub fn mov(&mut self, direction: ZoomDirection, button_state: ElementState)
    {
        self.mov = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn update(&mut self, speed: F, reverse: bool, center: &mut CoordControl<F>, win_center: Complex<F>, rot: F) -> anyhow::Result<()>
    {
        let t0 = core::mem::replace(&mut self.t, SystemTime::now());
        let dt = f!(self.t.duration_since(t0)?.as_secs_f64());

        let acc = core::mem::replace(&mut self.acc, F::zero());
        let vel_mul = Float::exp(f!(ZOOM_BASE)*acc*dt);
        self.vel *= vel_mul;
        let brk = core::mem::replace(&mut self.brk, F::zero());
        let hyp = Float::ln(Float::abs(self.vel));
        let sgn = Float::signum(hyp);
        let kat = hyp*hyp - f!(ZOOM_BASE)*dt*brk*Float::abs(brk);
        if Float::is_sign_negative(kat)
        {
            self.vel = F::one()
        }
        else
        {
            self.vel = Float::exp(sgn*Float::sqrt(kat));
        }

        let mut vel = Float::powf(self.vel, dt);
        if let Some(dir) = self.mov
        {
            match dir
            {
                false => vel *= speed,
                true => vel /= speed
            }
        }
        let zoom_mul = Float::powf(vel, dt);
        let new_zoom = match reverse
        {
            true => self.pos/zoom_mul,
            false => self.pos*zoom_mul
        };
        center.nudge((win_center/new_zoom - win_center/self.pos)*Complex::cis(rot));
        self.pos = new_zoom;
        let dir = (self.vel > F::one()) ^ reverse;
        if (self.pos > f!(ZOOM_RANGE.end) && dir)
            || (self.pos < f!(ZOOM_RANGE.start) && !dir)
        {
            self.vel = Float::recip(self.vel)
        }

        Ok(())
    }

    pub fn push(&mut self, push: Option<(ZoomDirection, F)>)
    {
        match push
        {
            Some((direction, accel)) => match direction
            {
                ZoomDirection::Inwards => self.acc += accel,
                ZoomDirection::Outwards => self.acc -= accel
            },
            None => self.stop()
        }
    }

    pub fn brk(&mut self, brk: F)
    {
        self.brk += brk;
    }

    pub fn stop(&mut self)
    {
        let Self { t, pos, mov, .. } = *self;
        *self = Self { t, pos, mov, ..Default::default() };
    }
    pub fn reset(&mut self)
    {
        let Self { t, mov, .. } = *self;
        *self = Self { t, mov, ..Default::default() };
    }
}