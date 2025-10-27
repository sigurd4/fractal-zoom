struct GlobalUniforms {
    view: mat3x3<f32>,
    exp: vec2<f32>
};

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>
};

@group(0) @binding(0)
var<uniform> globals: GlobalUniforms;

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