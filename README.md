# libphp

This crate contains a set of Rust bindings to PHP's C API (via `libphp.so`).

## Goals

This project contains both a typical `-sys` crate for performing the actual FFI calls to external C APIs, but also aims to provide a clean safe Rust API wrapping the external "unsafe" C APIs.