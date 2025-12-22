use std::{collections::HashMap, num::NonZeroU32};

use slotmap::SlotMap;
use wgpu::{ShaderStages, util::DeviceExt};

use crate::{
    Shape,
    backend::common::{GpuShape, ShapeHeader},
    binner::ShapeBinner,
    shape::TextureId,
};

pub struct WgpuRenderer {
    pipeline: wgpu::RenderPipeline,

    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group: wgpu::BindGroup,
    texture_sampler: wgpu::Sampler,

    shape_buffer_bind_group_layout: wgpu::BindGroupLayout,
    shape_buffer_bind_group: wgpu::BindGroup,

    shape_buffer: wgpu::Buffer,
    /// Size of the shape buffer in elements
    shape_buffer_size: usize,

    shape_ranges_buffer: wgpu::Buffer,
    /// Size of the shape ranges buffer in elements
    shape_ranges_buffer_size: usize,

    shape_indices_buffer: wgpu::Buffer,
    /// Size of the shape indices buffer in elements
    shape_indices_buffer_size: usize,

    screen_width_tiles: u32,

    textures: SlotMap<TextureId, wgpu::TextureView>,
    texture_id_map: HashMap<TextureId, u32>,
    placeholder_texture: wgpu::TextureView,
}

impl WgpuRenderer {
    const MAX_TEXTURES: u32 = 1024;

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Mondrian main drawing shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("main.wgsl").into()),
        });

        let shape_buffer_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Shape Buffer Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        count: None,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        count: None,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        count: None,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                ],
            });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        count: NonZeroU32::new(Self::MAX_TEXTURES),
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        count: None,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                ],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shape Pipeline Layout"),
            bind_group_layouts: &[&shape_buffer_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::FRAGMENT,
                range: 0..4,
            }],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: None,
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: None,
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let shape_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shape Buffer"),
            size: size_of::<GpuShape>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shape_ranges_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shape Ranges Buffer"),
            size: 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shape_indices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shape Indices Buffer"),
            size: 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shape_buffer_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shape Buffer Bind Group"),
            layout: &shape_buffer_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &shape_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &shape_ranges_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &shape_indices_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let placeholder_texture = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("Placeholder Texture"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
            .create_view(&wgpu::TextureViewDescriptor::default());

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(
                        &[&placeholder_texture; Self::MAX_TEXTURES as usize],
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
            ],
        });

        Self {
            pipeline,
            texture_bind_group,
            texture_bind_group_layout,
            texture_sampler,
            shape_buffer_bind_group,
            shape_buffer,
            shape_ranges_buffer,
            shape_ranges_buffer_size: 1,
            shape_indices_buffer,
            shape_indices_buffer_size: 1,
            shape_buffer_bind_group_layout,
            shape_buffer_size: 1,
            screen_width_tiles: 0,

            textures: SlotMap::with_key(),
            texture_id_map: HashMap::new(),
            placeholder_texture,
        }
    }

    fn recreate_shapes_bind_group(&mut self, device: &wgpu::Device) {
        self.shape_buffer_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shape Buffer Bind Group"),
            layout: &self.shape_buffer_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.shape_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.shape_ranges_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.shape_indices_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });
    }

    fn prepare_shape_buffers(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shapes: &[Shape],
        binner: &ShapeBinner,
    ) {
        let mut gpu_shapes: Vec<GpuShape> = shapes
            .iter()
            .map(|s| {
                let texture_id = s
                    .texture_id
                    .and_then(|tex_id| self.texture_id_map.get(&tex_id).cloned());
                GpuShape::from_shape(s, texture_id)
            })
            .collect();
        if gpu_shapes.len() < self.shape_buffer_size {
            gpu_shapes.resize(
                self.shape_buffer_size,
                GpuShape {
                    shape_header: ShapeHeader::SENTINEL,
                    group_id: u32::MAX,
                    ..Default::default()
                },
            );
        }

        let shape_data = bytemuck::cast_slice(&gpu_shapes);
        if self.shape_buffer_size < shapes.len() {
            self.shape_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Shape Buffer"),
                contents: shape_data,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });
            self.recreate_shapes_bind_group(device);
            self.shape_buffer_size = gpu_shapes.len();
        } else {
            queue.write_buffer(&self.shape_buffer, 0, shape_data);
        }

        let shape_ranges_data = bytemuck::cast_slice(&binner.tile_ranges);
        if self.shape_ranges_buffer_size < binner.tile_ranges.len() {
            self.shape_ranges_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Shape Ranges Buffer"),
                    contents: shape_ranges_data,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });
            self.recreate_shapes_bind_group(device);
            self.shape_ranges_buffer_size = binner.tile_ranges.len();
        } else {
            queue.write_buffer(&self.shape_ranges_buffer, 0, shape_ranges_data);
        }

        let shape_indices_data = bytemuck::cast_slice(&binner.shape_indices);
        if self.shape_indices_buffer_size < binner.shape_indices.len() {
            self.shape_indices_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Shape Indices Buffer"),
                    contents: shape_indices_data,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });
            self.recreate_shapes_bind_group(device);
            self.shape_indices_buffer_size = binner.shape_indices.len();
        } else {
            queue.write_buffer(&self.shape_indices_buffer, 0, shape_indices_data);
        }
    }

    /// Collects texture views from the shapes and prepares the texture bind group
    fn prepare_textures_bind_group(&mut self, device: &wgpu::Device, shapes: &[Shape]) {
        self.texture_id_map.clear();
        let mut texture_views: Vec<&wgpu::TextureView> =
            Vec::with_capacity(Self::MAX_TEXTURES as usize);
        for shape in shapes {
            let Some(tex_id) = shape.texture_id else {
                continue;
            };
            if self.texture_id_map.contains_key(&tex_id) {
                continue;
            };

            if let Some(texture_view) = self.textures.get(tex_id) {
                let new_id = texture_views.len() as u32;
                self.texture_id_map.insert(tex_id, new_id);
                texture_views.push(texture_view);
                if texture_views.len() >= Self::MAX_TEXTURES as usize {
                    println!(
                        "Warning: Reached maximum number of textures ({}). Some textures will not be available.",
                        Self::MAX_TEXTURES
                    );
                    break;
                }
            }
        }

        // Skip recreating the bind group if there are no textures in this frame
        if texture_views.is_empty() {
            return;
        }

        texture_views.resize(Self::MAX_TEXTURES as usize, &self.placeholder_texture);

        self.texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(&texture_views),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture_sampler),
                },
            ],
        });
    }

    pub fn register_texture(&mut self, texture_view: wgpu::TextureView) -> TextureId {
        self.textures.insert(texture_view)
    }

    pub fn unregister_texture(&mut self, texture_id: TextureId) {
        self.textures.remove(texture_id);
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shapes: &[Shape],
        binner: &ShapeBinner,
    ) {
        self.screen_width_tiles = binner.resolution.0.div_ceil(binner.tile_size);
        self.prepare_shape_buffers(device, queue, shapes, binner);
        self.prepare_textures_bind_group(device, shapes);
    }

    pub fn render(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.shape_buffer_bind_group, &[]);
        pass.set_bind_group(1, &self.texture_bind_group, &[]);
        pass.set_push_constants(
            ShaderStages::FRAGMENT,
            0,
            &u32::to_ne_bytes(self.screen_width_tiles),
        );
        pass.draw(0..3, 0..1); // Draw a full-screen triangle
    }
}
