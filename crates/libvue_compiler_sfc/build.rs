use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get the crate's manifest directory for resolving relative paths
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.join("../..").canonicalize().unwrap();

    let hermes_home = env::var("HERMES_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root.join("hermes"));
    let hermes_build = hermes_home.join("build");

    // Always rerun build script to check if outputs exist
    println!("cargo:rerun-if-changed=build.rs");

    // Create dist directory if needed (at workspace root)
    let dist_dir = workspace_root.join("dist");
    fs::create_dir_all(&dist_dir).expect("Failed to create dist directory");

    // Bundle the Vue compiler JS (always run to ensure it's up to date)
    let tools_dir = workspace_root.join("tools");
    let bundle_status = Command::new("node")
        .args(["--experimental-strip-types", "--no-warnings", "bundle.ts"])
        .current_dir(&tools_dir)
        .status()
        .expect("Failed to run bundle.ts");
    if !bundle_status.success() {
        panic!("Failed to bundle Vue compiler JS");
    }

    // Compile JS to native object with Static Hermes
    let shermes = hermes_build.join("bin/shermes");
    let vue_compiler_o = dist_dir.join("vue-compiler.o");
    let vue_compiler_js = dist_dir.join("vue-compiler.js");
    let shermes_status = Command::new(&shermes)
        .args([
            "-O", "-c", "-exported-unit=vue_compiler",
            "-o", vue_compiler_o.to_str().unwrap(),
            vue_compiler_js.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to run shermes");
    if !shermes_status.success() {
        panic!("Failed to compile Vue compiler with shermes");
    }

    // Compile the C++ wrapper
    cc::Build::new()
        .cpp(true)
        .file(manifest_dir.join("ffi/cpp/runtime.cpp"))
        .file(manifest_dir.join("ffi/cpp/vue_sfc.cpp"))
        .include(manifest_dir.join("ffi/cpp"))
        .include(hermes_home.join("API"))
        .include(hermes_home.join("API/jsi"))
        .include(hermes_home.join("include"))
        .include(hermes_home.join("public"))
        .include(hermes_build.join("lib/config"))
        .flag("-std=c++17")
        .compile("wrapper");

    // Link the compiled Vue compiler object
    println!("cargo:rustc-link-arg={}", vue_compiler_o.display());

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
    println!("cargo:rerun-if-changed={}", manifest_dir.join("ffi/cpp/runtime.h").display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("ffi/cpp/runtime.cpp").display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("ffi/cpp/runtime_internal.h").display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("ffi/cpp/vue_sfc.h").display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("ffi/cpp/vue_sfc.cpp").display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("ffi/js/vue_compiler_sfc_bridge.js").display());
    println!("cargo:rerun-if-changed={}", tools_dir.join("bundle.ts").display());
    println!("cargo:rerun-if-changed={}", tools_dir.join("package.json").display());
}
