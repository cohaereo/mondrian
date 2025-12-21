use std::ops::Deref;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::{application::ApplicationHandler, keyboard::KeyCode, window::WindowAttributes};

pub fn run_example<E: Example + 'static>(present_mode: wgpu::PresentMode, example: E) {
    let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = ExampleApp {
        present_mode,
        painter: mondrian::Painter::new(),
        device: None,
        renderer: None,
        example: Box::new(example),
    };
    event_loop
        .run_app(&mut app)
        .expect("Failed to run application");
}

pub trait Example {
    fn name(&self) -> &str;
    fn draw(&mut self, painter: &mut mondrian::Painter, resolution: (u32, u32));
}

struct ExampleApp {
    present_mode: wgpu::PresentMode,
    painter: mondrian::Painter,
    device: Option<Device<'static>>,
    renderer: Option<mondrian::backend::wgpu::WgpuRenderer>,
    example: Box<dyn Example>,
}

impl ExampleApp {
    fn draw(&mut self) {
        let device = match &self.device {
            Some(device) => device,
            None => return,
        };
        let renderer = match &mut self.renderer {
            Some(renderer) => renderer,
            None => return,
        };

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

        let resolution = (device.surface_config.width, device.surface_config.height);
        self.painter.start(resolution);
        self.example.draw(&mut self.painter, resolution);
        self.painter.finish(|shapes, binner| {
            renderer.update_shape_buffer(&device.device, &device.queue, shapes, binner);
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
                        1200, 900,
                    ))),
            )
            .expect("Failed to create window");

        let device = pollster::block_on(Device::new(window, self.present_mode));
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
    async fn new(window: winit::window::Window, present_mode: wgpu::PresentMode) -> Self {
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
                required_features: wgpu::Features::PUSH_CONSTANTS,
                required_limits: wgpu::Limits {
                    max_push_constant_size: 4,
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .expect("Failed to create device");

        let window_size = window.inner_size();
        let surface = unsafe {
            instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: window.display_handle().unwrap().as_raw(),
                    raw_window_handle: window.window_handle().unwrap().as_raw(),
                })
                .expect("Failed to create surface")
        };
        let mut surface_config = surface
            .get_default_config(&adapter, window_size.width, window_size.height)
            .expect("Failed to get default surface config");
        surface_config.format = wgpu::TextureFormat::Rgba8Unorm;
        surface_config.present_mode = present_mode;
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
