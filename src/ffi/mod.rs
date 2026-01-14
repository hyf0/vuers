//! Raw FFI bindings to the Vue SFC compiler via Static Hermes.
//!
//! This module provides low-level, unsafe bindings to the C++ wrapper layer
//! that interfaces with the Vue compiler compiled to native code via Static Hermes.
//!
//! # Architecture
//!
//! ```text
//! Rust (this module)
//!     │
//!     │ extern "C" calls
//!     ▼
//! C++ Wrapper (ffi/wrapper.cpp)
//!     │
//!     │ Hermes JSI calls
//!     ▼
//! JavaScript (ffi/vue-compiler.js)
//!     │
//!     │ @vue/compiler-sfc API
//!     ▼
//! Vue SFC Compiler (native code via Static Hermes)
//! ```
//!
//! # Handle-Based API
//!
//! All JS objects are represented as opaque handles ([`RawHandle`]). Handles are:
//! - 64-bit integers (0 = invalid/null)
//! - 1-indexed into an internal handle table
//! - Must be freed with [`vue_handle_free`] when no longer needed
//!
//! # Safety
//!
//! All functions in this module are unsafe because they:
//! - Dereference raw pointers (C strings)
//! - Operate on opaque handles that may be invalid
//! - Call into the Hermes runtime which is not thread-safe
//!
//! **Important**: The Hermes runtime is single-threaded. All FFI calls must be
//! made from the same thread. Do not share handles across threads.
//!
//! # Usage
//!
//! For most use cases, prefer the safe [`bindings`](crate::bindings) module.
//! This module is only needed for:
//! - Building custom abstractions
//! - Performance-critical code that needs to avoid overhead
//! - Interop with other FFI code

mod raw;

pub use raw::*;
