#import complex::{cmul, cis}; 

struct GlobalUniforms {
    time: f32,
    window_size: vec2<u32>,
    max_iterations: f32,
    center: vec2<f32>,
    zoom: f32,
    rot: f32,
    exp: vec2<f32>,
    shift: vec2<f32>,
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
    return f32(globals.max_iterations)*max(1.0, log(f32(globals.zoom)));
}
fn view_radius() -> f32
{
    return 100.0;
}
fn epsilon() -> f32
{
    return 0.00000000001;
}

fn z_in(position: vec4<f32>) -> vec2<f32>
{
    let pos = vec2(f32(position.x), f32(position.y))/f32(position.w) - vec2(f32(globals.window_size.x), f32(globals.window_size.y))/2.0;
    return cmul(pos/f32(globals.zoom), cis(f32(globals.rot))) - vec2(f32(globals.center.x), f32(globals.center.y));
}

fn shift_in() -> vec2<f32>
{
    return vec2(f32(globals.shift.x), f32(globals.shift.y));
}
fn exp_in() -> vec2<f32>
{
    return vec2(f32(globals.exp.x), f32(globals.exp.y));
}