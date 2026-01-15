use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let hermes_home = env::var("HERMES_HOME")
        .unwrap_or_else(|_| "./hermes".to_string());
    let hermes_path = PathBuf::from(&hermes_home);
    let hermes_build = hermes_path.join("build");

    // Always rerun build script to check if outputs exist
    println!("cargo:rerun-if-changed=build.rs");

    // Create dist directory if needed
    fs::create_dir_all("dist").expect("Failed to create dist directory");

    // Bundle the Vue compiler JS (always run to ensure it's up to date)
    let bundle_status = Command::new("node")
        .args(["--experimental-strip-types", "--no-warnings", "bundle.ts"])
        .current_dir("tools")
        .status()
        .expect("Failed to run bundle.ts");
    if !bundle_status.success() {
        panic!("Failed to bundle Vue compiler JS");
    }

    // Compile JS to native object with Static Hermes
    let shermes = hermes_build.join("bin/shermes");
    let shermes_status = Command::new(&shermes)
        .args(["-O", "-c", "-exported-unit=vue_compiler", "-o", "dist/vue-compiler.o", "dist/vue-compiler.js"])
        .status()
        .expect("Failed to run shermes");
    if !shermes_status.success() {
        panic!("Failed to compile Vue compiler with shermes");
    }

    // Compile the C++ wrapper
    cc::Build::new()
        .cpp(true)
        .file("ffi/cpp/runtime.cpp")
        .file("ffi/cpp/vue_sfc.cpp")
        .include("ffi/cpp")
        .include(hermes_path.join("API"))
        .include(hermes_path.join("API/jsi"))
        .include(hermes_path.join("include"))
        .include(hermes_path.join("public"))
        .include(hermes_build.join("lib/config"))
        .flag("-std=c++17")
        .compile("wrapper");

    // Link the compiled Vue compiler object
    println!("cargo:rustc-link-arg=dist/vue-compiler.o");

    // Link Hermes libraries
    println!("cargo:rustc-link-search={}", hermes_build.join("lib").display());
    println!("cargo:rustc-link-search={}", hermes_build.join("jsi").display());
    println!("cargo:rustc-link-search={}", hermes_build.join("tools/shermes").display());
    println!("cargo:rustc-link-search={}", hermes_build.join("external/boost/boost_1_86_0/libs/context").display());

    // Link required libraries in order
    println!("cargo:rustc-link-lib=static=shermes_console_a");
    println!("cargo:rustc-link-lib=static=hermesvm_a");
    println!("cargo:rustc-link-lib=static=jsi");
    println!("cargo:rustc-link-lib=static=boost_context");

    // Link system libraries
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=framework=Foundation");

    // Rerun if sources change
    println!("cargo:rerun-if-changed=ffi/cpp/runtime.h");
    println!("cargo:rerun-if-changed=ffi/cpp/runtime.cpp");
    println!("cargo:rerun-if-changed=ffi/cpp/runtime_internal.h");
    println!("cargo:rerun-if-changed=ffi/cpp/vue_sfc.h");
    println!("cargo:rerun-if-changed=ffi/cpp/vue_sfc.cpp");
    println!("cargo:rerun-if-changed=ffi/js/vue_compiler_sfc_bridge.js");
    println!("cargo:rerun-if-changed=tools/bundle.ts");
    println!("cargo:rerun-if-changed=tools/package.json");
}
