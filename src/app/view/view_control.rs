use core::fmt::Display;

use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumAssignOps, Zero};
use winit::event::{ElementState, TouchPhase};

use crate::{MOVE_CENTER_ACCEL, MOVE_CENTER_SPEED, MOVE_EXP_ACCEL, MOVE_EXP_SPEED, MOVE_SHIFT_ACCEL, MOVE_SHIFT_SPEED, MOVE_ZOOM_ACCEL, MyFloat, NEWTON_MU, ROT_ACCEL, ROT_SPEED, ZOOM_MU, ZOOM_MUL, ZOOM_RANGE, app::{ZoomDirection, view::View}, clamp_rem, f, fractal::Fractal};

use super::{MoveDirection, RotateDirection};

#[derive(Debug, Clone, Copy)]
pub struct ViewControl<F>
where
    F: MyFloat
{
    center_vel: Complex<F>,
    shift_vel: Complex<F>,
    exp_vel: Complex<F>,
    zoom_vel: F,
    rot_vel: F,

    center_move: [Option<bool>; 2],
    shift_move: [Option<bool>; 2],
    exp_move: [Option<bool>; 2],
    reverse: bool,
    rot_dir: Option<bool>,
}

impl<F> Default for ViewControl<F>
where
    F: MyFloat
{
    fn default() -> Self
    {
        Self {
            center_vel: Complex::zero(),
            shift_vel: Complex::zero(),
            exp_vel: Complex::zero(),
            zoom_vel: F::one(),
            rot_vel: F::zero(),
            center_move: [None; 2],
            shift_move: [None; 2],
            exp_move: [None; 2],
            reverse: true,
            rot_dir: None,
        }
    }
}

impl<F> ViewControl<F>
where
    F: MyFloat
{
    pub fn reset(&mut self)
    {
        let Self { center_move, exp_move, reverse, rot_dir, .. } = *self;
        *self = Self {
            center_move,
            exp_move,
            reverse,
            rot_dir,
            ..Default::default()
        }
    }

    pub fn move_center(&mut self, direction: MoveDirection, button_state: ElementState)
    {
        self.center_move[direction.axis() as usize] = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn accel_center(&mut self, direction: Option<MoveDirection>)
    {
        match direction
        {
            Some(direction) => {
                let dst = match direction.axis()
                {
                    false => &mut self.center_vel.re,
                    true => &mut self.center_vel.im
                };
                let accel = f!(MOVE_CENTER_ACCEL);
                match direction.forward()
                {
                    false => *dst -= accel,
                    true => *dst += accel
                }
            },
            None => self.center_vel = Complex::zero()
        }
    }

    pub fn move_exp(&mut self, direction: MoveDirection, button_state: ElementState)
    {
        self.exp_move[direction.axis() as usize] = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn accel_exp(&mut self, direction: Option<MoveDirection>)
    {
        match direction
        {
            Some(direction) => {
                let dst = match direction.axis()
                {
                    false => &mut self.exp_vel.re,
                    true => &mut self.exp_vel.im
                };
                let accel = f!(MOVE_EXP_ACCEL);
                match direction.forward()
                {
                    false => *dst -= accel,
                    true => *dst += accel
                }
            },
            None => self.exp_vel = Complex::zero()
        }
    }

    pub fn move_shift(&mut self, direction: MoveDirection, button_state: ElementState)
    {
        self.shift_move[direction.axis() as usize] = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn accel_shift(&mut self, direction: Option<MoveDirection>)
    {
        match direction
        {
            Some(direction) => {
                let dst = match direction.axis()
                {
                    false => &mut self.shift_vel.re,
                    true => &mut self.shift_vel.im
                };
                let accel = f!(MOVE_SHIFT_ACCEL);
                match direction.forward()
                {
                    false => *dst -= accel,
                    true => *dst += accel
                }
            },
            None => self.shift_vel = Complex::zero()
        }
    }

    pub fn rotate(&mut self, direction: RotateDirection, button_state: ElementState)
    {
        self.rot_dir = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn accel_rot(&mut self, direction: Option<RotateDirection>)
    {
        match direction
        {
            Some(direction) => {
                let accel = f!(ROT_ACCEL);
                match direction
                {
                    RotateDirection::Left => self.rot_vel -= accel,
                    RotateDirection::Right => self.rot_vel += accel
                }
            },
            None => self.rot_vel = F::zero()
        }
    }

    pub fn reverse(&mut self, button_state: ElementState)
    {
        self.reverse = match button_state
        {
            ElementState::Pressed => !self.reverse,
            ElementState::Released => self.reverse
        }
    }

    pub fn accel_zoom(&mut self, direction: Option<ZoomDirection>, accel: Option<F>, _view: &View<F>)
    {
        match direction
        {
            Some(direction) => {
                let accel = Float::powf(f!(ZOOM_MUL), accel.unwrap_or(f!(MOVE_ZOOM_ACCEL)));
                match direction
                {
                    ZoomDirection::Outwards => {
                        self.zoom_vel /= accel;
                        //self.center_vel += (view.win_center/accel - view.win_center)/view.zoom
                    },
                    ZoomDirection::Inwards => {
                        self.zoom_vel *= accel;
                        //self.center_vel += (view.win_center*accel - view.win_center)/view.zoom
                    }
                }
            },
            None => self.zoom_vel = F::one()
        }
    }

    pub fn update_view<T>(&mut self, view: &mut View<F>, fractal: &T)
    where
        T: Fractal
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

        for (dir, phase) in self.center_move.into_iter()
            .zip([ident, rot270] as [fn(Complex<_>) -> Complex<_>; _])
            .filter_map(|(center_move, phase)| center_move.map(|dir| (dir, phase)))
        {
            let move_center = phase(Complex::from_polar(f!(MOVE_CENTER_SPEED)/view.zoom, view.rot));
            match dir
            {
                true => view.center -= move_center,
                false => view.center += move_center
            }
        }
        view.center += self.center_vel;

        for (dir, phase) in self.shift_move.into_iter()
            .zip([ident, rot270] as [fn(Complex<_>) -> Complex<_>; _])
            .filter_map(|(exp_move, phase)| exp_move.map(|dir| (dir, phase)))
        {
            let shift_move = phase(Complex::from(f!(MOVE_SHIFT_SPEED)));
            match dir ^ self.reverse
            {
                true => view.shift -= shift_move,
                false => view.shift += shift_move
            }
        }
        match self.reverse
        {
            true => view.shift -= self.shift_vel,
            false => view.shift += self.shift_vel
        }

        for (dir, phase) in self.exp_move.into_iter()
            .zip([ident, rot270] as [fn(Complex<_>) -> Complex<_>; _])
            .filter_map(|(exp_move, phase)| exp_move.map(|dir| (dir, phase)))
        {
            let exp_move = phase(Complex::from(f!(MOVE_EXP_SPEED)/view.zoom));
            match dir ^ self.reverse
            {
                true => view.exp -= exp_move,
                false => view.exp += exp_move
            }
        }
        match self.reverse
        {
            true => view.exp -= self.exp_vel,
            false => view.exp += self.exp_vel
        }

        match self.rot_dir
        {
            Some(dir) => {
                let rot_speed = f!(ROT_SPEED);
                match dir ^ self.reverse
                {
                    true => view.rot += rot_speed,
                    false => view.rot -= rot_speed
                }
            }
            _ => ()
        }
        match self.reverse
        {
            true => view.rot -= self.rot_vel,
            false => view.rot += self.rot_vel
        }

        let zoom_mul = self.zoom_vel;
        let new_zoom = match self.reverse
        {
            true => view.zoom/zoom_mul,
            false => view.zoom*zoom_mul
        };
        view.center += view.win_center/new_zoom - view.win_center/view.zoom;
        view.zoom = new_zoom;
        let dir = (self.zoom_vel > F::one()) ^ self.reverse;
        if (view.zoom > f!(ZOOM_RANGE.end) && dir)
            || (view.zoom < f!(ZOOM_RANGE.start) && !dir)
        {
            self.zoom_vel = Float::recip(self.zoom_vel)
        }

        /*println!();
        println!("center = {}, center_vel = {}", view.center, self.center_vel);
        println!("shift = {}, shift_vel = {}", view.shift, self.shift_vel);
        println!("exp = {}, exp_vel = {}", view.exp, self.exp_vel);
        println!("rot = {}, rot_vel = {}", view.rot, self.rot_vel);
        println!("zoom = {}, zoom_vel = {}", view.zoom, self.zoom_vel);*/
    }
}
