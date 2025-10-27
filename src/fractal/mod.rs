use bytemuck::{Pod, Zeroable};

const RECURSION_LIMIT: usize = 256;

moddef::moddef!(
    mod {
        wgsl_bindgen
    },
    flat(pub) mod {
        mandelbrot
    }
);

use wgsl_bindgen::global_bindings;

pub use global_bindings::GlobalUniforms;
pub use global_bindings::WgpuBindGroup0;
pub use global_bindings::WgpuBindGroup0Entries;
pub use global_bindings::WgpuBindGroup0EntriesParams;
pub use global_bindings::VertexInput;

unsafe impl Pod for GlobalUniforms {}
unsafe impl Zeroable for GlobalUniforms {}

unsafe impl Pod for VertexInput {}
unsafe impl Zeroable for VertexInput {}

pub trait Fractal
{
    const LABEL: &str;

    fn setup_render_pipeline(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat
    ) -> wgpu::RenderPipeline;
}