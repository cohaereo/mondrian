use std::f32::consts::TAU;

use example_lib::{Example, WgpuDevice};
use glam::{Vec2, vec2, vec3};
use mondrian::backend::wgpu::WgpuRenderer;

fn main() {
    let app = ExampleApp {
        start_time: std::time::Instant::now(),
    };

    example_lib::run_example(wgpu::PresentMode::Fifo, app)
}

struct ExampleApp {
    start_time: std::time::Instant,
}

impl Example for ExampleApp {
    fn name(&self) -> &str {
        "Mondrian Shapes Example"
    }

    fn draw(
        &mut self,
        painter: &mut mondrian::Painter,
        _dev: &WgpuDevice,
        _renderer: &mut WgpuRenderer,
        resolution: (u32, u32),
    ) {
        let time = self.start_time.elapsed().as_secs_f32();

        painter.add_filled_rect(
            [0.0, 0.0],
            [resolution.0 as f32, resolution.1 as f32],
            0.0,
            vec3(0.1, 0.1, 0.15).extend(1.0),
        );

        painter.add_filled_circle([100.0, 100.0], 50.0, [1.0, 0.0, 0.0, 1.0]);
        painter.add_filled_circle([200.0, 100.0], 50.0, [0.0, 0.0, 1.0, 1.0]);

        painter.add_filled_rect_center_size(
            [150.0, 200.0],
            [100.0, 50.0],
            0.0,
            [0.0, 1.0, 0.0, 1.0],
        );

        painter.add_filled_rect_center_size(
            [150.0, 300.0],
            [100.0, 50.0],
            [32.0, 4.0, 32.0, 32.0],
            [1.0, 0.0, 1.0, 1.0],
        );

        // 6 rings in a 2x3 grid (no shape grouping)
        let yellow = [1.0, 1.0, 0.0, 1.0];
        let blue = [0.0, 0.0, 1.0, 1.0];
        painter.add_circle([100.0, 400.0], 25.0, blue, 2.0);
        painter.add_circle([140.0, 400.0], 25.0, yellow, 2.0);
        painter.add_circle([180.0, 400.0], 25.0, blue, 2.0);
        painter.add_circle([100.0, 430.0], 25.0, yellow, 2.0);
        painter.add_circle([140.0, 430.0], 25.0, blue, 2.0);
        painter.add_circle([180.0, 430.0], 25.0, yellow, 2.0);

        // 6 rings in a 2x3 grid (with shape grouping)
        // Note that individual shapes in a group do not control their style. The whole group is treated as one 'shape', and uses the style of the last added shape in the group.
        painter.begin_group();
        painter.add_circle([100.0, 500.0], 25.0, blue, 2.0);
        painter.add_circle([140.0, 500.0], 25.0, yellow, 2.0);
        painter.add_circle([180.0, 500.0], 25.0, blue, 2.0);
        painter.add_circle([100.0, 530.0], 25.0, yellow, 2.0);
        painter.add_circle([140.0, 530.0], 25.0, blue, 2.0);
        painter.add_circle([180.0, 530.0], 25.0, yellow, 2.0);
        painter.end_group();

        painter.add_rect_center_size(
            [450.0, 220.0],
            [50.0, 50.0],
            8.0,
            [1.0, 1.0, 0.0, 1.0],
            16.0,
        );

        painter.add_rect_center_size([570.0, 220.0], [50.0, 50.0], 8.0, [1.0, 1.0, 0.0, 1.0], 4.0);

        painter.add_rect_center_size([680.0, 220.0], [50.0, 50.0], 8.0, [1.0, 1.0, 0.0, 1.0], 1.0);

        painter.add_filled_line([400.0, 300.0], [500.0, 400.0], 8.0, [0.0, 0.0, 1.0, 1.0]);

        let pulsate = (time * 3.0).sin() * 0.5 + 0.5;
        let angle = (time.fract() * 360.0).to_radians();
        painter.add_filled_circle_sector(
            [300.0, 100.0],
            25.0 + pulsate * 5.0,
            50.0 - pulsate * 5.0,
            angle,
            (angle + pulsate * 0.4 + std::f32::consts::PI / 2.0) % TAU,
            [1.0, 1.0, 1.0, 1.0],
        );
        painter.add_circle_sector(
            [300.0, 200.0],
            25.0 + pulsate * 5.0,
            50.0 - pulsate * 5.0,
            angle,
            (angle + pulsate * 0.4 + std::f32::consts::PI / 2.0) % TAU,
            [1.0, 1.0, 1.0, 1.0],
            1.0,
        );

        painter.add_filled_polyquad(
            [380.0, 50.0],
            [420.0, 50.0],
            [450.0, 150.0],
            [350.0, 150.0],
            [1.0, 0.5, 0.1, 1.0],
        );

        let top_left = Vec2::ZERO;
        let bottom_right = vec2(resolution.0 as f32, resolution.1 as f32);
        let top_right = vec2(bottom_right.x, top_left.y);
        let bottom_left = vec2(top_left.x, bottom_right.y);

        painter.add_filled_triangle(
            top_left,
            top_left + vec2(32.0, 0.0),
            top_left + vec2(0.0, 32.0),
            [1.0, 0.0, 0.0, 1.0],
        );

        painter.add_filled_triangle(
            top_right,
            top_right + vec2(-32.0, 0.0),
            top_right + vec2(0.0, 32.0),
            [0.0, 1.0, 0.0, 1.0],
        );

        painter.add_filled_triangle(
            bottom_left,
            bottom_left + vec2(32.0, 0.0),
            bottom_left + vec2(0.0, -32.0),
            [0.0, 0.0, 1.0, 1.0],
        );

        painter.add_filled_triangle(
            bottom_right,
            bottom_right + vec2(-32.0, 0.0),
            bottom_right + vec2(0.0, -32.0),
            [1.0, 1.0, 0.0, 1.0],
        );
    }
}
