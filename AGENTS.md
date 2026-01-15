# Common Pitfalls & Best Practices

- **Check surrounding code for conventions:** Before adding new code, always study the existing patterns, naming conventions, and architectural choices in the file and directory you are working in.

- **FFI string passing:** Use (pointer, length) pairs instead of CString for FFI calls. This avoids allocation, copying, and NUL scanning. Safe because FFI calls are synchronous and the callee copies data into its own heap.
