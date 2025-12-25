use example_lib::{Example, WgpuDevice};
use mondrian::{backend::wgpu::WgpuRenderer, shape::TextureId};
use slotmap::Key;
use wgpu::util::DeviceExt;

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

impl ExampleApp {
    fn create_texture(
        &mut self,
        dev: &WgpuDevice,
        renderer: &mut WgpuRenderer,
        data: &[u8],
    ) -> TextureId {
        assert_eq!(
            &data[..4],
            b"RGBA",
            "Texture data must start with 'RGBA' header"
        );
        let size = (
            u32::from_be_bytes([data[4], data[5], data[6], data[7]]),
            u32::from_be_bytes([data[8], data[9], data[10], data[11]]),
        );

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
            data[12..].as_ref(),
        );

        renderer.register_texture(tex.create_view(&wgpu::TextureViewDescriptor::default()))
    }
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
                self.create_texture(dev, renderer, include_bytes!("textures/mtsdf.rgba"));
        }
        if self.texture_sdf.is_null() {
            self.texture_sdf =
                self.create_texture(dev, renderer, include_bytes!("textures/sdf.rgba"));
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
            .add_filled_rect_center_size([1270.0, 200.0], [6.0, 6.0], 0.0, [1.0, 1.0, 1.0, 1.0])
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
            .add_filled_rect_center_size([1270.0, 600.0], [6.0, 6.0], 0.0, [1.0, 1.0, 1.0, 1.0])
            .with_texture_id(self.texture_sdf)
            .with_texture_is_sdf();
    }
}
