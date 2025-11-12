#![feature(trait_alias)]
#![feature(iter_next_chunk)]

use core::{f32::EPSILON, f64::consts::TAU, fmt::{Debug, Display}, ops::Range};

use num_complex::{Complex, ComplexFloat};

moddef::moddef!(
    mod {
        fractal,
        app
    }
);

macro_rules! f {
    ($x:expr) => {
        <F as num_traits::NumCast>::from($x).unwrap()
    };
}   
use f as f;
use num_traits::{Float, FloatConst, Num, NumAssignOps, float::FloatCore};
use rand::{distr::{Uniform, uniform::SampleUniform}, prelude::Distribution};
use winit::{event_loop::{ActiveEventLoop, EventLoop}, window::Window};

use crate::{app::{App, State}, fractal::*};

const NEWTON_N: usize = 16;
const NEWTON_MU: f64 = 0.0001;
const DONUT: Range<f64> = 0.5..2.0;
const ZOOM_MU: f64 = 0.01;

const START_ZOOM: f32 = 100.0;

const ROT_SPEED: f64 = TAU/1000.0;
const MOVE_CENTER_SPEED: f64 = 10.0;
const MOVE_EXP_SPEED: f64 = 0.1;
const MOVE_SHIFT_SPEED: f64 = 1.0/1000.0;//*MAX_ITERATIONS.ilog2() as f64/16.0;

const ROT_ACCEL: f64 = 1.0;
const MOVE_CENTER_ACCEL: f64 = 1.0;
const MOVE_EXP_ACCEL: f64 = 1.0;
const MOVE_ZOOM_ACCEL: f64 = 1.0;
const MOVE_SHIFT_ACCEL: f64 = 1.0;

const ZOOM_RANGE: Range<f32> = START_ZOOM..f32::EPSILON.recip()*100.0;
const ZOOM_MUL: f64 = 0.995;
const MAX_ITERATIONS: u32 = 32;

pub trait MyFloat = Float + FloatConst + FloatCore + ComplexFloat + NumAssignOps + SampleUniform + Display + Debug;

fn main() -> anyhow::Result<()>
{
    let event_loop = EventLoop::new()?;
    
    let mut app = App::<f64, _, _>::new(|| Pendulum);

    event_loop.run_app(&mut app)?;
    Ok(())
}

fn clamp_rem<T>(x: T, range: Range<T>) -> T
where
    T: Num + Copy
{
    let span = range.end - range.start;
    range.end - (span - (x - range.start) % span) % span
}

fn random<F>(range: Range<F>) -> F
where
    F: MyFloat
{
    let rng = &mut rand::rng();
    Uniform::new(range.start, range.end).unwrap().sample(rng)
}

fn random_donut<F>(r: Range<F>) -> Complex<F>
where
    F: MyFloat
{
    let r = random(r);
    let theta = random(F::zero()..F::TAU());
    Complex::from_polar(r, theta)
}
