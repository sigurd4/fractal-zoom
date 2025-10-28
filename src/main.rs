#![feature(trait_alias)]

use core::{f32::EPSILON, f64::consts::TAU, fmt::Display, ops::Range};

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
use rand::distr::uniform::SampleUniform;
use winit::{event_loop::{ActiveEventLoop, EventLoop}, window::Window};

use crate::{app::{App, State}, fractal::Mandelbrot};

const START_ZOOM: f32 = 100.0;

const ROT_SPEED: f64 = TAU/1000.0;
const MOVE_CENTER_SPEED: f64 = 10.0;
const MOVE_EXP_SPEED: f64 = 1.0;

const ROT_ACCEL: f64 = 1.0;
const MOVE_CENTER_ACCEL: f64 = 1.0;
const MOVE_EXP_ACCEL: f64 = 1.0;
const MOVE_ZOOM_ACCEL: f64 = 1.0;

const ZOOM_RANGE: Range<f32> = START_ZOOM..f32::EPSILON.recip()*1e3;
const ZOOM_MUL: f64 = 0.995;
const MAX_ITERATIONS: u32 = 16;

pub trait MyFloat = Float + FloatConst + FloatCore + ComplexFloat + NumAssignOps + SampleUniform + Display;

fn main() -> anyhow::Result<()>
{
    let event_loop = EventLoop::new()?;

    let fractal = Mandelbrot;
    
    let mut app = App::<f64, _, _>::new(move || fractal);

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