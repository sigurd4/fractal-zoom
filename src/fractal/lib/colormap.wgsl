#import global_bindings::{wrap, max_iterations};
#import color::hsl2rgb;
#import complex::{norm, arg};
#import consts::{PI, TAU};

fn colormap4(z: vec2<f32>) -> vec4<f32>
{
    let light = 0.25 + wrap(z.x/TAU + 0.25, 1.0)*0.5;
    let hue = wrap(z.y/TAU, 1.0);

    return vec4(
        hsl2rgb(vec3(hue, 0.5, light)),
        0.8
    );
}

fn colormap3(z: vec2<f32>, i: f32) -> vec4<f32>
{
    let t = clamp(i/max_iterations() % 1.0, 0.0, 1.0);

    let z_norm = 1.0 - exp(-f32(norm(z)));
    let hue = f32(arg(z))/radians(360) + 0.5;

    return vec4(
        hsl2rgb(vec3(hue, z_norm/2.0, t)),
        0.8
    );
}

fn colormap2(z: vec2<f32>, i: f32) -> vec4<f32>
{
    let t = clamp(i/max_iterations(), 0.0, 1.0);

    let hue = f32(arg(z))/radians(360) + 0.5;

    return vec4(
        hsl2rgb(vec3(hue, 0.5, t)),
        0.8
    );
}

fn colormap1(z: vec2<f32>, i: f32) -> vec4<f32>
{
    let z_norm = f32(norm(z));

    let hue = f32(arg(z))/radians(360) + 0.5;

    return vec4(
        hsl2rgb(vec3(hue, 0.5, z_norm % 1.0)),
        1.0
    );
}