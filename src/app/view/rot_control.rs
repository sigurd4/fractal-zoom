use core::ops::Deref;
use std::time::SystemTime;

use num_complex::Complex;
use num_traits::Zero;
use winit::event::ElementState;

use crate::{MyFloat, app::{CoordControl, RotateDirection}, f};

#[derive(Debug, Clone, Copy)]
pub struct RotControl<F>
where
    F: MyFloat
{
    t: SystemTime,
    pos: F,
    vel: F,
    acc: F,
    mov: Option<bool>,
}

impl<F> From<F> for RotControl<F>
where
    F: MyFloat
{
    fn from(pos: F) -> Self
    {
        Self {
            pos: pos % F::TAU(),
            ..Default::default()
        }
    }
}
impl<F> Deref for RotControl<F>
where
    F: MyFloat
{
    type Target = F;

    fn deref(&self) -> &Self::Target
    {
        &self.pos
    }
}
impl<F> Default for RotControl<F>
where
    F: MyFloat
{
    fn default() -> Self
    {
        Self {
            t: SystemTime::now(),
            pos: Zero::zero(),
            vel: Zero::zero(),
            acc: Zero::zero(),
            mov: None
        }
    }
}

impl<F> RotControl<F>
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

    pub fn update(&mut self, speed: F, reverse: bool, center: Complex<F>, win_center: &mut Complex<F>, zoom: F) -> anyhow::Result<()>
    {
        let t0 = core::mem::replace(&mut self.t, SystemTime::now());
        let dt = f!(self.t.duration_since(t0)?.as_secs_f64());
        self.vel += core::mem::replace(&mut self.acc, F::zero())*dt;
        let pos0 = self.pos;
        match reverse
        {
            false => self.pos += self.vel*dt,
            true => self.pos -= self.vel*dt,
        }
        if let Some(dir) = self.mov
        {
            match dir ^ reverse
            {
                true => self.pos += speed*dt,
                false => self.pos -= speed*dt
            }
        }
        *win_center = *win_center*Complex::cis(pos0 - self.pos);

        Ok(())
    }

    pub fn rot(&mut self, direction: RotateDirection, button_state: ElementState)
    {
        self.mov = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn push(&mut self, push: Option<(RotateDirection, F)>)
    {
        match push
        {
            Some((direction, accel)) => match direction
            {
                RotateDirection::Left => self.acc -= accel,
                RotateDirection::Right => self.acc += accel
            },
            None => self.stop()
        }
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