#import global_bindings::{GlobalUniforms, VertexInput, z_in, shift_in, exp_in, wrap, globals, max_iterations, view_radius, epsilon};
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
    var z = z_in(position);
    let phi = shift_in();
    let lambda = exp_in();

    let r = vec2(phi.y - phi.x, phi.y - phi.x)/2.0;
    let r_lambda = powc(r, lambda);
    var r_k = cmul(r, r_lambda);
    let c = (phi.y + phi.x)/2.0;
    
    let nf = max_iterations();
    let n: u32 = u32(nf);
    var i: u32 = 1;
    var d: f64 = 0.0;
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
        //d += pow(abs(1.0 - 2.0*abs(z.x - 0.5)), nf);
        //d += pow(abs(1.0 - 2.0*abs(z.y - 0.5)), nf);
        var ok = false;
        var stop = false;
        z.x = cantor_dim(z.x, c, r_k.x, &ok, &stop);
        z.y = cantor_dim(z.y, c, r_k.y, &ok, &stop);
        if(!ok || stop)
        {
            break;
        }
        r_k = cmul(r_k, r_lambda);
    }
    let m = f32(i) - f32(sqrt(d));
    let zz = vec2(f32(z.x), f32(z.y));

    return vec4(m, m, m, 1.0);
}

fn cantor_dim(z: f64, c: f64, r: f64, ok: ptr<function, bool>, stop: ptr<function, bool>) -> f64
{
    let s = r < 0; // sierpinski carpet
    if(z <= c)
    {
        if(z > c - abs(r))
        {
            if(!s)
            {
                *stop = true;
            }
            return (z - (c - abs(r)))/(2.0*abs(r));
        }
        else
        {
            *ok = true;
            return (z - (c - 1.0 + abs(r)))/(0.5 - abs(r));
        }
    }
    else
    {
        if(z < c + abs(r))
        {
            if(!s)
            {
                *stop = true;
            }
            return (z - (c - abs(r)))/(2.0*abs(r));
        }
        else
        {
            *ok = true;
            return (z - (c + abs(r)))/(0.5 - abs(r));
        }
    }
}