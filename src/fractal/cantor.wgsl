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
    //let phi = vec2(atan(globals.shift.x)/PI + 0.5, atan(globals.shift.y)/PI + 0.5);
    let phi = globals.shift;
    let lambda = globals.exp;

    let r = vec2(phi.y - phi.x, phi.y - phi.x)/2.0;
    let r_lambda = powc(r, lambda);
    var r_k = cmul(r, r_lambda);
    let c = (phi.y + phi.x)/2.0;
    
    let nf = max_iterations();
    let n: u32 = u32(nf);
    var i: u32 = 1;
    var d = 0.0;
    for(; i < n; i++)
    {
        // c = φ₂ + φ₁
        // C := C\((k + (c - rᴷ)/2)rᴷⁿ, (k + (c + rᴷ)/2)rᴷⁿ)
        // (z - c) - k \= (-rᴷ/2, rᴷ/2)
        // z := z*drᴷⁿ
        z = vec2(
            (z.x - floor(z.x)),
            (z.y - floor(z.y))
        );
        d += pow(abs(1.0 - 2.0*abs(z.x - 0.5)), nf);
        d += pow(abs(1.0 - 2.0*abs(z.y - 0.5)), nf);
        let e = f32(u32(z.x > c));
        if(z.x <= c)
        {
            if(z.x > c - r_k.x)
            {
                break;
            }
            else
            {
                z.x /= c - r_k.x;
            }
        }
        else
        {
            if(z.x < c + r_k.x)
            {
                break;
            }
            else
            {
                z.x = (z.x - 1.0)/(1.0 - c - r_k.x) + 1.0;
            }
        }
        if(z.y <= c)
        {
            if(z.y > c - r_k.y)
            {
                break;
            }
            else
            {
                z.y /= c - r_k.y;
            }
        }
        else
        {
            if(z.y < c + r_k.y)
            {
                break;
            }
            else
            {
                z.y = (z.y - 1.0)/(1.0 - c - r_k.y) + 1.0;
            }
        }
        r_k = cmul(r_k, r_lambda);
    }
    let m = f32(i) - sqrt(d);

    return colormap3(z, m);
}