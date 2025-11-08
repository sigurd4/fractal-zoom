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

fn cdiv(lhs: vec2<f32>, rhs: vec2<f32>) -> vec2<f32>
{
    return mat2x2(lhs.x, lhs.y, lhs.y, -lhs.x)*rhs/norm_sqr(rhs);
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

fn croot(c: vec2<f32>) -> vec2<f32>
{
    var b = bitcast<vec2<u32>>(c);
    let mask = u32(1) << 31;
    
    for(var i = u32(0); i < 32; i++)
    {
        b.y ^= mask & ((b.x << i) ^ (b.y << i));
    }
    return bitcast<vec2<f32>>(b);
    return c;
}