# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

vue-compiler compiles the Vue template compiler (@vue/compiler-dom) to native code using Static Hermes, making it callable from Rust without a JavaScript runtime.

## Build Commands

### Prerequisites
- Install [just](https://github.com/casey/just) command runner
- Install [Ninja](https://ninja-build.org/) build system
- `HERMES_HOME` defaults to `./hermes` (the submodule)

### Building
```bash
# Set up everything (first time only)
just setup

# Build the project
just build
```

### Running
```bash
# Run demo
just run

# Run all benchmarks
just bench

# Individual benchmarks
just bench-native
just bench-node
just bench-pure
```

## Architecture

The build pipeline has 4 stages:

1. **Bundle JS** (`tools/bundle.ts`): Uses Rolldown to bundle `@vue/compiler-sfc` into a single `dist/vue-compiler.js` file with no external dependencies

2. **Compile to Native** (`shermes`): Static Hermes compiles the bundled JS to a native object file `dist/vue-compiler.o`

3. **C++ Wrapper** (`ffi/wrapper.cpp`): Provides FFI interface between Rust and the Hermes runtime. Exposes `vue_compile_template()` and `vue_compile_batch()` functions. Caches the Hermes runtime and JS function references for performance.

4. **Rust Crate** (`src/lib.rs`): Safe Rust API wrapping the FFI calls. Exposes `compile_template()` and `compile_batch()` functions.

## Project Structure

```
libvue_compiler_sfc/
├── src/                    # Rust source
│   └── lib.rs              # Public API
├── examples/               # Example programs
│   └── demo.rs             # Demo/benchmark
├── ffi/                    # FFI bridge layer
│   ├── wrapper.cpp         # C++ bridge to Hermes
│   └── vue-compiler.js     # JS entry (compiled to native)
├── tools/                  # Build tooling (npm package)
│   ├── package.json
│   ├── bundle.ts           # Rolldown bundler
│   └── benchmark-*.ts      # Benchmark scripts
├── hermes/                 # Git submodule (Static Hermes)
├── dist/                   # Build artifacts
│   ├── vue-compiler.js     # Bundled JS
│   └── vue-compiler.o      # Compiled native object
├── build.rs                # Cargo build script
├── Cargo.toml
├── justfile                # Task runner
└── README.md
```

### Linking
The `build.rs` links against:
- `dist/vue-compiler.o` (compiled Vue compiler)
- Hermes static libraries: `shermes_console_a`, `hermesvm_a`, `jsi`, `boost_context`
- System libraries: `c++`, `Foundation` framework (macOS)
