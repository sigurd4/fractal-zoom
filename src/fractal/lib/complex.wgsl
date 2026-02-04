fn conj(z: vec2<f64>) -> vec2<f64>
{
    return vec2(
        z.x,
        -z.y
    );
}

fn cis(rot: f64) -> vec2<f64>
{
    return vec2(
        cos(rot),
        sin(rot)
    );
}

fn cmul(lhs: vec2<f64>, rhs: vec2<f64>) -> vec2<f64>
{
    return mat2x2(lhs.x, lhs.y, -lhs.y, lhs.x)*rhs;
}

fn cdiv(lhs: vec2<f64>, rhs: vec2<f64>) -> vec2<f64>
{
    return mat2x2(lhs.x, lhs.y, lhs.y, -lhs.x)*rhs/norm_sqr(rhs);
}

fn norm_sqr(x: vec2<f64>) -> f64
{
    return dot(x, x);
}

fn norm(x: vec2<f64>) -> f64
{
    return sqrt(norm_sqr(x));
}

fn arg(x: vec2<f64>) -> f64
{
    return atan2(x.y, x.x);
}

fn clog(x: vec2<f64>) -> vec2<f64>
{
    return vec2(
        log(norm_sqr(x))/2.0,
        arg(x)
    );
}

fn cexp(x: vec2<f64>) -> vec2<f64>
{
    return exp(x.x)*cis(x.y);
}

fn powc(x: vec2<f64>, y: vec2<f64>) -> vec2<f64>
{
    if x.x == 0.0 && x.y == 0.0
    {
        return vec2(0.0, 0.0);
    }
    return cexp(cmul(y, clog(x)));
}