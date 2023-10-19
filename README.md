# php-sys

This crate contains a set of bindings to PHP's C API.

It provides the following functionality:
* Access to PHP's core data structures and functions (exposed from `php.h`).
* Ability to execute PHP code inside of Rust projects, with additional helpers for converting between common Rust & PHP data types.