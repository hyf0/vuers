use libvue_compiler_sfc::Compiler;

fn main() {
    let template = r#"<div class="container">
  <span>{{ msg }}</span>
  <button @click="handleClick">Click me</button>
</div>"#;

    println!("=== Vue Template Compiler (Static Hermes) ===\n");

    println!("Input:");
    println!("------");
    println!("{}\n", template);

    let compiler = Compiler::new().expect("Failed to create compiler");

    let result = compiler.compile_template(template, "template.vue", "demo", false, None);

    match result {
        Ok(output) => {
            println!("Output:");
            println!("-------");
            println!("{}", output.code());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
