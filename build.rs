use miette::IntoDiagnostic;
use wgsl_bindgen::{AdditionalScanDirectory, WgslBindgenOptionBuilder, WgslShaderIrCapabilities, WgslTypeSerializeStrategy};

fn main() -> miette::Result<()>
{
    WgslBindgenOptionBuilder::default()
        .workspace_root("src/fractal")
        .additional_scan_dirs(vec![AdditionalScanDirectory::from((None, "src/fractal/lib"))])
        .add_entry_point("src/fractal/blancmange.wgsl")
        .add_entry_point("src/fractal/cantor.wgsl")
        .add_entry_point("src/fractal/feigenbaum.wgsl")
        .add_entry_point("src/fractal/mandelbrot.wgsl")
        .add_entry_point("src/fractal/fibonacci_hamiltonian.wgsl")
        .add_entry_point("src/fractal/julia.wgsl")
        .add_entry_point("src/fractal/pendulum.wgsl")
        .add_entry_point("src/fractal/supergolden.wgsl")
        .serialization_strategy(WgslTypeSerializeStrategy::Bytemuck)
        .emit_rerun_if_change(true)
        //.shader_source_type(WgslShaderSourceType::ComposerWithRelativePath)
        .ir_capabilities(WgslShaderIrCapabilities::FLOAT64)
        .type_map(wgsl_bindgen::GlamWgslTypeMap)
        .output("src/fractal/wgsl_bindgen.rs")
        .build()?
        .generate()
        .into_diagnostic()?;
    Ok(())
}