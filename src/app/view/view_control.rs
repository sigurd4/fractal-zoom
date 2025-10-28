use core::fmt::Display;

use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, FloatConst, NumAssignOps, Zero};
use winit::event::{ElementState, TouchPhase};

use crate::{MOVE_CENTER_ACCEL, MOVE_CENTER_SPEED, MOVE_EXP_ACCEL, MOVE_EXP_SPEED, MOVE_ZOOM_ACCEL, MyFloat, ROT_ACCEL, ROT_SPEED, ZOOM_MUL, ZOOM_RANGE, app::{ZoomDirection, view::View}, clamp_rem, f};

use super::{MoveDirection, RotateDirection};

#[derive(Debug, Clone, Copy)]
pub struct ViewControl<F>
where
    F: MyFloat
{
    center_vel: Complex<F>,
    phi_vel: Complex<F>,
    zoom_vel: F,
    rot_vel: F,

    center_move: [Option<bool>; 2],
    phi_move: [Option<bool>; 2],
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
            phi_vel: Complex::zero(),
            zoom_vel: F::one(),
            rot_vel: F::zero(),
            center_move: [None; 2],
            phi_move: [None; 2],
            reverse: true,
            rot_dir: None,
        }
    }
}

impl<F> ViewControl<F>
where
    F: MyFloat
{
    pub fn move_center(&mut self, direction: MoveDirection, button_state: ElementState)
    {
        self.center_move[direction.axis() as usize] = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub fn accel_center(&mut self, direction: MoveDirection)
    {
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
    }

    pub fn accel_phi(&mut self, direction: MoveDirection)
    {
        let dst = match direction.axis()
        {
            false => &mut self.phi_vel.re,
            true => &mut self.phi_vel.im
        };
        let accel = f!(MOVE_EXP_ACCEL);
        match direction.forward()
        {
            false => *dst -= accel,
            true => *dst += accel
        }
    }

    pub fn accel_zoom(&mut self, direction: ZoomDirection, accel: Option<F>, _view: &View<F>)
    {
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
    }

    pub fn accel_rot(&mut self, direction: RotateDirection)
    {
        let accel = f!(ROT_ACCEL);
        match direction
        {
            RotateDirection::Left => self.rot_vel -= accel,
            RotateDirection::Right => self.rot_vel += accel
        }
    }

    pub fn move_exp(&mut self, direction: MoveDirection, button_state: ElementState)
    {
        self.phi_move[direction.axis() as usize] = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
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

    pub fn rotate(&mut self, direction: RotateDirection, button_state: ElementState)
    {
        self.rot_dir = match button_state
        {
            ElementState::Pressed => Some(direction.forward()),
            ElementState::Released => None
        }
    }

    pub(super) fn update_view(&mut self, view: &mut View<F>)
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

        let prev_phi = view.phi;
        for (dir, phase) in self.phi_move.into_iter()
            .zip([ident, rot270] as [fn(Complex<_>) -> Complex<_>; _])
            .filter_map(|(exp_move, phase)| exp_move.map(|dir| (dir, phase)))
        {
            let phi_move = phase(Complex::from(f!(MOVE_EXP_SPEED)/view.zoom));
            match dir ^ self.reverse
            {
                true => view.phi -= phi_move,
                false => view.phi += phi_move
            }
        }
        match self.reverse
        {
            true => view.phi -= self.phi_vel,
            false => view.phi += self.phi_vel
        }
        view.phi.re = clamp_rem(view.phi.re, F::zero()..F::TAU());
        view.phi.im = clamp_rem(view.phi.im, F::zero()..F::TAU());
        let exp = view.exp();
        // f = view.center.powc(exp) - view.center
        let df_dcenter = exp*view.center.powc(exp - F::one()) - F::one();
        let df_dexp = view.center.ln()*view.center.powc(exp);
        let dcenter_dphi = df_dexp/df_dcenter*view.dexp_dphi();
        if dcenter_dphi.is_finite()
        {
            println!("dcenter_dphi = {dcenter_dphi}");
            //view.center -= dcenter_dphi*(view.phi - prev_phi);
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

        println!("center = {}, rot = {}, zoom = {}", view.center, view.rot, view.zoom)
    }

    pub fn reset(&mut self)
    {
        self.center_vel = Complex::zero();
        self.phi_vel = Complex::zero();
        self.rot_vel = F::zero();
        self.zoom_vel = F::one();
    }
}