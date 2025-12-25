use example_lib::{Example, WgpuDevice};
use glam::vec3;
use mondrian::backend::wgpu::WgpuRenderer;

fn main() {
    let app = ExampleApp {};

    example_lib::run_example(wgpu::PresentMode::Fifo, app)
}

struct ExampleApp {}

impl Example for ExampleApp {
    fn name(&self) -> &str {
        "Mondrian Shape Effects Example"
    }

    fn draw(
        &mut self,
        painter: &mut mondrian::Painter,
        _dev: &WgpuDevice,
        _renderer: &mut WgpuRenderer,
        resolution: (u32, u32),
    ) {
        painter.add_filled_rect(
            [0.0, 0.0],
            [resolution.0 as f32, resolution.1 as f32],
            0.0,
            vec3(1.0, 1.0, 1.0).extend(1.0),
        );

        painter
            .add_filled_circle([100.0, 100.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_glow(vec3(1.0, 0.0, 0.0), 32.0);

        painter
            .add_filled_circle([100.0, 300.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_glow(vec3(0.5, 0.0, 0.0), 48.0);

        painter
            .add_filled_circle([100.0, 500.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_glow(vec3(0.25, 0.0, 0.0), 64.0);

        painter
            .add_filled_circle([300.0, 100.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_shadow(0.25, 32.0);

        painter
            .add_filled_circle([300.0, 300.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_shadow(0.5, 32.0);

        painter
            .add_filled_circle([300.0, 500.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_shadow(0.75, 32.0);

        painter
            .add_filled_circle([300.0, 700.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_shadow(1.0, 32.0);
    }
}
