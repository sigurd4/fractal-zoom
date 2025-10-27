#import global_bindings::{GlobalUniforms, VertexInput, globals, norm_sqr, powc, norm}

@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32>
{
    let pos= in.position*globals.view;
    return vec4<f32>(pos.x, pos.y, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32>
{
    let c = position.xy/position.w;
    var z = c;
    for(var i = 0; i < 1024 && norm_sqr(z) < 4.0; i++)
    {
        z = powc(z, globals.exp) + c;
    }

    let z_norm = norm(z);
    let color = vec2(z_norm + z.x, z_norm + z.y);

    return vec4(
        color.x,
        z_norm*2.0 - (color.x + color.y)/2.0,
        color.y,
        z_norm*2.0
    );
}