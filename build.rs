use wgsl_bindgen::{WgslBindgenOptionBuilder, WgslShaderSourceType, WgslTypeSerializeStrategy};

fn main() -> anyhow::Result<()>
{
    println!("cargo:rerun-if-changed=NULL");
    WgslBindgenOptionBuilder::default()
        .workspace_root("src/fractal")
        .add_entry_point("src/fractal/mandelbrot.wgsl")
        .serialization_strategy(WgslTypeSerializeStrategy::Bytemuck)
        .emit_rerun_if_change(true)
        //.shader_source_type(WgslShaderSourceType::ComposerWithRelativePath)
        .type_map(wgsl_bindgen::GlamWgslTypeMap)
        .output("src/fractal/wgsl_bindgen.rs")
        .build()?
        .generate()?;
    Ok(())
}