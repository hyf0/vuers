//! Compile an SFC file and emit outputs to disk
//!
//! Usage: cargo run --example compile_sfc
//!
//! Output files:
//!   dist/App.js     - Compiled JavaScript module
//!   dist/App.css    - Scoped CSS

use libvue_compiler_sfc::bindings::{self, ParseResult};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input
    let input_path = "examples/fixtures/App.vue";
    let source = fs::read_to_string(input_path)?;
    let filename = "App.vue";

    // Create output directory
    let out_dir = Path::new("dist");
    fs::create_dir_all(out_dir)?;

    // Generate scope ID
    let scope_id = format!("{:x}", hash(filename));

    println!("=== Compiling {} ===\n", filename);

    // 1. Parse SFC
    println!("1. Parsing SFC...");
    let parsed = ParseResult::parse(&source, filename)?;

    if !parsed.is_ok() {
        for err in parsed.errors() {
            eprintln!("   Error: {}", err);
        }
        return Err("Parse failed".into());
    }

    let desc = parsed.descriptor().ok_or("No descriptor")?;
    println!("   - template: {}", desc.has_template());
    println!("   - script_setup: {}", desc.has_script_setup());
    println!("   - styles: {} (scoped: {})", desc.style_count(), desc.has_scoped_style());

    // 2. Compile script
    println!("\n2. Compiling script...");
    let script_result = desc.compile_script(&scope_id, false)?;
    let bindings = script_result.bindings();
    println!("   - content length: {} bytes", script_result.content().len());
    println!("   - has bindings: {}", bindings.is_some());

    // 3. Compile template
    println!("\n3. Compiling template...");
    let template_result = if let Some(tmpl) = desc.template() {
        let result = bindings::compile_template(
            tmpl.content(),
            filename,
            &scope_id,
            desc.has_scoped_style(),
            bindings.as_ref(),
        )?;
        println!("   - code length: {} bytes", result.code().len());
        println!("   - errors: {}", result.error_count());
        Some(result)
    } else {
        None
    };

    // 4. Compile styles
    println!("\n4. Compiling styles...");
    let mut css_parts = Vec::new();
    for (i, style) in desc.styles().enumerate() {
        let result = bindings::compile_style(
            style.content(),
            filename,
            &scope_id,
            style.is_scoped(),
        )?;
        println!("   - style[{}]: {} bytes, scoped={}", i, result.code().len(), style.is_scoped());
        css_parts.push(result.code().to_string());
    }

    // 5. Write outputs
    println!("\n5. Writing outputs...");

    let js_output = assemble_js(
        script_result.content(),
        template_result.as_ref().map(|t| t.code()).unwrap_or(""),
        &scope_id,
    );
    let js_path = out_dir.join("App.js");
    fs::write(&js_path, &js_output)?;
    println!("   - {} ({} bytes)", js_path.display(), js_output.len());

    let css_output = css_parts.join("\n\n");
    let css_path = out_dir.join("App.css");
    fs::write(&css_path, &css_output)?;
    println!("   - {} ({} bytes)", css_path.display(), css_output.len());

    println!("\n=== Done ===");
    Ok(())
}

fn assemble_js(script_content: &str, template_code: &str, scope_id: &str) -> String {
    let script_content = script_content.replace("export default", "const __default__ =");
    format!(
        r#"/* Compiled from App.vue */

{script_content}

{template_code}

__default__.render = render
__default__.__scopeId = "data-v-{scope_id}"

export default __default__
"#
    )
}

fn hash(s: &str) -> u32 {
    s.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
}
