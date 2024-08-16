use wgpu::{*, util::*};
use zoon::{AnimationLoop, chrono::Duration};
use super::{
    GpuContext,
    Shader,
    ANIMATION_LOOP,
    CANVAS_POINTER_POSITION,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 1] = vertex_attr_array![0 => Float32x2];

    fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, 1.0] },
    Vertex { position: [-1.0, -1.0] },
    Vertex { position: [1.0, -1.0] },
    Vertex { position: [1.0, 1.0] },
];

const INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3,
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniform {
    resolution: [f32; 2],
    time: f32,
    frame: i32,
    mouse: [f32; 2],
    _padding: [f32; 2],
}

impl Uniform {
    fn new() -> Self {
        Self {
            resolution: [0.0, 0.0],
            time: 0.0,
            frame: 0,
            mouse: [0.0, 0.0],
            _padding: [0.0, 0.0],
        }
    }

    fn update(&mut self, ctx: &GpuContext, delta: Duration) {
        self.resolution = [ctx.config.width as f32, ctx.config.height as f32];
        self.time += delta.num_milliseconds() as f32 / 1000.0;
        self.frame += 1;
        let (x, y) = CANVAS_POINTER_POSITION.get();
        self.mouse = [x as f32, y as f32];
    }
}


#[derive(Debug, Default, PartialEq)]
pub struct ShaderWork;

impl Shader for ShaderWork {
    async fn run(canvas: zoon::web_sys::HtmlCanvasElement) {
        let mut ctx = GpuContext::new(canvas).await;
        let shader = ctx.device.create_shader_module(include_wgsl!("./glslsandbox_example.wgsl"));

        // Buffers
        let vertex_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsages::INDEX,
        });
        let mut uniform = Uniform::new();
        let uniform_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // Bind groups
        let uniform_bind_group_layout = ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let uniform_bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            layout: &uniform_bind_group_layout,
        });

        let pipeline_layout = ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &uniform_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });
        let render_pipeline = ctx.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: ctx.config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState::default(),   // topology: TriangleList, front_face: Ccw などをココで設定できる
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        let animation_loop = AnimationLoop::new(move |delta| {
            ctx.resize();
            uniform.update(&ctx, delta);
            ctx.queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));

            let Ok(frame) = ctx.surface.get_current_texture() else {
                eprintln!("Target texture not found.");
                return;
            };
            let view = frame.texture.create_view(&TextureViewDescriptor {
                format: Some(ctx.config.format.add_srgb_suffix()),
                ..Default::default()
            });
            let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
                label: None,
            });

            {
                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(
                                Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: StoreOp::Store,
                        }
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                render_pass.set_pipeline(&render_pipeline);
                render_pass.set_bind_group(0, &uniform_bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..(INDICES.len() as u32), 0, 0..1);
            }

            ctx.queue.submit(std::iter::once(encoder.finish()));
            frame.present();
        });

        ANIMATION_LOOP.set(Some(animation_loop));
    }
}
