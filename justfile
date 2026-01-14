# Default recipe - show available commands
default:
    @just --list

# Set up the project (install deps, submodules, build hermes)
setup: setup-tools setup-hermes

setup-tools:
    cd tools && pnpm install

setup-hermes:
    git submodule update --init
    cd hermes && cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release
    cd hermes && cmake --build ./build

# Build the Rust project
build:
    cargo build --release

# Run an example (default: demo)
run example="demo":
    cargo run --release --example {{example}}

# Run all benchmarks
bench: bench-native bench-node bench-pure

# Run native (Static Hermes) benchmark
bench-native:
    cargo run --release --example demo

# Run Node.js V8 benchmark
bench-node:
    cd tools && node --experimental-strip-types --no-warnings benchmark-node.ts

# Run pure JS benchmark
bench-pure:
    cd tools && node --experimental-strip-types --no-warnings benchmark-pure-js.ts

# Run cold start benchmark
bench-startup:
    cd tools && node --experimental-strip-types --no-warnings benchmark-startup-node.ts

# Bundle Vue compiler JS (called by build.rs, usually not needed manually)
bundle:
    cd tools && node --experimental-strip-types --no-warnings bundle.ts

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist/*.o dist/*.js
