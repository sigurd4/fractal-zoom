use core::ops::RangeInclusive;
use std::sync::Arc;

use linspace::Linspace;
use num_traits::{Float, NumAssignOps, float::FloatCore};
use rand::distr::uniform::SampleUniform;
use wgpu::{SurfaceConfiguration, util::DeviceExt};
use winit::{dpi::PhysicalSize, event::WindowEvent, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{PIXEL_SIZE, app::{MoveDirection, RotateDirection, view::{View, ViewControl}}, fractal::{Fractal, GlobalUniforms, VertexInput, WgpuBindGroup0, WgpuBindGroup0Entries, WgpuBindGroup0EntriesParams}};

#[derive(Debug)]
pub struct State<F, Z>
where
    F: Float,
    Z: Fractal
{
    fractal: Z,
    view: View<F>,
    view_control: ViewControl,
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    global_uniforms_buffer: wgpu::Buffer,
    global_bind_group: WgpuBindGroup0,
    vertex_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl<F, Z> State<F, Z>
where
    F: Float,
    Z: Fractal
{
    pub async fn new(window: Window, fractal: Z) -> anyhow::Result<Self>
    where
        F: SampleUniform
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
                | wgpu::Features::TEXTURE_BINDING_ARRAY,
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
        let surface_format = caps.formats.first()
            .ok_or(anyhow::Error::msg("No supported adapter formats"))?;
        let config = surface.get_default_config(&adapter, size.width, size.height)
            .ok_or(anyhow::Error::msg("No default config provided"))?;
        surface.configure(&device, &config);

        /*let scale_factor = window.scale_factor() as f32;
        let frame_size = nalgebra::Vector::from_array_storage(nalgebra::ArrayStorage([
            [size.width as f32, size.height as f32]
        ]));*/

        let view = View::default();

        let global_uniforms = GlobalUniforms::new(
            view.transform_3x3(),
            view.exp_vec2()
        );

        let global_uniforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Global uniforms buffer"),
            contents: bytemuck::cast_slice(&[global_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        // Create bind group using generated types - fully type-safe!
        let global_bind_group = WgpuBindGroup0::from_bindings(
            &device,
            WgpuBindGroup0Entries::new(WgpuBindGroup0EntriesParams {
                globals: wgpu::BufferBinding {
                    buffer: &global_uniforms_buffer,
                    offset: 0,
                    size: None,
                },
            })
        );

        let render_pipeline = Z::setup_render_pipeline(&device, *surface_format);
        let vertex_buffer = Self::vertex_buffer(&device, size);

        Ok(Self {
            fractal,
            view,
            view_control: Default::default(),
            window,
            surface,
            config,
            device,
            queue,
            size,
            global_uniforms_buffer,
            global_bind_group,
            vertex_buffer,
            render_pipeline
        })
    }

    fn vertex_buffer(device: &wgpu::Device, resolution: PhysicalSize<u32>) -> wgpu::Buffer
    {
        let vertices = (0..resolution.height)
            .step_by(PIXEL_SIZE)
            .flat_map(|j| (0..resolution.width)
                .step_by(PIXEL_SIZE)
                .map(move |i| VertexInput::new(glam::vec3(
                    i as f32 - resolution.width as f32,
                    j as f32 - resolution.height as f32,
                    1.0
                )))
            ).collect::<Vec<_>>();

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fractal Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
    
    pub fn resize(&mut self, new_size: PhysicalSize<u32>)
    where
        F: NumAssignOps
    {
        if new_size.width > 0 && new_size.height > 0
        {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.vertex_buffer = Self::vertex_buffer(&self.device, new_size);
        }
    }

    pub fn update(&mut self)
    where
        F: NumAssignOps
    {
        self.view.update(self.view_control);

        // Update global uniforms with new frame size immediately
        let global_uniforms = GlobalUniforms::new(
            self.view.transform_3x3(),
            self.view.exp_vec2()
        );

        self.queue.write_buffer(
            &self.global_uniforms_buffer,
            0,
            bytemuck::cast_slice(&[global_uniforms]),
        );
    }

    pub fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    )
    where
        F: NumAssignOps + SampleUniform + FloatCore,
        RangeInclusive<F>: Linspace<F>
    {
        if window_id != self.window.id()
        {
            return;
        }

        match event
        {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                // Update window title when demo changes
                self.window.set_title(&format!(
                    "fractal-zoom: {}",
                    Z::LABEL
                ));

                enum Action
                {
                    Move(MoveDirection),
                    Rotate(RotateDirection),
                    Zoom,
                    Idle
                }

                match match event.physical_key
                {
                    PhysicalKey::Code(key_code) => match key_code {
                        KeyCode::Escape => {
                            event_loop.exit();
                            return;
                        }
                        KeyCode::KeyW | KeyCode::ArrowUp => Action::Move(MoveDirection::Up),
                        KeyCode::KeyA | KeyCode::ArrowLeft => Action::Move(MoveDirection::Left),
                        KeyCode::KeyS | KeyCode::ArrowDown => Action::Move(MoveDirection::Down),
                        KeyCode::KeyD | KeyCode::ArrowRight => Action::Move(MoveDirection::Right),
                        KeyCode::KeyQ => Action::Rotate(RotateDirection::Left),
                        KeyCode::KeyE => Action::Rotate(RotateDirection::Right),
                        KeyCode::Space => Action::Zoom,
                        _ => Action::Idle
                    },
                    PhysicalKey::Unidentified(_) => Action::Idle
                }
                {
                    Action::Idle => (),
                    Action::Move(direction) => self.view_control.move_center(direction, event.state),
                    Action::Rotate(direction) => self.view_control.rotate(direction, event.state),
                    Action::Zoom => self.view_control.zoom(event.state),
                }

                self.window.request_redraw();
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size);
                self.window.request_redraw();
            }
            /*WindowEvent::CursorMoved { position, .. } => {
                state.update_mouse_position(position);
                state.window.request_redraw();
            }*/
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::RedrawRequested => {
                match self.render()
                {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("{e:?}"),
                }
                self.window.request_redraw();
            }
            _ => {
                self.window.request_redraw();
            }
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    where
        F: NumAssignOps + SampleUniform + FloatCore,
        RangeInclusive<F>: Linspace<F>
    {
        self.update();

        let output = self.surface.get_current_texture()?;
        let output_view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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
        self.queue.submit(core::iter::once(encoder.finish()));

        output.present();
        Ok(())
    }

    fn vertex_count(&self) -> u32
    {
        (self.vertex_buffer.size()/core::mem::size_of::<VertexInput>() as u64) as u32
    }
}