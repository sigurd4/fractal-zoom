#import global_bindings::{GlobalUniforms, VertexInput, globals, max_iterations, view_radius, epsilon};
#import colormap::colormap3;
#import complex::{cmul, cis, norm_sqr, norm, powc, croot}

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
    let c_norm_sqr = norm_sqr(c);
    var exp_inv = 1.0/globals.exp;
    var z = croot(powc(-c, exp_inv));
    var dz = vec2(1.0, 0.0);
    
    let n = u32(max_iterations());
    var i: u32 = 0;
    for(; i < n && norm_sqr(z) < c_norm_sqr*4.0; i++)
    {
        z = powc(z - c, exp_inv);
        dz = dz*exp_inv/(exp_inv - powc(z - c, vec2(1.0, 0.0) - exp_inv));
    }

    return colormap3(z, i);
}