#import global_bindings::{GlobalUniforms, VertexInput, wrap, globals, max_iterations, view_radius, epsilon};
#import colormap::colormap3;
#import complex::{cmul, cis, norm_sqr, norm, powc, cdiv}
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
    //let phi = vec2(atan(globals.shift.x)/PI + 0.5, atan(globals.shift.y)/PI + 0.5);
    let phi = globals.shift;
    let r = vec2(phi.y - phi.x, 0.0);
    var lambda = r;
    let mul_lambda = powc(r, globals.exp);
    let c = phi.y + phi.x;
    z = cmul(z, lambda);
    
    let n = u32(max_iterations());
    var i: u32 = 0;
    for(; i < n;)
    {
        // 2z = (2k + (φ₂ + φ₁ - rᴷ), 2k + (φ₂ + φ₁ + rᴷ))
        // 2z = (2k + (φ₂ + φ₁ - rᴷ), 2k + (φ₂ + φ₁ + rᴷ))
        z = vec2(
            wrap(2.0*z.x, 2.0),
            wrap(2.0*z.y, 2.0)
        );
        if ((z.x > c - abs(lambda.x) && z.x < c + abs(lambda.x)) != (lambda.x < 0.0))
            || ((z.y > c - abs(lambda.x) && z.y < c + abs(lambda.x)) != (lambda.x < 0.0))
            || lambda.x == 0.0
            || lambda.x == 0.0
        {
            break;
        }
        lambda = cmul(lambda, mul_lambda);
        z = cdiv(z, lambda);
        i++;
    }
    let m = f32(i);// - log(log(norm(z)))/log(norm(globals.exp));

    return colormap3(z0, m);
}