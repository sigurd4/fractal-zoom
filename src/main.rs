use core::f64::consts::TAU;

use num_complex::Complex;

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
use winit::{event_loop::{ActiveEventLoop, EventLoop}, window::Window};

use crate::{app::{App, State}, fractal::Mandelbrot};

const INITIAL_WINDOW_SIZE: [f64; 2] = [640.0, 480.0];
const CONTRAST: f32 = 0.015;
const START_ZOOM: f32 = 100.0;
const ROT_SPEED: f64 = TAU/100.0;
const MOVE_SPEED: f64 = 10.0;
const ZOOM_MUL: f64 = 0.96;
const PIXEL_SIZE: usize = 6;

fn main() -> anyhow::Result<()>
{
    let event_loop = EventLoop::new()?;

    let fractal = Mandelbrot;
    
    let mut app = App::<f32, _, _>::new(move || fractal);

    event_loop.run_app(&mut app)?;
    Ok(())
}