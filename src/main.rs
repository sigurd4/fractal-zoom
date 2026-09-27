#![feature(trait_alias)]
#![feature(iter_next_chunk)]
#![feature(unique_rc_arc)]
#![feature(more_float_constants)]

use core::{
    f32::EPSILON,
    f64::consts::TAU,
    fmt::{Debug, Display},
    ops::Range
};
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
use f;
use num_traits::{Float, FloatConst, Num, NumAssignOps, float::FloatCore};
use rand::{
    distr::{Uniform, uniform::SampleUniform},
    prelude::Distribution
};
use winit::{
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window
};

use crate::{
    app::{App, State},
    fractal::*
};

const NEWTON_N: usize = 16;
const NEWTON_MU: f64 = 0.0001;
const DONUT: Range<f64> = 0.5..2.0;
const ZOOM_MU: f64 = 0.01;

const START_ZOOM: f32 = 2e2;

const ROT_SPEED: f64 = TAU / 16.0;
const MOVE_CENTER_SPEED: f64 = 330.0;
const MOVE_EXP_SPEED: f64 = 30.0;
const MOVE_SHIFT_SPEED: f64 = 30.0; //*MAX_ITERATIONS.ilog2() as f64/16.0;

const ROT_ACCEL: f64 = 1.0;
const MOVE_CENTER_ACCEL: f64 = 1.0;
const MOVE_EXP_ACCEL: f64 = 1.0;
const MOVE_ZOOM_ACCEL: f64 = 1.0;
const MOVE_SHIFT_ACCEL: f64 = 1.0;

const ZOOM_RANGE: Range<f32> = START_ZOOM..f32::EPSILON.recip() * 100.0;
const ZOOM_MUL: f64 = 0.1;
const ZOOM_BASE: f64 = 1e4;
const MAX_ITERATIONS: u32 = 32;
const MIN_MAX_ITERATIONS: u32 = 32;
const REFRESH_RATE: f64 = 30.0;

const SHIFT_ZOOM_VARIANCE: f64 = 1.1;
const EXP_ZOOM_VARIANCE: f64 = 1.1;

pub trait MyFloat = Float + FloatConst + FloatCore + ComplexFloat + NumAssignOps + SampleUniform + Display + Debug;

fn main() -> anyhow::Result<()>
{
    let event_loop = EventLoop::new()?;

    let fractals = ([
        Arc::new(Feigenbaum::default()),                     // Ok
        Arc::new(FibonacciHamiltonianJulia::default()),      // Ok
        Arc::new(FibonacciHamiltonianMandelbrot::default()), // Ok
        //Arc::new(Cantor::smith_volterra()), // Broken
        //Arc::new(Cantor::smith_volterra().sierpinski()), // Broken
        // TODO: cantor triangle
        Arc::new(Blancmange::default()), // Ok
        Arc::new(SupergoldenJulia),      // Ok
        Arc::new(SupergoldenMandelbrot), // Ok
        Arc::new(Julia::clover()),       // Perfect
        Arc::new(Rauzy::default()),      // Ok
        Arc::new(Julia::dendrite()),     // Perfect
        // TODO: fibonacci word fractal 60 degree
        // TODO: Boundary of the tame twindragon
        Arc::new(Henon::default()), // Perfect
        //Arc::new(FibonacciSnowflake), // TODO: fail
        // TODO: Triflake
        Arc::new(Cantor::cantor()), // OK
        // TODO: L-system
        Arc::new(Julia::pearls()), // Perfect
        // TODO: Appolonian gasket
        // TODO: Appolonian packing
        // TODO: Minkowski Sausage a.k.a. Quadratic von Koch island
        Arc::new(Julia::douady_rabbit()), // Perfect
        // TODO: Viksec fractal
        // TODO: Quadratic von Koch type 1
        // TODO: Quadric cross
        Arc::new(Weierstrass::default()), // Ok
        // TODO: Quadratic von Koch type 2
        // TODO: Twindragon
        // TODO: 3-branches tree
        // TODO: Sierpinski triangle
        // TODO: Sierpinski arrowhead
        // TODO: T-square fractal
        // TODO: Golden dragon
        // TODO: Pascal triangle modulo 3
        // TODO: Sierpinski hexagon
        // TODO: Fibonacci word fractal
        // TODO: IFS
        // TODO: Quadric fractal
        // TODO: Pascal triangle modulo 5
        Arc::new(Ikeda::default()), // Ok, but needs better parameterization
        // TODO: 50 segment quadric fractal
        // TODO: Pinwheel tiling
        // TODO: Sphinx tiling
        // TODO: Hexaflake
        // TODO: Fractal H-I de Rivera
        // TODO: Von Koch curve 85°
        Arc::new(KochPeano::koch()),
        // TODO: Pentaflake
        // TODO: Monkey's tree
        Arc::new(Cantor::cantor().sierpinski()), // Ok
        // TODO: 3D CantorDust
        // TODO: Carthesian product of von Koch curve and cantor set
        Arc::new(Cesaro::levyc()), // Perfect
        // TODO: Penrose tiling
        Arc::new(Mandelbrot),       // Perfect
        Arc::new(Julia::default()), // Perfect
        // TODO: Sierpinski curve
        // TODO: Hilbert curve
        // TODO: Peano curve
        // TODO: Moore curve
        // TODO: Lebesgue curve
        // TODO: Dragon curve
        // TODO: Terdragon curve
        // TODO: Gosper curve
        // TODO: Koch curve
        // TODO: Sierpinski tetrahedron (3D)
        // TODO: H-fractal
        // TODO: Pythagoras tree
        // TODO: 2D greek cross fractal
        // TODO: Rössler attractor (3D)
        // TODO: Lorenz attractor (3D)
        // TODO: Pyramid surface (3D)
        // TODO: Fractal pyramid (3D)
        // TODO: Dodecahedron fractal (3D)
        // TODO: Quadratic koch surface type 1 (3D)
        // TODO: Apollonian sphere packing (3D)
        // TODO: Quadratic koch surface type 2 (3D)
        // TODO: Jerusalem cube (3D)
        // TODO: Icosahedron fractal (3D)
        // TODO: 3D greek cross fractal (3D)
        // TODO: Octahedron fractal (3D)
        // TODO: Von koch surface (3D)
        // TODO: Von koch (3D)
        // TODO: Menger sponge (3D)
        // TODO: 3D hilbert curve
        // TODO: 3D lebesgue curve
        // TODO: 3D moore curve
        // TODO: 3D H-fractal
        // TODO: Mandelbulb (3D)
        //Arc::new(KochPeano::peano())
        //Arc::new(BurningShip), // Perfect
        //Arc::new(HeighwayDragon::default()), // TODO: fail
    ] as [Arc<dyn Fractal<f64>>; _])
        .into_iter()
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
