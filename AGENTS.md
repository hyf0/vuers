## Verification

MANDATORY: Always run verification commands after making changes.

- `cargo test` to run all Rust unit tests.
- `cargo clippy` to check for code quality issues.
- `cargo fmt -- --check` to ensure code is properly formatted.

**Tip**: Use Bash sub-agents to run commands in parallel for faster verification.


# Common Pitfalls & Best Practices

- **Check surrounding code for conventions:** Before adding new code, always study the existing patterns, naming conventions, and architectural choices in the file and directory you are working in.

- **FFI string passing:** Use (pointer, length) pairs instead of CString for FFI calls. This avoids allocation, copying, and NUL scanning. Safe because FFI calls are synchronous and the callee copies data into its own heap.
