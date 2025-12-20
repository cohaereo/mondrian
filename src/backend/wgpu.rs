use wgpu::util::DeviceExt;

use crate::{Shape, backend::common::GpuShape};

pub struct WgpuRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,

    shape_buffer_bindgroup_layout: wgpu::BindGroupLayout,
    shape_buffer: wgpu::Buffer,
    /// Size of the shape buffer in elements
    shape_buffer_size: usize,
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
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    visibility: wgpu::ShaderStages::FRAGMENT,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shape Pipeline Layout"),
            bind_group_layouts: &[&shape_buffer_bindgroup_layout],
            push_constant_ranges: &[],
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shape Buffer Bind Group"),
            layout: &shape_buffer_bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &shape_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        Self {
            pipeline,
            bind_group,
            shape_buffer,
            shape_buffer_bindgroup_layout,
            shape_buffer_size: 1,
        }
    }

    pub fn update_shape_buffer(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        shapes: &[Shape],
    ) {
        let mut gpu_shapes: Vec<GpuShape> = shapes.iter().map(|s| s.into()).collect();
        if gpu_shapes.len() < self.shape_buffer_size {
            gpu_shapes.resize(
                self.shape_buffer_size,
                GpuShape {
                    shape_type: u32::MAX,
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
            self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Shape Buffer Bind Group"),
                layout: &self.shape_buffer_bindgroup_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.shape_buffer,
                        offset: 0,
                        size: None,
                    }),
                }],
            });
            self.shape_buffer_size = gpu_shapes.len();
        } else {
            queue.write_buffer(&self.shape_buffer, 0, shape_data);
        }
    }

    pub fn render(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..3, 0..1); // Draw a full-screen triangle
    }
}
