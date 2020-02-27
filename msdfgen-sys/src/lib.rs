/*!
# Unsafe bindings to msdfgen

This crate provides generated unsafe Rust bindings to [msdfgen](https://github.com/Chlumsky/msdfgen) C library.

Probably this isn't that you really need. See [safe bindings](https://crates.io/crate/msdfgen).
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "generate-bindings"))]
include!("bindings.rs");
