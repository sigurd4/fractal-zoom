#import global_bindings::{GlobalUniforms, VertexInput, wrap, globals, max_iterations, view_radius, epsilon};
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

    let c = globals.shift;
    var z = cmul(pos/globals.zoom, cis(globals.rot)) - globals.center;
    let z0 = z;
    
    let n = u32(max_iterations());
    var i: u32 = 0;
    for(; i < n;)
    {
        z = z*3.0;
        z = vec2(
            wrap(z.x + 1.5, 3.0),
            wrap(z.y + 1.5, 3.0)
        );
        // This is not entirely correct
        if u32(z.x) == 1 || u32(z.y) == 1
        {
            break;
        }
        if i == 0
        {

        }
        i++;
    }
    let m = f32(i);// - log(log(norm(z)))/log(norm(globals.exp));

    return colormap3(z0, m);
}