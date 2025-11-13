use core::ops::{Deref, DerefMut};
use std::time::SystemTime;

use num_complex::Complex;
use num_traits::{Float, Zero};
use winit::event::ElementState;

use crate::{MyFloat, app::{MoveDirection, RotateDirection}, f};

#[derive(Debug, Clone, Copy)]
pub struct CoordControl<F>
where
    F: MyFloat
{
    t: SystemTime,
    pos: Complex<F>,
    vel: Complex<F>,
    acc: Complex<F>,
    mov: [Option<bool>; 2],
    rot: Option<bool>
}

impl<F> From<Complex<F>> for CoordControl<F>
where
    F: MyFloat
{
    fn from(pos: Complex<F>) -> Self
    {
        Self {
            pos,
            ..Default::default()
        }
    }
}
impl<F> Deref for CoordControl<F>
where
    F: MyFloat
{
    type Target = Complex<F>;

    fn deref(&self) -> &Self::Target
    {
        &self.pos
    }
}
impl<F> DerefMut for CoordControl<F>
where
    F: MyFloat
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.pos
    }
}
impl<F> Default for CoordControl<F>
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
            mov: [None; 2],
            rot: None
        }
    }
}

impl<F> CoordControl<F>
where
    F: MyFloat
{
    pub fn new(pos: Complex<F>, vel: Complex<F>) -> Self
    {
        Self {
            pos,
            vel,
            ..Default::default()
        }
    }

    pub fn update(&mut self, speed: Complex<F>, rot_speed: F, reverse: bool) -> anyhow::Result<()>
    {
        fn rot270<F>(z: Complex<F>) -> Complex<F>
        where
            F: Float
        {
            let Complex { re, im } = z;
            Complex { re: im, im: -re }
        }

        fn ident<T>(z: T) -> T
        {
            z
        }

        let t0 = core::mem::replace(&mut self.t, SystemTime::now());
        let dt = f!(self.t.duration_since(t0)?.as_secs_f64());
        self.vel += core::mem::replace(&mut self.acc, Complex::zero())*dt;
        match reverse
        {
            false => self.pos += self.vel*dt,
            true => self.pos -= self.vel*dt,
        }

        for (dir, phase) in self.mov.into_iter()
            .zip([ident, rot270] as [fn(Complex<_>) -> Complex<_>; _])
            .filter_map(|(mov, phase)| mov.map(|dir| (dir, phase)))
        {
            let vel = phase(speed);
            match dir
            {
                true => self.pos -= vel*dt,
                false => self.pos += vel*dt
            }
        }

        if let Some(rot) = self.rot
        {
            self.pos *= Complex::cis(match rot
            {
                true => rot_speed,
                false => -rot_speed
            })
        }

        Ok(())
    }

    pub fn rot(&mut self, direction: RotateDirection, button_state: ElementState)
    {
        self.rot = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn mov(&mut self, direction: MoveDirection, button_state: ElementState)
    {
        self.mov[direction.axis() as usize] = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn nudge(&mut self, nudge: Complex<F>)
    {
        self.pos += nudge
    }

    pub fn push(&mut self, push: Option<(MoveDirection, F)>)
    {
        match push
        {
            Some((direction, accel)) => {
                let dst = match direction.axis()
                {
                    false => &mut self.acc.re,
                    true => &mut self.acc.im
                };
                match direction.forward()
                {
                    false => *dst -= accel,
                    true => *dst += accel
                }
            },
            None => self.stop()
        }
    }

    pub fn stop(&mut self)
    {
        let Self { t, pos, mov, rot, .. } = *self;
        *self = Self { t, pos, mov, rot, ..Default::default() };
    }
    pub fn reset(&mut self)
    {
        let Self { t, mov, rot, .. } = *self;
        *self = Self { t, mov, rot, ..Default::default() };
    }
}