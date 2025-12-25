use example_lib::{Example, WgpuDevice, load_rgba_texture};
use glam::vec3;
use mondrian::{TextureId, backend::wgpu::WgpuRenderer};
use slotmap::Key;

fn main() {
    let app = ExampleApp {
        texture_mtsdf: TextureId::null(),
        texture_sdf: TextureId::null(),
    };

    example_lib::run_example(wgpu::PresentMode::Fifo, app)
}

struct ExampleApp {
    texture_mtsdf: TextureId,
    texture_sdf: TextureId,
}

impl Example for ExampleApp {
    fn name(&self) -> &str {
        "Mondrian Shape Effects Example"
    }

    fn draw(
        &mut self,
        painter: &mut mondrian::Painter,
        dev: &WgpuDevice,
        renderer: &mut WgpuRenderer,
        resolution: (u32, u32),
    ) {
        if self.texture_mtsdf.is_null() {
            self.texture_mtsdf =
                load_rgba_texture(dev, renderer, include_bytes!("textures/mtsdf.rgba"));
        }
        if self.texture_sdf.is_null() {
            self.texture_sdf =
                load_rgba_texture(dev, renderer, include_bytes!("textures/sdf.rgba"));
        }

        // White background
        painter.add_filled_rect(
            [0.0, 0.0],
            [resolution.0 as f32, resolution.1 as f32],
            0.0,
            vec3(1.0, 1.0, 1.0).extend(1.0),
        );

        // White circle with red glow of varying intensity
        painter
            .add_filled_circle([100.0, 100.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_glow(vec3(1.0, 0.0, 0.0), 32.0);

        painter
            .add_filled_circle([100.0, 300.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_glow(vec3(0.5, 0.0, 0.0), 48.0);

        painter
            .add_filled_circle([100.0, 500.0], 50.0, [1.0, 1.0, 1.0, 1.0])
            .with_glow(vec3(0.25, 0.0, 0.0), 64.0);

        // White circle with black shadow of varying opacity
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

        // MTSDF character with drop shadow
        painter
            .add_filled_rect_center_size([700.0, 250.0], [150.0, 150.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_shadow(0.4, 12.0)
            .with_texture_id(self.texture_mtsdf)
            .with_texture_is_mtsdf();
        painter
            .add_filled_rect_center_size([900.0, 250.0], [50.0, 50.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_shadow(0.4, 12.0)
            .with_texture_id(self.texture_mtsdf)
            .with_texture_is_mtsdf();

        // SDF character with drop shadow
        painter
            .add_filled_rect_center_size([700.0, 670.0], [150.0, 150.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_shadow(0.4, 12.0)
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();
        painter
            .add_filled_rect_center_size([900.0, 670.0], [50.0, 50.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_shadow(0.4, 12.0)
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();
    }
}
