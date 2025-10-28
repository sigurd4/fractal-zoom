struct GlobalUniforms {
    window_size: vec2<u32>,
    center: vec2<f32>,
    zoom: f32,
    rot: f32,
    exp: vec2<f32>
};

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) vertex_id: u32
};

@group(0) @binding(0)
var<uniform> globals: GlobalUniforms;

fn hsl2rgb(c: vec3<f32>) -> vec3<f32>
{
    let rgb = clamp( abs(((c.x*6.0 + vec3(0.0,4.0,2.0)) % 6.0) - 3.0) - 1.0, vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0) );

    return c.z + c.y * (rgb-0.5)*(1.0-abs(2.0*c.z-1.0));
}

fn cis(rot: f32) -> vec2<f32>
{
    return vec2(
        cos(rot),
        sin(rot)
    );
}
fn dcis(rot: f64) -> vec2<f64>
{
    return vec2(
        cos(rot),
        sin(rot)
    );
}

fn cmul(lhs: vec2<f32>, rhs: vec2<f32>) -> vec2<f32>
{
    return mat2x2(lhs.x, lhs.y, -lhs.y, lhs.x)*rhs;
}
fn dcmul(lhs: vec2<f64>, rhs: vec2<f64>) -> vec2<f64>
{
    return mat2x2(lhs.x, lhs.y, -lhs.y, lhs.x)*rhs;
}

fn norm_sqr(x: vec2<f32>) -> f32
{
    return dot(x, x);
}
fn dnorm_sqr(x: vec2<f64>) -> f64
{
    return dot(x, x);
}

fn norm(x: vec2<f32>) -> f32
{
    return sqrt(norm_sqr(x));
}
fn dnorm(x: vec2<f64>) -> f64
{
    return sqrt(dnorm_sqr(x));
}

fn arg(x: vec2<f32>) -> f32
{
    return atan2(x.y, x.x);
}
fn darg(x: vec2<f64>) -> f64
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
fn dclog(x: vec2<f64>) -> vec2<f64>
{
    return vec2(
        log(dnorm_sqr(x))/2.0,
        darg(x)
    );
}

fn cexp(x: vec2<f32>) -> vec2<f32>
{
    return exp(x.x)*cis(x.y);
}
fn dcexp(x: vec2<f64>) -> vec2<f64>
{
    return exp(x.x)*dcis(x.y);
}

fn powc(x: vec2<f32>, y: vec2<f32>) -> vec2<f32>
{
    return cexp(cmul(y, clog(x)));
}
fn dpowc(x: vec2<f64>, y: vec2<f64>) -> vec2<f64>
{
    return dcexp(dcmul(y, dclog(x)));
}