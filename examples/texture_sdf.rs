use example_lib::{Example, WgpuDevice, load_rgba_texture};
use mondrian::{backend::wgpu::WgpuRenderer, shape::TextureId};
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
        "Mondrian (MT)SDF Texture Example"
    }

    fn draw(
        &mut self,
        painter: &mut mondrian::Painter,
        dev: &WgpuDevice,
        renderer: &mut WgpuRenderer,
        _resolution: (u32, u32),
    ) {
        if self.texture_mtsdf.is_null() {
            self.texture_mtsdf =
                load_rgba_texture(dev, renderer, include_bytes!("textures/mtsdf.rgba"));
        }
        if self.texture_sdf.is_null() {
            self.texture_sdf =
                load_rgba_texture(dev, renderer, include_bytes!("textures/sdf.rgba"));
        }

        // MTSDF
        painter
            .add_filled_rect_center_size([200.0, 200.0], [200.0, 200.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_mtsdf);

        painter
            .add_filled_rect_center_size([600.0, 200.0], [200.0, 200.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_mtsdf)
            .with_texture_is_mtsdf();

        painter
            .add_filled_rect_center_size([900.0, 200.0], [100.0, 100.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_mtsdf)
            .with_texture_is_mtsdf();

        painter
            .add_filled_rect_center_size([1100.0, 200.0], [50.0, 50.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_mtsdf)
            .with_texture_is_mtsdf();

        painter
            .add_filled_rect_center_size([1200.0, 200.0], [25.0, 25.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_mtsdf)
            .with_texture_is_mtsdf();

        painter
            .add_filled_rect_center_size([1250.0, 200.0], [12.0, 12.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_mtsdf)
            .with_texture_is_mtsdf();

        painter
            .add_filled_rect_center_size([1270.0, 200.0], [8.0, 8.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_mtsdf)
            .with_texture_is_mtsdf();

        // SDF
        painter
            .add_filled_rect_center_size([200.0, 600.0], [200.0, 200.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_sdf);

        painter
            .add_filled_rect_center_size([600.0, 600.0], [200.0, 200.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();

        painter
            .add_filled_rect_center_size([900.0, 600.0], [100.0, 100.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();

        painter
            .add_filled_rect_center_size([1100.0, 600.0], [50.0, 50.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();

        painter
            .add_filled_rect_center_size([1200.0, 600.0], [25.0, 25.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();

        painter
            .add_filled_rect_center_size([1250.0, 600.0], [12.0, 12.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();

        painter
            .add_filled_rect_center_size([1270.0, 600.0], [8.0, 8.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();
    }
}
