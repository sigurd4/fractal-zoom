#import global_bindings::{GlobalUniforms, VertexInput, globals, norm_sqr, powc, norm, cis, cmul, dpowc, dnorm_sqr, dnorm, arg, darg, hsl2rgb, max_iterations};

@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32>
{
    let corner = in.vertex_index % 3;
    let n = in.vertex_index/3 % 2 == 1;
    let pos = vec2(
        f32(u32(corner == 1 || (corner == 0 && n))*globals.window_size.x) - f32(globals.window_size.x)/2.0,
        f32(u32(corner == 2 || (corner == 0 && n))*globals.window_size.y) - f32(globals.window_size.y)/2.0
    );

    return vec4<f32>(pos, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32>
{
    let pos = position.xy/position.w - vec2(f32(globals.window_size.x), f32(globals.window_size.y))/2.0;
    let c = cmul(pos/globals.zoom, cis(globals.rot)) - globals.center;
    var z = c;
    var i: u32 = 0;
    let n = u32(max_iterations());
    for(; i < n && norm_sqr(z) < 4.0; i++)
    {
        z = (powc(z, globals.exp) + c);
    }

    return colormap2(z, i);
}

fn colormap2(z: vec2<f32>, i: u32) -> vec4<f32>
{
    let t = clamp(f32(i)/max_iterations(), 0.0, 1.0);

    let hue = f32(arg(z))/radians(360) + 0.5;

    return vec4(
        hsl2rgb(vec3(hue, 0.5, t)),
        1.0
    );
}

fn colormap1(z: vec2<f32>, i: u32) -> vec4<f32>
{
    let z_norm = f32(norm(z));

    let hue = f32(arg(z))/radians(360) + 0.5;

    return vec4(
        hsl2rgb(vec3(hue, 0.5, z_norm % 1.0)),
        1.0
    );
}