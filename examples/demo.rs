use rusty_vue::compile_template;

fn main() {
    let template = r#"<div class="container">
  <span>{{ msg }}</span>
  <button @click="handleClick">Click me</button>
</div>"#;

    println!("=== Vue Template Compiler (Static Hermes) ===\n");

    println!("Input:");
    println!("------");
    println!("{}\n", template);

    match compile_template(template) {
        Ok(code) => {
            println!("Output:");
            println!("-------");
            println!("{}", code);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
