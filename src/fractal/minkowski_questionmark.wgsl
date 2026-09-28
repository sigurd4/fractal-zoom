#import global_bindings::{GlobalUniforms, VertexInput, z_in, shift_in, exp_in, globals, max_iterations, view_radius, epsilon};
#import colormap::colormap3;
#import complex::{cmul, cexp, cinv, cdiv, conj, cis, norm_sqr, norm, powc};
#import consts::{ln_2, pi};

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
    let x = z_in(position);
    var z = x + vec2(0.5, 0.5);
    var a = vec2(floor(z.x), floor(z.y));
    var u = vec2(fract(z.x), fract(z.y));
    z = cinv(u);
    var y = a;
    var t = shift_in();
    let e = exp_in();
    let r = max(1.0, norm_sqr(y));
    
    let n = u32(max_iterations());
    var i: u32 = 0;
    for(; i < n && norm_sqr(z) < 4.0*r; i++)
    {
        a = vec2(floor(z.x), floor(z.y));
        u = vec2(fract(z.x), fract(z.y));
        z = cinv(u);

        t = cmul(-t, powc(e, -a));
        y += t;
    }

    let m = f32(f32(i) - log(log(norm(y))));
    let yy = vec2(f32(y.x), f32(y.y));

    return colormap3(yy, m);
}
