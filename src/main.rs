#![feature(trait_alias)]
#![feature(iter_next_chunk)]
#![feature(unique_rc_arc)]

use core::{f32::EPSILON, f64::consts::TAU, fmt::{Debug, Display}, ops::Range};
use std::sync::Arc;

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

const START_ZOOM: f32 = 2e2;

const ROT_SPEED: f64 = TAU/16.0;
const MOVE_CENTER_SPEED: f64 = 330.0;
const MOVE_EXP_SPEED: f64 = 100.0;
const MOVE_SHIFT_SPEED: f64 = 100.0;//*MAX_ITERATIONS.ilog2() as f64/16.0;

const ROT_ACCEL: f64 = 1.0;
const MOVE_CENTER_ACCEL: f64 = 1.0;
const MOVE_EXP_ACCEL: f64 = 1.0;
const MOVE_ZOOM_ACCEL: f64 = 1.0;
const MOVE_SHIFT_ACCEL: f64 = 1.0;

const ZOOM_RANGE: Range<f32> = START_ZOOM..f32::EPSILON.recip()*100.0;
const ZOOM_MUL: f64 = 0.1;
const ZOOM_BASE: f64 = 1e4;
const MAX_ITERATIONS: u32 = 32;

const SHIFT_ZOOM_VARIANCE: f64 = 1.1;
const EXP_ZOOM_VARIANCE: f64 = 1.1;

pub trait MyFloat = Float + FloatConst + FloatCore + ComplexFloat + NumAssignOps + SampleUniform + Display + Debug;

fn main() -> anyhow::Result<()>
{
    let event_loop = EventLoop::new()?;

    let fractals = (
        [
            //Arc::new(Feigenbaum::default()),
            Arc::new(Cantor::cantor()),
            Arc::new(Cantor::cantor().sierpinski()),
            Arc::new(Cantor::assymetric(1.0/4.0..1.0/2.0)),
            Arc::new(Cantor::assymetric(1.0/4.0..1.0/2.0).sierpinski()),
            Arc::new(Cantor::assymetric(1.0/8.0..7.0/8.0)),
            Arc::new(Cantor::assymetric(1.0/8.0..7.0/8.0).sierpinski()),
            //Arc::new(FibonacciHamiltonian::default()),
            //Arc::new(Cantor::smith_volterra()), // TODO (convergance?)
            //Arc::new(Cantor::smith_volterra().sierpinski()), // TODO (convergance?)
            // TODO: cantor triangle
            //Arc::new(Blancmange::default()), // TODO
            //Arc::new(Supergolden), // TODO
            //Arc::new(Julia::clover()),
            //Arc::new(Rauzy::default()), // TODO
            // TODO: gosper island
            //Arc::new(Julia::dendrite()),
            //Arc::new(FibonacciSnowlake), // TODO: fail
            // TODO: Boundary of the tame twindragon
            //Arc::new(Henon::default()),
            // TODO: Koch snowflake
            //Arc::new(HeighwayDragon::default()),
        ] as [Arc<dyn Fractal<f64>>; _]
    ).into_iter()
        .rev()
        .cycle();
    
    let mut app = App::<f64, _, _>::new(fractals);

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
