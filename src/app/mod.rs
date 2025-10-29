use core::{fmt::Display, ops::RangeInclusive};

use linspace::Linspace;
use num_traits::{Float, FloatConst, NumAssignOps, float::FloatCore};
use rand::{distr::{uniform::SampleUniform}};
use winit::{application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::ActiveEventLoop, window::{Fullscreen, Window, WindowId}};

use crate::{MyFloat, fractal::Fractal};

moddef::moddef!(
    flat(pub) mod {
        view,
        state
    }
);

pub struct App<F, Z, G = fn() -> Z>
where
    F: MyFloat,
    G: FnMut() -> Z,
    Z: Fractal
{
    fractal: G,
    state: Option<State<F, Z>>
}

impl<F, Z, G> App<F, Z, G>
where
    F: MyFloat,
    G: FnMut() -> Z,
    Z: Fractal
{
    pub const fn new(fractal: G) -> Self
    {
        Self {
            fractal,
            state: None
        }
    }
}

impl<F, Z, G> ApplicationHandler<()> for App<F, Z, G>
where
    F: MyFloat,
    RangeInclusive<F>: Linspace<F>,
    G: FnMut() -> Z,
    Z: Fractal
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        let window = event_loop.create_window(
            Window::default_attributes()
            .with_title("fractal-zoom")
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .with_min_inner_size(LogicalSize::new(640, 480))
            .with_inner_size(LogicalSize::new(1024, 768))
        ).unwrap();

        self.state = Some(futures::executor::block_on(async {
            State::new(window, (self.fractal)()).await.unwrap()
        }));
    }
    
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    )
    {
        if let Some(state) = self.state.as_mut()
        {
            state.window_event(event_loop, window_id, event);
        }
    }
}