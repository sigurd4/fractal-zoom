#import global_bindings::{GlobalUniforms, VertexInput, globals}

@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32>
{
    return vec4<f32>(vec3<f32>(in.position, 1.0)*globals.view, 0.0, 1.0);
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

fn cis(rot: f32) -> vec2<f32>
{
    return vec2(
        cos(rot),
        sin(rot)
    );
}

fn cmul(lhs: vec2<f32>, rhs: vec2<f32>) -> vec2<f32>
{
    return vec2(
        lhs.x*rhs.x - lhs.y*rhs.y,
        lhs.x*lhs.y + lhs.y*rhs.x
    );
}

fn norm_sqr(x: vec2<f32>) -> f32
{
    return x.x*x.x + x.y*x.y;
}

fn norm(x: vec2<f32>) -> f32
{
    return sqrt(norm_sqr(x));
}

fn arg(x: vec2<f32>) -> f32
{
    return atan2(x.y, x.x);
}

fn clog(x: vec2<f32>) -> vec2<f32>
{
    return vec2(
        log(norm_sqr(x))/2.0,
        arg(x)
    );
}

fn cexp(x: vec2<f32>) -> vec2<f32>
{
    return exp(x.x)*cis(x.y);
}

fn powc(x: vec2<f32>, y: vec2<f32>) -> vec2<f32>
{
    return cexp(y*clog(x));
}