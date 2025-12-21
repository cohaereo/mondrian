use wgpu::{ShaderStages, util::DeviceExt};

use crate::{Shape, backend::common::GpuShape, binner::ShapeBinner};

pub struct WgpuRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,

    shape_buffer_bindgroup_layout: wgpu::BindGroupLayout,
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
}

impl WgpuRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Mondrian main drawing shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("main.wgsl").into()),
        });

        let shape_buffer_bindgroup_layout =
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shape Pipeline Layout"),
            bind_group_layouts: &[&shape_buffer_bindgroup_layout],
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shape Buffer Bind Group"),
            layout: &shape_buffer_bindgroup_layout,
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

        Self {
            pipeline,
            bind_group,
            shape_buffer,
            shape_ranges_buffer,
            shape_ranges_buffer_size: 1,
            shape_indices_buffer,
            shape_indices_buffer_size: 1,
            shape_buffer_bindgroup_layout,
            shape_buffer_size: 1,
            screen_width_tiles: 0,
        }
    }

    pub fn update_shape_buffer(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shapes: &[Shape],
        binner: &ShapeBinner,
    ) {
        self.screen_width_tiles = binner.resolution.0.div_ceil(binner.tile_size);
        let mut gpu_shapes: Vec<GpuShape> = shapes.iter().map(|s| s.into()).collect();
        if gpu_shapes.len() < self.shape_buffer_size {
            gpu_shapes.resize(
                self.shape_buffer_size,
                GpuShape {
                    shape_type: u32::MAX,
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
            self.recreate_bindgroup(device);
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
            self.recreate_bindgroup(device);
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
            self.recreate_bindgroup(device);
            self.shape_indices_buffer_size = binner.shape_indices.len();
        } else {
            queue.write_buffer(&self.shape_indices_buffer, 0, shape_indices_data);
        }
    }

    fn recreate_bindgroup(&mut self, device: &wgpu::Device) {
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shape Buffer Bind Group"),
            layout: &self.shape_buffer_bindgroup_layout,
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

    pub fn render(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_push_constants(
            ShaderStages::FRAGMENT,
            0,
            &u32::to_ne_bytes(self.screen_width_tiles),
        );
        pass.draw(0..3, 0..1); // Draw a full-screen triangle
    }
}
