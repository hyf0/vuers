# AGENTS.md

This file provides guidance when working with code in this repository.

## Project Overview

vue-compiler compiles the Vue template compiler (@vue/compiler-sfc) to native code using Static Hermes, making it callable from Rust without a JavaScript runtime.

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

## Verification

MANDATORY: Always run verification commands after making changes.

- `cargo test` to run all Rust unit tests.
- `cargo clippy` to check for code quality issues.
- `cargo fmt -- --check` to ensure code is properly formatted.

**Tip**: Use Bash sub-agents to run commands in parallel for faster verification.

## Architecture & Data Flow

```
BUILD TIME:
  @vue/compiler-sfc (npm)
         ↓
  Rolldown → dist/vue-compiler.js (bundled, no deps)
         ↓
  shermes → dist/vue-compiler.o (native object)
         ↓
  clang++ → wrapper (runtime.cpp, vue_sfc.cpp)
         ↓
  cargo build → links all with Hermes libraries

RUN TIME:
  Rust app → Compiler::new() → parse()/compile_*() → FFI
         → Hermes runtime → Vue compiler → output
```

## Build Pipeline

The build pipeline has 4 stages:

1. **Bundle JS** (`tools/bundle.ts`): Uses Rolldown to bundle `@vue/compiler-sfc` into a single `dist/vue-compiler.js` file with no external dependencies

2. **Compile to Native** (`shermes -O -c -exported-unit=vue_compiler`): Static Hermes compiles the bundled JS to a native object file `dist/vue-compiler.o`

3. **C++ Wrapper** (`crates/lib_vue_compiler_sfc_sys/ffi/cpp/`): Provides FFI interface between Rust and the Hermes runtime via `runtime.cpp` and `vue_sfc.cpp`. Manages Hermes runtime lifecycle and exposes Vue compiler functions.

4. **Rust Crates**: Two-layer architecture:
   - `lib_vue_compiler_sfc_sys`: Raw unsafe FFI bindings (`build.rs` runs the build)
   - `libvue_compiler_sfc`: Safe Rust API with `Compiler.parse()`, `Compiler.compile_template()`, `Compiler.compile_style()`, and `Descriptor.compile_script()`

## Project Structure

```
vuers/
├── crates/
│   ├── lib_vue_compiler_sfc_sys/   # Raw FFI crate
│   │   ├── src/lib.rs              # FFI bindings (extern "C")
│   │   ├── ffi/
│   │   │   ├── cpp/                # C++ wrapper (runtime.cpp, vue_sfc.cpp)
│   │   │   └── js/                 # JS bridge code
│   │   └── build.rs                # Build script (bundles JS, compiles native)
│   └── libvue_compiler_sfc/        # Safe Rust API crate
│       ├── src/
│       │   ├── lib.rs              # Public API
│       │   ├── bindings/           # Safe wrapper types
│       │   └── tests/              # Unit tests
│       └── examples/               # Demo programs
├── tools/                          # Build tooling (npm package)
│   ├── bundle.ts                   # Rolldown bundler
│   └── benchmark-*.ts              # Benchmark scripts
├── hermes/                         # Git submodule (Static Hermes)
├── dist/                           # Build artifacts
│   ├── vue-compiler.js             # Bundled JS
│   └── vue-compiler.o              # Compiled native object
├── Cargo.toml                      # Workspace manifest
└── justfile                        # Task runner
```

## Linking

The `build.rs` in `lib_vue_compiler_sfc_sys` links against:
- `dist/vue-compiler.o` (compiled Vue compiler)
- Hermes static libraries: `shermes_console_a`, `hermesvm_a`, `jsi`, `boost_context`
- System libraries: `c++`, `Foundation` framework (macOS)

## shermes Compilation Flags

| Flag | Meaning |
|------|---------|
| `-O` | Expensive optimizations (highest level) |
| `-O0` | No optimizations |
| `-Og` | Debug-friendly optimizations |
| `-Os` | Optimize for size |
| `-c` | Compile only (output .o file) |
| `-exported-unit=X` | Name the exported unit |
| `-typed` | Enable typed JS compilation |
| `-reuse-prop-cache` | Reuse property cache entries |

## Performance Notes

**Key insight**: The bottleneck is the Vue compiler execution itself, NOT FFI overhead. V8's JIT outperforms Static Hermes AOT because:
- Runtime type profiling vs static-only analysis
- Hot path optimization vs uniform treatment
- Speculative compilation with deoptimization

**Optimization attempts and results**:
- Remove JSON serialization: +164% throughput
- Cache compile function: +4% throughput
- Batch compilation: slower (JSON overhead)
- C++ → C wrapper: no change (not the bottleneck)

## Common Pitfalls & Best Practices

- **Check surrounding code for conventions:** Before adding new code, always study the existing patterns, naming conventions, and architectural choices in the file and directory you are working in.

- **FFI string passing:** Use (pointer, length) pairs instead of CString for FFI calls. This avoids allocation, copying, and NUL scanning. Safe because FFI calls are synchronous and the callee copies data into its own heap.

## References

- [Devon Govett's Static Hermes blog](https://devongovett.me/blog/static-hermes.html)
- [How to try Static Hermes](https://github.com/facebook/hermes/discussions/1137)
- [Static Hermes internals](https://github.com/facebook/hermes/discussions/1685)
