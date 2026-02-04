#import global_bindings::{GlobalUniforms, VertexInput, globals, max_iterations, view_radius, epsilon};
#import colormap::colormap3;
#import complex::{cmul, cis, norm_sqr, norm, powc}

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

    var z = globals.shift;
    let c = cmul(pos/globals.zoom, cis(globals.rot)) - globals.center;
    let r = max(max(1.0, norm_sqr(z)), norm_sqr(c));
    
    let n = u32(max_iterations());
    var i: u32 = 0;
    for(; i < n && norm_sqr(z) < r*4.0; i++)
    {
        z = cmul(z - c, powc(z, globals.exp)) - c;
    }
    let m = f32(i) - log(log(norm(z)))/log(norm(globals.exp));

    return colormap3(z, m);
}