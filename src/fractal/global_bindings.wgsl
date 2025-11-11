struct GlobalUniforms {
    time: f32,
    window_size: vec2<u32>,
    max_iterations: u32,
    center: vec2<f32>,
    zoom: f32,
    rot: f32,
    exp: vec2<f32>
};

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) vertex_id: u32
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) c: vec2<f32>
}

@group(0) @binding(0)
var<uniform> globals: GlobalUniforms;

fn max_iterations() -> f32
{
    return f32(globals.max_iterations)*max(1.0, log(globals.zoom));
}
fn view_radius() -> f32
{
    return 100.0;
}
fn epsilon() -> f32
{
    return 0.00000000001;
}