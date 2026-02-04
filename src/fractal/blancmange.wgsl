#import global_bindings::{GlobalUniforms, VertexInput, z_in, shift_in, exp_in, globals, max_iterations, view_radius, epsilon};
#import colormap::colormap3;
#import complex::{cmul, cis, norm_sqr, norm, powc, cdiv, arg}

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
    var c = z_in(position);
    var z = shift_in();
    let e = exp_in();
    let r = max(max(1.0, norm_sqr(z)), norm_sqr(c));
    
    let n = u32(max_iterations());
    var i: u32 = 0;
    for(; i < n && norm_sqr(z) < r*4.0; i++)
    {
        c /= 2.0;
        z = cdiv(z, e) + vec2(triangle(c.x), triangle(c.y));
    }
    let m = f32(i) - f32(log(log(norm(z)))/log(1.0/norm(e)));
    let zz = vec2(f32(z.x), f32(z.y));

    return colormap3(zz, m);
}

fn triangle(r: f64) -> f64
{
    return 2.0*abs(2.0*(r - floor(r + 0.5))) - 1.0;
}