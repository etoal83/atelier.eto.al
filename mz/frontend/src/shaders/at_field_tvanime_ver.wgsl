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

const PI = radians(180.);

fn scale(a: f32) -> mat2x2f {
    return mat2x2f(
        vec2f(a, 0.0),
        vec2f(0.0, a),
    );
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let px = (2.0 * pos.xy - u.res) / min(u.res.x, u.res.y) * vec2f(1.0, -1.0);

    let n = 8;
    let radius = 2.0 * PI / f32(n);
    let angle = atan2(px.x, px.y) + PI;

    let d = cos(floor(0.5 + angle/radius) * radius - angle) * length(px) * 0.5;

    let color = vec3(1.0, 0.6392, 0.2078) * fract(d, 8.0);
    return vec4<f32>(color, 1.0);
}
