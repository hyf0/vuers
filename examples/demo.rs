use libvue_compiler_sfc::compile_template;

fn main() {
    let template = r#"<div class="container">
  <span>{{ msg }}</span>
  <button @click="handleClick">Click me</button>
</div>"#;

    println!("=== Vue Template Compiler (Static Hermes) ===\n");

    println!("Input:");
    println!("------");
    println!("{}\n", template);

    match compile_template(template, "template.vue", "demo", false, None) {
        Ok(result) => {
            println!("Output:");
            println!("-------");
            println!("{}", result.code());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
