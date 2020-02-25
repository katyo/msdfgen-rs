# Safe bindings to msdfgen library

[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT)
[![Crates.io Package](https://img.shields.io/crates/v/msdfgen.svg?style=popout)](https://crates.io/crates/msdfgen)
[![Docs.rs API Docs](https://docs.rs/msdfgen/badge.svg)](https://docs.rs/msdfgen)
[![Travis-CI Status](https://travis-ci.com/katyo/msdfgen-rs.svg?branch=master)](https://travis-ci.com/katyo/msdfgen-rs)

## Crates

- [msdfgen-sys](https://crates.io/crates/msdfgen-sys) Low-level unsafe bindings generated using bindgen.
- [msdfgen-lib](https://crates.io/crates/msdfgen-lib) Bundled library which can be build and link with application.
- [msdfgen](https://crates.io/crates/msdfgen) High-level safe bindings which should be used by applications.

## Features

- __ttf-parse__ Enables [ttf-parser](https://crates.io/crates/ttf-parser) crate integration which allows create shapes for glyphs of specific font.
- __font__ Enables [font](https://crates.io/crates/font) crate integration which allows create shapes for glyphs of specific font.
- __freetype-rs__ Enables [freetype-rs](https://crates.io/crates/freetype-rs) crate integration which allows create shapes for glyphs of specific font.
- __png__ Enables [png](https://crates.io/crates/png) crate integration which allows load and save bitmaps from/as PNG images.

## Usage

```rust
use msdfgen_lib; // forces linking with msdfgen library
use std::fs::File;
use material_icons::{Icon, icon_to_char, FONT};
use ttf_parser::Font;
use msdfgen::{FontExt, Bitmap, generate_msdf, EDGE_THRESHOLD, OVERLAP_SUPPORT};

let font = Font::from_data(&FONT, 0).unwrap();
let chr = icon_to_char(Icon::Fingerprint);
let glyph = font.glyph_index(chr).unwrap();
let mut shape = font.glyph_shape(glyph).unwrap();
let mut bitmap = Bitmap::new(32, 32);

shape.edge_coloring_simple(3.0, 0);

generate_msdf(&mut bitmap, &shape, 4.0, 1.0, 0.0, EDGE_THRESHOLD, OVERLAP_SUPPORT);

let mut output = File::create("fingerprint-msdf.png").unwrap();
bitmap.write_png(&mut output).unwrap();
```
