use miette::IntoDiagnostic;
use wgsl_bindgen::{WgslBindgenOptionBuilder, WgslShaderIrCapabilities, WgslTypeSerializeStrategy};

fn main() -> miette::Result<()>
{
    WgslBindgenOptionBuilder::default()
        .workspace_root("src/fractal")
        .add_entry_point("src/fractal/mandelbrot.wgsl")
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