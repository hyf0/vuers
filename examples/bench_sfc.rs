use std::fs;
use std::time::Instant;
use libvue_compiler_sfc::bindings::{self, ParseResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("examples/fixtures/App.vue")?;
    let filename = "App.vue";
    let scope_id = "benchmark123";

    const WARMUP: usize = 50;
    const ITERATIONS: usize = 500;

    println!("=== SFC Compilation Benchmark (Length-based FFI) ===\n");
    println!("Warmup: {} iterations", WARMUP);
    println!("Benchmark: {} iterations\n", ITERATIONS);

    // Warmup
    for _ in 0..WARMUP {
        let _ = compile(&source, filename, scope_id);
    }

    // Benchmark
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = compile(&source, filename, scope_id)?;
    }
    let duration = start.elapsed();
    let per_op = duration / ITERATIONS as u32;
    
    println!("Total: {:?}", duration);
    println!("Per operation: {:?}", per_op);
    println!("Throughput: {:.0} ops/sec", ITERATIONS as f64 / duration.as_secs_f64());

    Ok(())
}

fn compile(source: &str, filename: &str, scope_id: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let parsed = ParseResult::parse(source, filename)?;
    let desc = parsed.descriptor().ok_or("No descriptor")?;
    let script_result = desc.compile_script(scope_id, false)?;
    let bindings = script_result.bindings();

    let template_code = if let Some(tmpl) = desc.template() {
        let result = bindings::compile_template(
            tmpl.content(),
            filename,
            scope_id,
            desc.has_scoped_style(),
            bindings.as_ref(),
        )?;
        result.code().to_string()
    } else {
        String::new()
    };

    let mut css_parts = Vec::new();
    for style in desc.styles() {
        let result = bindings::compile_style(
            style.content(),
            filename,
            scope_id,
            style.is_scoped(),
        )?;
        css_parts.push(result.code().to_string());
    }

    Ok((
        format!("{}\n{}", script_result.content(), template_code),
        css_parts.join("\n"),
    ))
}
