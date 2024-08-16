use wgpu::{*, util::*};
use zoon::AnimationLoop;
use super::{
    GpuContext,
    Shader,
    ANIMATION_LOOP,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    // const ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];


#[derive(Debug, Default, PartialEq)]
pub struct ShaderWork;

impl Shader for ShaderWork {
    async fn run(canvas: zoon::web_sys::HtmlCanvasElement) {
        let mut ctx = GpuContext::new(canvas).await;
        let shader = ctx.device.create_shader_module(include_wgsl!("./hello_triangle_with_vertex_buffer.wgsl"));
    
        // Buffers
        let vertex_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });
    
        // Render pipeline
        let pipeline_layout = ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
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
    
        // Animation loop
        let animation_loop = AnimationLoop::new(move |_| {
            ctx.resize();
    
            let Ok(frame) = ctx.surface.get_current_texture() else {
                eprintln!("Target texture not found.");
                return;
            };
            let view = frame.texture.create_view(&TextureViewDescriptor::default());
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
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.draw(0..(VERTICES.len() as u32), 0..1);
            }
    
            ctx.queue.submit(std::iter::once(encoder.finish()));
            frame.present();
        });
    
        ANIMATION_LOOP.set(Some(animation_loop));
    }
}