use std::{fs::File, io::{Read, Write}};

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
        .add_entry_point("src/fractal/fibonacci_snowflake.wgsl")
        .add_entry_point("src/fractal/julia.wgsl")
        .add_entry_point("src/fractal/pendulum.wgsl")
        .add_entry_point("src/fractal/rauzy.wgsl")
        .add_entry_point("src/fractal/heighway_dragon.wgsl")
        .add_entry_point("src/fractal/henon.wgsl")
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

    // Open and read the file entirely
    let mut src = File::open("src/fractal/wgsl_bindgen.rs")
        .into_diagnostic()?;
    let mut data = String::new();
    src.read_to_string(&mut data)
        .into_diagnostic()?;
    drop(src);  // Close the file early

    // Run the replace operation in memory
    let new_data = data.replace("push_constant_ranges: &[]", "immediate_size: 0");

    // Recreate the file and dump the processed contents to it
    let mut dst = File::create("src/fractal/wgsl_bindgen.rs")
        .into_diagnostic()?;
    dst.write(new_data.as_bytes())
        .into_diagnostic()?;

    Ok(())
}