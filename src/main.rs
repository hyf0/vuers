use rusty_vue::{compile_template};

fn main() {
    let template = r#"<div class="container"><span>{{ msg }}</span><button @click="handleClick">Click me</button></div>"#;

    let ret = compile_template(template);
    println!("Single compilation result: {:?}", ret);
}
