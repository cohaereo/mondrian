use example_lib::Example;
use glam::vec2;

fn main() {
    const NUM_FRAMES: usize = 10000;
    let app = BenchApp {
        bench_frames: NUM_FRAMES,
        current_frame: 0,
        start_time: std::time::Instant::now(),
    };
    example_lib::run_example(wgpu::PresentMode::Immediate, app);
}

struct BenchApp {
    bench_frames: usize,
    current_frame: usize,
    start_time: std::time::Instant,
}

impl Example for BenchApp {
    fn name(&self) -> &str {
        "Mondrian 10k Circles Benchmark"
    }

    fn draw(&mut self, painter: &mut mondrian::Painter, resolution: (u32, u32)) {
        for _ in 0..10000 {
            let x = fastrand::f32() * resolution.0 as f32;
            let y = fastrand::f32() * resolution.1 as f32;
            let radius = fastrand::f32() * 10.0;
            painter.add_filled_circle(
                vec2(x, y),
                radius,
                glam::vec3(fastrand::f32(), fastrand::f32(), fastrand::f32())
                    .normalize()
                    .extend(1.0),
            );
        }

        self.current_frame += 1;
        if self.current_frame >= self.bench_frames {
            let elapsed = self.start_time.elapsed().as_secs_f32();
            let fps = self.bench_frames as f32 / elapsed;
            println!(
                "Rendered {} frames in {:.2} seconds ({:.2} FPS)",
                self.bench_frames, elapsed, fps
            );
            std::process::exit(0);
        }
    }
}
