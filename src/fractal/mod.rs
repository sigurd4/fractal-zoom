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

pub trait Fractal
{
    const LABEL: &str;

    fn setup_render_pipeline(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat
    ) -> wgpu::RenderPipeline;
}