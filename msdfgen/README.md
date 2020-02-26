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
- __all__ Meta-feature which enables all supported features.

## Usage

```rust
use msdfgen_lib as _; // forces linking with msdfgen library

use std::fs::File;
use notosans::REGULAR_TTF as FONT;
use ttf_parser::Font;

use msdfgen::{FontExt, Bitmap, Range, EDGE_THRESHOLD, OVERLAP_SUPPORT};

fn main() {
    let font = Font::from_data(&FONT, 0).unwrap();
    let glyph = font.glyph_index('A').unwrap();
    let mut shape = font.glyph_shape(glyph).unwrap();

    if !shape.validate() {
        panic!("Invalid shape");
    }
    shape.normalize();

    let bounds = shape.get_bounds();

    let width = 32;
    let height = 32;

    let mut bitmap = Bitmap::new(width, height);

    println!("bounds: {:?}", bounds);

    shape.edge_coloring_simple(3.0, 0);

    let framing = bounds.autoframe(width, height, Range::Px(4.0), None).unwrap();

    println!("framing: {:?}", framing);

    shape.generate_msdf(&mut bitmap, &framing, EDGE_THRESHOLD, OVERLAP_SUPPORT);

    let mut output = File::create("A-msdf.png").unwrap();
    bitmap.write_png(&mut output).unwrap();
}
```
