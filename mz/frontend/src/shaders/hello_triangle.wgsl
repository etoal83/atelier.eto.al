struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) i_vertex: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(i_vertex)) * 0.5;
    let y = f32(i32(i_vertex & 1u) * 2 - 1) * 0.5;
    out.pos = vec4f(x, y, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4f(0.3, 0.2, 0.1, 1.0);
}
