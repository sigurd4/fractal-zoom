use core::{fmt::Display, ops::RangeInclusive};
use std::{sync::{Arc, UniqueArc}, time::SystemTime};

use linspace::Linspace;
use num_complex::Complex;
use num_traits::{Float, FloatConst, NumAssignOps, float::FloatCore};
use rand::distr::uniform::SampleUniform;
use wgpu::{SurfaceConfiguration, util::DeviceExt};
use winit::{dpi::{PhysicalPosition, PhysicalSize, Size}, event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent}, keyboard::{KeyCode, PhysicalKey}, window::{Fullscreen, Window}};

use crate::{MOVE_CENTER_ACCEL, MOVE_EXP_ACCEL, MOVE_SHIFT_ACCEL, MOVE_ZOOM_ACCEL, MyFloat, ROT_ACCEL, ZOOM_MUL, app::{AppAction, MoveDirection, RotateDirection, ZoomDirection, view::View}, f, fractal::{Fractal, GlobalUniforms, VertexInput, WgpuBindGroup0, WgpuBindGroup0Entries, WgpuBindGroup0EntriesParams}};

#[derive(Debug)]
pub struct State<F, Z>
where
    F: MyFloat,
    Z: Fractal<F>
{
    fractal: Z,
    view: View<F>,
    render: Render,
    global_uniforms_buffer: wgpu::Buffer,
    global_bind_group: WgpuBindGroup0,
    vertex_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline
}

impl<F, Z> State<F, Z>
where
    F: MyFloat + Display,
    Z: Fractal<F>
{
    pub async fn new(window: Window, fractal: Z) -> anyhow::Result<Self>
    where
        F: SampleUniform
    {
        let render = Render::new(window).await?;
        Self::from_parts(render, fractal)
    }

    pub fn with_fractal<X>(self, fractal: X) -> anyhow::Result<State<F, X>>
    where
        X: Fractal<F>
    {
        let Self { render: render, .. } = self;
        State::from_parts(render, fractal)
    }

    fn from_parts(render: Render, fractal: Z) -> anyhow::Result<Self>
    {
        let size = render.window.inner_size();
        let view = View::new(&fractal, size);

        let global_uniforms = view.uniforms();

        let global_uniforms_buffer = render.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Global uniforms buffer"),
            contents: bytemuck::cast_slice(&[global_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        // Create bind group using generated types - fully type-safe!
        let global_bind_group = WgpuBindGroup0::from_bindings(
            &render.device,
            WgpuBindGroup0Entries::new(WgpuBindGroup0EntriesParams {
                globals: wgpu::BufferBinding {
                    buffer: &global_uniforms_buffer,
                    offset: 0,
                    size: None,
                },
            })
        );

        let render_pipeline = fractal.setup_render_pipeline(&render.device, render.surface_format);
        let vertices = core::array::from_fn::<_, 6, _>(|i| VertexInput { vertex_id: i as u32 });

        println!("Creating vertex buffer.");
        let vertex_buffer = render.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fractal Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Ok(Self {
            fractal,
            view,
            render,
            global_uniforms_buffer,
            global_bind_group,
            vertex_buffer,
            render_pipeline
        })
    }
    
    pub fn resize(&mut self, new_size: PhysicalSize<u32>)
    {
        if new_size.width > 0 && new_size.height > 0
        {
            self.view.resize(new_size);
            self.render.config.width = new_size.width;
            self.render.config.height = new_size.height;
            self.render.surface.configure(&self.render.device, &self.render.config);
        }
    }

    pub fn update(&mut self) -> anyhow::Result<()>
    {
        self.view.update()?;

        // Update global uniforms with new frame size immediately
        let global_uniforms = self.view.uniforms();

        self.render.queue.write_buffer(
            &self.global_uniforms_buffer,
            0,
            bytemuck::cast_slice(&[global_uniforms]),
        );
        Ok(())
    }

    pub fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) -> AppAction
    where
        RangeInclusive<F>: Linspace<F>
    {
        if window_id != self.render.window.id()
        {
            return AppAction::Idle;
        }

        match event
        {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                // Update window title when demo changes
                self.render.window.set_title(&format!(
                    "fractal-zoom: {}",
                    self.fractal.label()
                ));

                enum Action
                {
                    MoveCenter(MoveDirection),
                    MoveShift(MoveDirection),
                    MoveExp(MoveDirection),
                    Zoom(ZoomDirection),

                    Rotate(RotateDirection),
                    RotateCenter(RotateDirection),
                    RotateShift(RotateDirection),
                    RotateExp(RotateDirection),
                    
                    AccelCenter(Option<MoveDirection>),
                    AccelShift(Option<MoveDirection>),
                    AccelExp(Option<MoveDirection>),
                    AccelZoom(Option<ZoomDirection>),
                    AccelRotate(Option<RotateDirection>),

                    Reverse,
                    Stop,
                    Idle,
                    Fullscreen,
                    Reset,
                    ResetTime,
                    ResetView,
                    NextFractal
                }

                match match event.physical_key
                {
                    PhysicalKey::Code(key_code) => match key_code {
                        KeyCode::Escape | KeyCode::Abort => {
                            event_loop.exit();
                            return AppAction::Idle;
                        }
                        KeyCode::KeyI => Action::MoveExp(MoveDirection::Up),
                        KeyCode::KeyJ => Action::MoveExp(MoveDirection::Left),
                        KeyCode::KeyK => Action::MoveExp(MoveDirection::Down),
                        KeyCode::KeyL => Action::MoveExp(MoveDirection::Right),
                        KeyCode::KeyW => Action::MoveShift(MoveDirection::Up),
                        KeyCode::KeyA => Action::MoveShift(MoveDirection::Left),
                        KeyCode::KeyS => Action::MoveShift(MoveDirection::Down),
                        KeyCode::KeyD => Action::MoveShift(MoveDirection::Right),
                        KeyCode::ArrowUp => Action::MoveCenter(MoveDirection::Up),
                        KeyCode::ArrowLeft => Action::MoveCenter(MoveDirection::Left),
                        KeyCode::ArrowDown => Action::MoveCenter(MoveDirection::Down),
                        KeyCode::ArrowRight => Action::MoveCenter(MoveDirection::Right),
                        KeyCode::KeyQ => Action::Rotate(RotateDirection::Left),
                        KeyCode::KeyE => Action::Rotate(RotateDirection::Right),
                        KeyCode::Comma => Action::RotateCenter(RotateDirection::Left),
                        KeyCode::Period => Action::RotateCenter(RotateDirection::Right),
                        KeyCode::KeyZ => Action::RotateExp(RotateDirection::Left),
                        KeyCode::KeyX => Action::RotateExp(RotateDirection::Right),
                        KeyCode::KeyN => Action::RotateShift(RotateDirection::Left),
                        KeyCode::KeyM => Action::RotateShift(RotateDirection::Right),
                        KeyCode::NumpadAdd => Action::Zoom(ZoomDirection::Inwards),
                        KeyCode::NumpadSubtract => Action::Zoom(ZoomDirection::Outwards),
                        KeyCode::Space => Action::Reverse,
                        KeyCode::KeyF if matches!(event.state, ElementState::Pressed) => Action::Fullscreen,
                        KeyCode::KeyR if matches!(event.state, ElementState::Pressed) => Action::Reset,
                        KeyCode::KeyV if matches!(event.state, ElementState::Pressed) => Action::ResetView,
                        KeyCode::KeyT if matches!(event.state, ElementState::Pressed) => Action::ResetTime,
                        KeyCode::KeyV if matches!(event.state, ElementState::Pressed) => Action::ResetView,
                        KeyCode::KeyG if matches!(event.state, ElementState::Pressed) => Action::NextFractal,
                        _ => Action::Idle
                    },
                    PhysicalKey::Unidentified(_) => Action::Idle
                }
                {
                    Action::Idle => return AppAction::Idle,
                    Action::MoveCenter(direction) => self.view.center.mov(direction, event.state),
                    Action::MoveExp(direction) => self.view.exp.mov(direction, event.state),
                    Action::MoveShift(direction) => self.view.shift.mov(direction, event.state),
                    Action::Zoom(direction) => self.view.zoom.mov(direction, event.state),

                    Action::Rotate(direction) => self.view.rot.rot(direction, event.state),
                    Action::RotateCenter(direction) => self.view.center.rot(direction, event.state),
                    Action::RotateShift(direction) => self.view.shift.rot(direction, event.state),
                    Action::RotateExp(direction) => self.view.exp.rot(direction, event.state),

                    Action::AccelCenter(direction) => self.view.center.push(direction.map(|dir| (dir, f!(MOVE_CENTER_ACCEL)))),
                    Action::AccelShift(direction) => self.view.shift.push(direction.map(|dir| (dir, f!(MOVE_SHIFT_ACCEL)))),
                    Action::AccelExp(direction) => self.view.exp.push(direction.map(|dir| (dir, f!(MOVE_EXP_ACCEL)))),
                    Action::AccelRotate(direction) => self.view.rot.push(direction.map(|dir| (dir, f!(ROT_ACCEL)))),
                    Action::AccelZoom(direction) => self.view.zoom.push(direction.map(|dir| (dir, f!(MOVE_ZOOM_ACCEL)))),

                    Action::Reverse => self.view.reverse(event.state),
                    Action::Fullscreen => self.render.window.set_fullscreen(match self.render.window.fullscreen()
                    {
                        Some(Fullscreen::Borderless(_) | Fullscreen::Exclusive(_)) => None,
                        None => Some(Fullscreen::Borderless(None))
                    }),
                    Action::Reset => {
                        self.view.reset(&self.fractal);
                    },
                    Action::ResetTime => {
                        self.view.reset_time();
                    }
                    Action::ResetView => {
                        self.view.reset_view(&self.fractal);
                    }
                    Action::NextFractal => return AppAction::NextFractal,
                    Action::Stop => event_loop.exit()
                }

                self.render.window.request_redraw();
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size);
                self.render.window.request_redraw();
            },
            WindowEvent::MouseInput { device_id: _, state, button } => {
                match (button, state)
                {
                    (MouseButton::Left, ElementState::Pressed) => {
                        self.view.recenter();
                        self.view.reverse = false
                    },
                    (MouseButton::Middle, ElementState::Pressed) => self.view.zoom.stop(),
                    (MouseButton::Right, ElementState::Pressed) => {
                        self.view.recenter();
                        self.view.reverse = true
                    }
                    _ => ()
                }
            },
            WindowEvent::MouseWheel { device_id: _, delta, phase } => match phase
            {
                _ => {
                    let (accel, brk) = match delta
                    {
                        MouseScrollDelta::LineDelta(x, y) => (y as f64, x as f64),
                        MouseScrollDelta::PixelDelta(PhysicalPosition {x, y}) => (y, x)
                    };
                    self.view.zoom.push(Some((ZoomDirection::Inwards, f!(accel))));
                    self.view.zoom.brk(f!(brk))
                }
            },
            WindowEvent::CursorMoved { position, device_id: _ } => {
                self.view.update_mouse_pos(position);
                self.render.window.request_redraw();
            },
            WindowEvent::ScaleFactorChanged { .. } => self.resize(self.view.win_size()),
            WindowEvent::RedrawRequested => {
                match self.render()
                {
                    Ok(()) => {}
                    Err(err) => match err
                    {
                        wgpu::SurfaceError::Lost => self.resize(self.view.win_size()),
                        wgpu::SurfaceError::OutOfMemory => event_loop.exit(),
                        err => eprintln!("wgpu::SurfaceError::{err:?}")
                    }
                }
                self.render.window.request_redraw();
            },
            WindowEvent::Destroyed => event_loop.exit(),
            _ => {
                self.render.window.request_redraw();
            }
        };
        self.update().expect("Error: ");
        AppAction::Idle
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    where
        F: NumAssignOps + SampleUniform + FloatCore + FloatConst,
        RangeInclusive<F>: Linspace<F>
    {
        let output = self.render.surface.get_current_texture()?;
        let output_view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );

        let mut encoder = self.render.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.global_bind_group.set(&mut render_pass);

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count(), 0..1);

        core::mem::drop(render_pass);
        self.render.queue.submit(core::iter::once(encoder.finish()));

        output.present();
        Ok(())
    }

    fn vertex_count(&self) -> u32
    {
        (self.vertex_buffer.size()/core::mem::size_of::<VertexInput>() as u64) as u32
    }
}

#[derive(Debug)]
struct Render
{
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_format: wgpu::TextureFormat
}

impl Render
{
    pub async fn new(window: Window) -> anyhow::Result<Render>
    {
        let window = Arc::new(window);
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await?;

        let limits = adapter.limits();
        println!("WebGPU Adapter Initialized");
        println!(
            "Max binding array elements per stage: {}",
            limits.max_binding_array_elements_per_shader_stage
        );
        println!(
            "Max binding array sampler elements per stage: {}",
            limits.max_binding_array_sampler_elements_per_shader_stage
        );

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::TEXTURE_COMPRESSION_BC
                | wgpu::Features::PUSH_CONSTANTS
                | wgpu::Features::TEXTURE_BINDING_ARRAY
                | wgpu::Features::SHADER_F64,
            required_limits: wgpu::Limits {
                max_push_constant_size: 128,
                max_binding_array_elements_per_shader_stage: 4,
                max_binding_array_sampler_elements_per_shader_stage: 4,
                ..Default::default()
            },
            memory_hints: Default::default(),
            ..Default::default()
        }).await?;

        let size = window.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let surface_format = *caps.formats.first()
            .ok_or(anyhow::Error::msg("No supported adapter formats"))?;
        let config = surface.get_default_config(&adapter, size.width, size.height)
            .ok_or(anyhow::Error::msg("No default config provided"))?;
        surface.configure(&device, &config);

        Ok(Self { window, surface, config, device, queue, surface_format })
    }
}