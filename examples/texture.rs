use std::f32::consts::TAU;

use example_lib::{Example, WgpuDevice};
use mondrian::{backend::wgpu::WgpuRenderer, shape::TextureId};
use slotmap::Key;
use wgpu::util::DeviceExt;

fn main() {
    let app = ExampleApp {
        start_time: std::time::Instant::now(),
        texture_id1: TextureId::null(),
        texture_id2: TextureId::null(),
    };

    example_lib::run_example(wgpu::PresentMode::Fifo, app)
}

struct ExampleApp {
    start_time: std::time::Instant,
    texture_id1: TextureId,
    texture_id2: TextureId,
}

impl ExampleApp {
    fn create_texture(
        &mut self,
        dev: &WgpuDevice,
        renderer: &mut WgpuRenderer,
        data: &[u8],
        size: (u32, u32),
    ) -> TextureId {
        let tex = dev.create_texture_with_data(
            &dev.queue,
            &wgpu::wgt::TextureDescriptor {
                label: Some("Example Texture"),
                size: wgpu::Extent3d {
                    width: size.0,
                    height: size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::wgt::TextureDataOrder::MipMajor,
            data,
        );

        renderer.register_texture(tex.create_view(&wgpu::TextureViewDescriptor::default()))
    }
}

impl Example for ExampleApp {
    fn name(&self) -> &str {
        "Mondrian Texture Example"
    }

    fn draw(
        &mut self,
        painter: &mut mondrian::Painter,
        dev: &WgpuDevice,
        renderer: &mut WgpuRenderer,
        _resolution: (u32, u32),
    ) {
        if self.texture_id1.is_null() {
            self.texture_id1 = self.create_texture(
                dev,
                renderer,
                include_bytes!("textures/painting.data"),
                (512, 512),
            );
        }
        if self.texture_id2.is_null() {
            self.texture_id2 = self.create_texture(
                dev,
                renderer,
                include_bytes!("textures/mondrian.data"),
                (256, 256),
            );
        }
        let time = self.start_time.elapsed().as_secs_f32();

        painter
            .add_filled_circle([150.0, 150.0], 100.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_id1);

        painter.add_filled_circle([150.0, 650.0], 100.0, [0.2, 1.0, 0.2, 0.5]);

        painter
            .add_filled_circle([400.0, 650.0], 100.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_id2);

        painter
            .add_filled_rect([300.0, 50.0], [500.0, 250.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_id1);
        painter
            .add_filled_rect([550.0, 50.0], [750.0, 250.0], 48.0, [1.0, 1.0, 0.0, 1.0])
            .with_texture_id(self.texture_id1);
        painter
            .add_filled_circle_sector(
                [900.0, 150.0],
                50.0,
                100.0,
                0.0,
                TAU * (0.5 + 0.5 * (time).sin()),
                [0.0, 0.0, 1.0, 1.0],
            )
            .with_texture_id(self.texture_id1);
        painter
            .add_filled_polyquad(
                [100.0, 350.0],
                [250.0, 350.0],
                [200.0, 450.0],
                [50.0, 450.0],
                [1.0, 1.0, 0.0, 1.0],
            )
            .with_texture_id(self.texture_id1);
        painter
            .add_filled_triangle(
                [400.0, 300.0],
                [500.0, 500.0],
                [300.0, 500.0],
                [1.0, 0.0, 1.0, 1.0],
            )
            .with_texture_id(self.texture_id1);

        painter
            .add_filled_line([570.0, 320.0], [730.0, 480.0], 12.0, [0.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_id1);
    }
}
