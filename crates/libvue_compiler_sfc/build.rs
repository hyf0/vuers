use std::env;
use std::path::PathBuf;

fn main() {
    // Get workspace root to find the compiled Vue compiler object
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.join("../..").canonicalize().unwrap();
    let vue_compiler_o = workspace_root.join("dist/vue-compiler.o");

    // Link the compiled Vue compiler object (needed for test binaries)
    println!("cargo:rustc-link-arg={}", vue_compiler_o.display());
}
