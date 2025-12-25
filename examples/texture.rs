use std::f32::consts::TAU;

use example_lib::{Example, WgpuDevice, load_rgba_texture};
use mondrian::{backend::wgpu::WgpuRenderer, shape::TextureId};
use slotmap::Key;

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
            self.texture_id1 =
                load_rgba_texture(dev, renderer, include_bytes!("textures/painting.rgba"));
        }
        if self.texture_id2.is_null() {
            self.texture_id2 =
                load_rgba_texture(dev, renderer, include_bytes!("textures/mondrian.rgba"));
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
