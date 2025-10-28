use core::fmt::Display;

use num_complex::Complex;
use num_traits::{Float, FloatConst, NumAssignOps};
use winit::event::ElementState;

use crate::{MOVE_CENTER_SPEED, MOVE_EXP_SPEED, ROT_SPEED, ZOOM_MUL, app::view::View, clamp_rem, f};

use super::{MoveDirection, RotateDirection};

#[derive(Debug, Clone, Copy)]
pub struct ViewControl
{
    center_move: [Option<bool>; 2],
    zoom_dir: bool,
    rot_dir: Option<bool>,
    exp_move: [Option<bool>; 2],
}

impl Default for ViewControl
{
    fn default() -> Self
    {
        Self {
            center_move: [None; 2],
            zoom_dir: true,
            rot_dir: None,
            exp_move: [None; 2]
        }
    }
}

impl ViewControl
{
    pub fn move_center(&mut self, direction: MoveDirection, button_state: ElementState)
    {
        self.center_move[direction.axis() as usize] = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
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

    pub fn zoom(&mut self, button_state: ElementState)
    {
        self.zoom_dir = match button_state
        {
            ElementState::Pressed => false,
            ElementState::Released => true
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

    pub(super) fn update_view<F>(self, view: &mut View<F>)
    where
        F: Float + NumAssignOps + Display + FloatConst
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

        for (dir, phase) in self.exp_move.into_iter()
            .zip([ident, rot270] as [fn(Complex<_>) -> Complex<_>; _])
            .filter_map(|(exp_move, phase)| exp_move.map(|dir| (dir, phase)))
        {
            let exp_move = phase(Complex::from(f!(MOVE_EXP_SPEED)/view.zoom));
            match dir
            {
                true => view.phi -= exp_move,
                false => view.phi += exp_move
            }
            view.phi.re = clamp_rem(view.phi.re, F::zero()..F::TAU());
            view.phi.im = clamp_rem(view.phi.im, F::zero()..F::TAU())
        }

        match self.rot_dir
        {
            Some(dir) => {
                let rot_speed = f!(ROT_SPEED);
                match dir
                {
                    true => view.rot += rot_speed,
                    false => view.rot -= rot_speed
                }
            }
            _ => ()
        }

        let zoom_mul = match self.zoom_dir
        {
            true => f!(ZOOM_MUL.recip()),
            false => f!(ZOOM_MUL)
        };
        let new_zoom = view.zoom*zoom_mul;
        view.center += view.win_center/new_zoom - view.win_center/view.zoom;
        view.zoom *= zoom_mul;

        //println!("center = {}, rot = {}, zoom = {}", view.center, view.rot, view.zoom)
    }
}