struct VertexInput {
    @location(0) pos: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
}

struct Uniform {
    res: vec2<f32>,
    time: f32,
    frame: i32,
    mouse: vec2<f32>,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> u: Uniform;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4f(in.pos, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(@builtin(position) pos: vec4f) -> @location(0) vec4<f32> {
    let px = (pos.xy / u.res) + (u.mouse / u.res / 4.0);

    var color = 0.0;
    color += sin(px.x * cos(u.time / 15.0) * 80.0) + cos(px.y * cos(u.time / 15.0) * 10.0);
    color += sin(px.y * sin(u.time / 10.0) * 40.0) + cos(px.x * sin(u.time / 25.0) * 40.0);
    color += sin(px.x * sin(u.time / 5.0) * 10.0) + sin(px.y * sin(u.time / 35.0) * 80.0);
    color *= sin(u.time / 10.0) * 0.5;

    return vec4f(vec3f(color, color * 0.5, sin(color + u.time / 3.0) * 0.75), 1.0);
}
