use core::{fmt::Display, ops::RangeInclusive};

use linspace::Linspace;
use num_traits::{Float, FloatConst, NumAssignOps, float::FloatCore};
use rand::{distr::{uniform::SampleUniform}};
use winit::{application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::ActiveEventLoop, window::{Fullscreen, Window, WindowId}};

use crate::{MyFloat, fractal::{self, Fractal}};

moddef::moddef!(
    flat(pub) mod {
        view,
        state
    }
);

pub struct App<F, Z, G>
where
    F: MyFloat,
    G: IntoIterator<Item = Z>,
    Z: Fractal<F>
{
    fractal: G::IntoIter,
    state: Option<State<F, Z>>
}

impl<F, Z, G> App<F, Z, G>
where
    F: MyFloat,
    G: IntoIterator<Item = Z>,
    Z: Fractal<F>
{
    pub fn new(fractal: G) -> Self
    {
        Self {
            fractal: fractal.into_iter(),
            state: None
        }
    }

    fn next_fractal(&mut self, event_loop: &ActiveEventLoop) -> Option<Z>
    {
        let fractal = self.fractal.next();
        if fractal.is_none()
        {
            event_loop.exit()
        }
        fractal
    }
}

impl<F, Z, G> ApplicationHandler<()> for App<F, Z, G>
where
    F: MyFloat,
    RangeInclusive<F>: Linspace<F>,
    G: IntoIterator<Item = Z>,
    Z: Fractal<F>
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

        self.state = futures::executor::block_on(async {
            Some(State::new(window, self.next_fractal(event_loop)?).await.unwrap())
        });
        if self.state.is_none()
        {
            event_loop.exit();
        }
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
            match state.window_event(event_loop, window_id, event)
            {
                AppAction::Idle => (),
                AppAction::NextFractal => if let Some(fractal) = self.next_fractal(event_loop)
                {
                    self.state = self.state.take().map(|state| state.with_fractal(fractal).unwrap());
                }
            }
        }
    }
}

pub enum AppAction
{
    Idle,
    NextFractal
}