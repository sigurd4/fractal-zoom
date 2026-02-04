fn conj(z: vec2<f32>) -> vec2<f32>
{
    return vec2(
        z.x,
        -z.y
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
    if x.x == 0.0 && x.y == 0.0
    {
        return vec2(0.0, 0.0);
    }
    return cexp(cmul(y, clog(x)));
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
}