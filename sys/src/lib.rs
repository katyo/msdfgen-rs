/*!
# Unsafe bindings to msdfgen

This crate provides generated unsafe Rust bindings to [msdfgen](https://github.com/Chlumsky/msdfgen) C library.

Probably this isn't that you really need. See [safe bindings](https://crates.io/crate/msdfgen).
 */

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!(concat!("bindings/", env!("MSDFGEN_BINDINGS")));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(MSDFGEN_VERSION, b"1.9\0");
    }
}
