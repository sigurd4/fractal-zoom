use wgsl_bindgen::{WgslBindgenOptionBuilder, WgslTypeSerializeStrategy};

fn main() -> anyhow::Result<()>
{
    WgslBindgenOptionBuilder::default()
        .workspace_root("src/fractal")
        .add_entry_point("src/fractal/mandelbrot.wgsl")
        .serialization_strategy(WgslTypeSerializeStrategy::Encase)
        .emit_rerun_if_change(true)
        .type_map(wgsl_bindgen::NalgebraWgslTypeMap)
        .output("src/fractal/wgsl_bindgen.rs")
        .build()?
        .generate()?;
    Ok(())
}