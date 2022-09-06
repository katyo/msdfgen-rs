# Safe bindings to msdfgen library

[![github](https://img.shields.io/badge/github-katyo/msdfgen--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/msdfgen-rs)
[![crate](https://img.shields.io/crates/v/msdfgen.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/msdfgen)
[![docs](https://img.shields.io/badge/docs.rs-msdfgen-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/msdfgen)
[![MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/workflow/status/katyo/msdfgen-rs/Rust?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/msdfgen-rs/actions?query=workflow%3ARust)

## Crates

- [msdfgen-sys](https://crates.io/crates/msdfgen-sys) Low-level unsafe bindings generated using bindgen.
- [msdfgen](https://crates.io/crates/msdfgen) High-level safe bindings which should be used by applications.

## Features

- __ttf-parse__ Enables [ttf-parser](https://crates.io/crates/ttf-parser) crate integration which allows create shapes for glyphs of specific font.
- __font__ Enables [font](https://crates.io/crates/font) crate integration which allows create shapes for glyphs of specific font.
- __freetype-rs__ Enables [freetype-rs](https://crates.io/crates/freetype-rs) crate integration which allows create shapes for glyphs of specific font.
- __png__ Enables [png](https://crates.io/crates/png) crate integration which allows load and save bitmaps from/as PNG images.
- __all__ Meta-feature which enables all supported features.

## Usage

```no_run
use std::fs::File;
use notosans::REGULAR_TTF as FONT;
use ttf_parser::Face;
use msdfgen::{FontExt, Bitmap, Gray, Range, MsdfGeneratorConfig, FillRule, MID_VALUE};

let font = Face::from_slice(&FONT, 0).unwrap();

let glyph = font.glyph_index('A').unwrap();

let mut shape = font.glyph_shape(glyph).unwrap();

let width = 32;
let height = 32;

let bound = shape.get_bound();
let framing = bound.autoframe(width, height, Range::Px(4.0), None).unwrap();
let fill_rule = FillRule::default();

let mut bitmap = Bitmap::new(width, height);

shape.edge_coloring_simple(3.0, 0);

let config = MsdfGeneratorConfig::default();

shape.generate_msdf(&mut bitmap, &framing, &config);

// optionally
shape.correct_sign(&mut bitmap, &framing, fill_rule);
shape.correct_msdf_error(&mut bitmap, &framing, &config);

let error = shape.estimate_error(&mut bitmap, &framing, 5, Default::default());

println!("Estimated error: {}", error);

bitmap.flip_y();

let mut output = File::create("A-letter-msdf.png").unwrap();
bitmap.write_png(&mut output).unwrap();

let mut preview = Bitmap::<Gray<f32>>::new(width * 10, height * 10);
bitmap.render(&mut preview, Default::default(), MID_VALUE);

let mut output = File::create("A-letter-preview.png").unwrap();
preview.write_png(&mut output).unwrap();
```
