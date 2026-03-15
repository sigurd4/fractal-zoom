use core::time::Duration;
use std::time::SystemTime;

use crate::{MAX_ITERATIONS, MIN_MAX_ITERATIONS, REFRESH_RATE};

#[derive(Debug)]
pub struct FpsMonitor
{
    t: SystemTime,
    max_iterations: f32
}

impl Default for FpsMonitor
{
    fn default() -> Self
    {
        Self {
            t: SystemTime::now(),
            max_iterations: MAX_ITERATIONS as f32
        }
    }
}

impl FpsMonitor
{
    pub fn update(&mut self)
    {
        let t_prev = std::mem::replace(&mut self.t, SystemTime::now());
        if let Some(dt) = self.t.duration_since(t_prev).ok()
        {
            let refresh_time = Duration::from_secs_f64(1.0/REFRESH_RATE);
            self.max_iterations -= dt.div_duration_f32(refresh_time).ln();
        }
        self.max_iterations = self.max_iterations();
    }

    pub fn max_iterations(&self) -> f32
    {
        self.max_iterations.max(MIN_MAX_ITERATIONS as f32)
    }
}