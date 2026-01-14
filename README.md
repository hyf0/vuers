# vuers

Compile the Vue template compiler (@vue/compiler-dom) to native code using Static Hermes, making it callable from Rust without a JavaScript runtime.

## Prerequisites

- [pnpm](https://pnpm.io/) - Node.js package manager
- [just](https://github.com/casey/just) - Command runner
- [Ninja](https://ninja-build.org/) - Build system (for compiling Hermes)
- [CMake](https://cmake.org/) - Build configuration
- [Rust](https://rustup.rs/) - For building the Rust library

### Installing just

```bash
# macOS
brew install just

# Linux
cargo install just

# Windows (via cargo)
cargo install just
```

## Quick Start

```bash
# Clone the repository
git clone <repo-url>
cd rusty-vue

# Set up everything (install deps, build Hermes)
just setup

# Build the Rust library
just build

# Run the demo
just run
```

## Available Commands

Run `just` to see all available commands:

| Command | Description |
|---------|-------------|
| `just setup` | Install dependencies, init submodules, build Hermes |
| `just build` | Build the Rust project (release mode) |
| `just run` | Run the demo binary |
| `just bench` | Run all benchmarks |
| `just bench-native` | Run Static Hermes benchmark |
| `just bench-node` | Run Node.js V8 benchmark |
| `just clean` | Clean build artifacts |

## How It Works

1. **Bundle**: Vue compiler is bundled into a single JS file using Rolldown
2. **Compile**: Static Hermes compiles the JS to native code (`.o` file)
3. **Link**: Rust links against the compiled object and Hermes runtime
4. **Call**: Rust code can call `compile_template()` directly

See [CLAUDE.md](./CLAUDE.md) for detailed architecture documentation.

## API

```rust
use rusty_vue::compile_template;

fn main() {
    let code = compile_template("<div>{{ msg }}</div>").unwrap();
    println!("{}", code);
}
```

## Benchmarks

Compare native compilation vs Node.js V8:

```bash
just bench
```

## License

ISC
