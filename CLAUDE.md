# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rusty-vue compiles the Vue template compiler (@vue/compiler-dom) to native code using Static Hermes, making it callable from Rust without a JavaScript runtime.

## Build Commands

### Prerequisites
- Initialize the hermes submodule and build it (see spec.md for build instructions):
  ```bash
  git submodule update --init
  cd hermes && cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release && cmake --build ./build
  ```
- `HERMES_HOME` defaults to `./hermes` (the submodule)

### Building
```bash
# Install JS dependencies
pnpm install

# Bundle Vue compiler to single JS file
node scripts/bundle.mjs

# Compile JS to native object with Static Hermes
$HERMES_HOME/build/bin/shermes -O -c -exported-unit=vue_compiler dist/vue-compiler.js

# Build Rust library and binary
cargo build --release
```

### Running
```bash
# Run benchmarks
cargo run --release

# Run Node.js comparison benchmarks
node scripts/benchmark-node.mjs
node scripts/benchmark-pure-js.mjs
```

## Architecture

The build pipeline has 4 stages:

1. **Bundle JS** (`scripts/bundle.mjs`): Uses Rolldown to bundle `@vue/compiler-sfc` into a single `dist/vue-compiler.js` file with no external dependencies

2. **Compile to Native** (`shermes`): Static Hermes compiles the bundled JS to a native object file `dist/vue-compiler.o`

3. **C++ Wrapper** (`src/wrapper.cpp`): Provides FFI interface between Rust and the Hermes runtime. Exposes `vue_compile_template()` and `vue_compile_batch()` functions. Caches the Hermes runtime and JS function references for performance.

4. **Rust Crate** (`src/lib.rs`): Safe Rust API wrapping the FFI calls. Exposes `compile_template()` and `compile_batch()` functions.

### Key Files
- `src/vue-compiler.js` - JS entry point exposing `compile` and `compileBatch` on globalThis
- `src/wrapper.cpp` - C++ FFI bridge using Hermes JSI API
- `src/lib.rs` - Rust public API
- `build.rs` - Cargo build script linking Hermes libraries
- `spec.md` - Detailed architecture and benchmark documentation

### Linking
The `build.rs` links against:
- `dist/vue-compiler.o` (compiled Vue compiler)
- Hermes static libraries: `shermes_console_a`, `hermesvm_a`, `jsi`, `boost_context`
- System libraries: `c++`, `Foundation` framework (macOS)
