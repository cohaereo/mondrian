use std::{f32::consts::TAU, ops::Deref};

use glam::{Vec2, vec2};
use wgpu::rwh::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::{application::ApplicationHandler, keyboard::KeyCode, window::WindowAttributes};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = ExampleApp {
        painter: mondrian::Painter::new(),
        device: None,
        renderer: None,
        start_time: std::time::Instant::now(),
    };
    event_loop
        .run_app(&mut app)
        .expect("Failed to run application");
}

struct ExampleApp {
    painter: mondrian::Painter,
    device: Option<Device<'static>>,
    renderer: Option<mondrian::backend::wgpu::WgpuRenderer>,
    start_time: std::time::Instant,
}

impl ExampleApp {
    fn draw(&mut self) {
        let time = self.start_time.elapsed().as_secs_f32();
        let device = match &self.device {
            Some(device) => device,
            None => return,
        };
        let renderer = match &mut self.renderer {
            Some(renderer) => renderer,
            None => return,
        };

        self.painter
            .add_circle([100.0, 100.0], 50.0, [1.0, 0.0, 0.0, 1.0]);
        self.painter
            .add_circle([200.0, 100.0], 50.0, [0.0, 0.0, 1.0, 1.0]);

        self.painter
            .add_box([150.0, 200.0], [100.0, 50.0], 0.0, [0.0, 1.0, 0.0, 1.0]);

        self.painter.add_box(
            [150.0, 300.0],
            [100.0, 50.0],
            [32.0, 4.0, 32.0, 32.0],
            [1.0, 0.0, 1.0, 1.0],
        );

        self.painter
            .add_line([400.0, 300.0], [500.0, 400.0], 8.0, [0.0, 0.0, 1.0, 1.0]);

        let pulsate = (time * 3.0).sin() * 0.5 + 0.5;
        let angle = (time.fract() * 360.0).to_radians();
        self.painter.add_circle_sector(
            [300.0, 100.0],
            25.0 + pulsate * 5.0,
            50.0 - pulsate * 5.0,
            angle,
            (angle + pulsate * 0.4 + std::f32::consts::PI / 2.0) % TAU,
            [1.0, 1.0, 1.0, 1.0],
        );

        self.painter.add_polyquad(
            [380.0, 50.0],
            [420.0, 50.0],
            [450.0, 150.0],
            [350.0, 150.0],
            [1.0, 0.5, 0.1, 1.0],
        );

        let top_left = Vec2::ZERO;
        let bottom_right = vec2(
            device.surface_config.width as f32,
            device.surface_config.height as f32,
        );
        let top_right = vec2(bottom_right.x, top_left.y);
        let bottom_left = vec2(top_left.x, bottom_right.y);

        self.painter.add_triangle(
            top_left,
            top_left + vec2(32.0, 0.0),
            top_left + vec2(0.0, 32.0),
            [1.0, 0.0, 0.0, 1.0],
        );

        self.painter.add_triangle(
            top_right,
            top_right + vec2(-32.0, 0.0),
            top_right + vec2(0.0, 32.0),
            [0.0, 1.0, 0.0, 1.0],
        );

        self.painter.add_triangle(
            bottom_left,
            bottom_left + vec2(32.0, 0.0),
            bottom_left + vec2(0.0, -32.0),
            [0.0, 0.0, 1.0, 1.0],
        );

        self.painter.add_triangle(
            bottom_right,
            bottom_right + vec2(-32.0, 0.0),
            bottom_right + vec2(0.0, -32.0),
            [1.0, 1.0, 0.0, 1.0],
        );

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let frame = device
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        self.painter.finish(|shapes| {
            renderer.update_shape_buffer(&device.device, &device.queue, shapes);
            renderer.render(&mut pass);
        });

        drop(pass);
        device.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

impl ApplicationHandler for ExampleApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Mondrian Shapes Example")
                    .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize::new(
                        800, 600,
                    ))),
            )
            .expect("Failed to create window");

        let device = pollster::block_on(Device::new(window));
        let renderer =
            mondrian::backend::wgpu::WgpuRenderer::new(&device, device.surface_config.format);
        self.device = Some(device);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                    event_loop.exit();
                }
            }
            winit::event::WindowEvent::RedrawRequested => {
                self.draw();
                self.device.as_ref().unwrap().window.request_redraw();
            }
            winit::event::WindowEvent::Resized(new_size) => {
                if let Some(device) = &mut self.device {
                    device.surface_config.width = new_size.width;
                    device.surface_config.height = new_size.height;
                    device
                        .surface
                        .configure(&device.device, &device.surface_config);
                }
            }
            _ => {}
        }
    }
}

struct Device<'a> {
    _instance: wgpu::Instance,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    window: winit::window::Window,
}

impl<'a> Device<'a> {
    async fn new(window: winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                ..Default::default()
            })
            .await
            .expect("Failed to create device");

        let window_size = window.inner_size();
        let surface = unsafe {
            instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: window.raw_display_handle().unwrap(),
                    raw_window_handle: window.raw_window_handle().unwrap(),
                })
                .expect("Failed to create surface")
        };
        let surface_config = surface
            .get_default_config(&adapter, window_size.width, window_size.height)
            .expect("Failed to get default surface config");
        surface.configure(&device, &surface_config);

        Self {
            _instance: instance,
            _adapter: adapter,
            device,
            queue,

            surface,
            surface_config,
            window,
        }
    }
}

impl Deref for Device<'_> {
    type Target = wgpu::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
