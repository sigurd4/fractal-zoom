fn hsl2rgb(c: vec3<f32>) -> vec3<f32>
{
    let rgb = clamp( abs(((c.x*6.0 + vec3(0.0,4.0,2.0)) % 6.0) - 3.0) - 1.0, vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0) );

    return c.z + c.y * (rgb-0.5)*(1.0 - abs(2.0*c.z-1.0));
}