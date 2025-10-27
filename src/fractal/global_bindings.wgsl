struct GlobalUniforms {
    view: mat2x3<f32>,
    exp: vec2<f32>
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @builtin(vertex_index) vertex_index: u32
};

@group(0) @binding(0)
var<uniform> globals: GlobalUniforms;