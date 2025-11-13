#import global_bindings::{GlobalUniforms, VertexInput, wrap, globals, max_iterations, view_radius, epsilon};
#import colormap::colormap3;
#import complex::{cmul, cis, norm_sqr, norm, powc}
#import consts::PI;

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

    var z = cmul(pos/globals.zoom, cis(globals.rot)) - globals.center;
    let z0 = z;
    let gamma = vec2(atan(1.0/globals.shift.x)/PI + 0.5, atan(1.0/globals.shift.y)/PI + 0.5);
    
    let n = u32(max_iterations());
    var i: u32 = 0;
    for(; i < n;)
    {
        z = vec2(
            wrap(z.x + 1.0/2.0, 1.0),
            wrap(z.y + 1.0/2.0, 1.0)
        );
        // This is not entirely correct
        if z.x > gamma.x || z.x < 1.0 - gamma.x || gamma.x == 0.0 || z.y > gamma.y || z.y < 1.0 - gamma.y || gamma.y == 0.0
        {
            break;
        }
        z /= gamma;
        i++;
    }
    let m = f32(i);// - log(log(norm(z)))/log(norm(globals.exp));

    return colormap3(z0, m);
}