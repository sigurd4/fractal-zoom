#import global_bindings::{GlobalUniforms, VertexInput, z_in, shift_in, exp_in, globals, max_iterations, view_radius, epsilon};
#import colormap::colormap4;
#import complex::{cmul, cis, norm_sqr, norm, powc}
#import consts::TAU;

const G: f32 = 9.8; // gravitational acceleration (m/s^2)

struct PendulumState {
    theta1: f64,
    theta2: f64,
    p1: f64,
    p2: f64
};

struct Derivatives {
    dtheta1: f64,
    dtheta2: f64,
    dp1: f64,
    dp2: f64
};

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
    let e = exp_in();
    let arg = z_in(position);
    var state = PendulumState(
        arg.x,
        arg.y,
        e.x,
        e.y,
    );
    
    let n = max_iterations();
    let dt = sqrt(f64(globals.time))/n;
    var i: u32 = 0;
    for(; i < u32(n); i++)
    {
        state = rk4_step(state, dt);
    }
    state = rk4_step(state, dt*fract(n));

    let z = pendulum_z(state);
    let zz = vec2(f32(z.x), f32(z.y));
    return colormap4(zz);
}

fn pendulum_z(state: PendulumState) -> vec2<f64>
{
    return vec2(
        sin(state.theta1) + sin(state.theta2),
        cos(state.theta1) + cos(state.theta2)
    );
}

fn rk4_step(state: PendulumState, dt: f64) -> PendulumState
{
    let k1: Derivatives = compute_derivatives(state);

    let s2 = PendulumState(
        state.theta1 + 0.5 * dt * k1.dtheta1,
        state.theta2 + 0.5 * dt * k1.dtheta2,
        state.p1 + 0.5 * dt * k1.dp1,
        state.p2 + 0.5 * dt * k1.dp2
    );

    let k2 = compute_derivatives(s2);

    let s3 = PendulumState(
        state.theta1 + 0.5 * dt * k2.dtheta1,
        state.theta2 + 0.5 * dt * k2.dtheta2,
        state.p1 + 0.5 * dt * k2.dp1,
        state.p2 + 0.5 * dt * k2.dp2
    );

    let k3 = compute_derivatives(s3);

    let s4 = PendulumState(
        state.theta1 + dt * k3.dtheta1,
        state.theta2 + dt * k3.dtheta2,
        state.p1 + dt * k3.dp1,
        state.p2 + dt * k3.dp2
    );

    let k4 = compute_derivatives(s4);

    var newState = PendulumState();
    let dt6 = dt/6.0;
    newState.theta1 = (state.theta1 + dt6 * (k1.dtheta1 + 2 * k2.dtheta1 + 2 * k3.dtheta1 + k4.dtheta1)) % f64(TAU);
    newState.theta2 = (state.theta2 + dt6 * (k1.dtheta2 + 2 * k2.dtheta2 + 2 * k3.dtheta2 + k4.dtheta2)) % f64(TAU);
    newState.p1 = state.p1 + dt6 * (k1.dp1 + 2 * k2.dp1 + 2 * k3.dp1 + k4.dp1);
    newState.p2 = state.p2 + dt6 * (k1.dp2 + 2 * k2.dp2 + 2 * k3.dp2 + k4.dp2);

    return newState;
}

fn compute_derivatives(state: PendulumState) -> Derivatives
{
    let theta1 = state.theta1;
    let theta2 = state.theta2;
    let p1 = state.p1;
    let p2 = state.p2;
    let delta = theta1 - theta2;
    let cos_delta = cos(delta);
    let sin_delta = sin(delta);
    let denominator = 16 - 9 * cos_delta * cos_delta;
    let coeff = 6/(denominator);
    let dtheta1 = coeff * (2 * p1 - 3 * cos_delta * p2);
    let dtheta2 = coeff * (8 * p2 - 3 * cos_delta * p1);
    let coeff2 = -f64(0.5);
    let endbit = dtheta1 * dtheta2 * sin_delta;
    let dp1 = coeff2 * (3 * f64(G) * sin(theta1) + endbit);
    let dp2 = coeff2 * (f64(G) * sin(theta2) - endbit);

    var der = Derivatives();
    let shift = shift_in();
    der.dtheta1 = dtheta1 + shift.x;
    der.dtheta2 = dtheta2 + shift.y;
    der.dp1 = dp1;
    der.dp2 = dp2;
    return der;
}