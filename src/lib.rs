/*!
# Safe bindings to msdfgen library

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

```ignore
use std::fs::File;
use notosans::REGULAR_TTF as FONT;
use ttf_parser::Face;
use msdfgen::{FontExt, Bitmap, Gray, Range, EDGE_THRESHOLD, OVERLAP_SUPPORT};

let font = Face::from_slice(&FONT, 0).unwrap();

let glyph = font.glyph_index('A').unwrap();

let mut shape = font.glyph_shape(glyph).unwrap();

let width = 32;
let height = 32;

let bound = shape.get_bound();
let framing = bound.autoframe(width, height, Range::Px(4.0), None).unwrap();

let mut bitmap = Bitmap::new(width, height);

shape.edge_coloring_simple(3.0, 0);

shape.generate_msdf(&mut bitmap, &framing, EDGE_THRESHOLD, OVERLAP_SUPPORT);

// optionally
shape.correct_sign(&mut bitmap, &framing, Default::default());

let error = shape.estimate_error(&mut bitmap, &framing, 5, Default::default());

println!("Estimated error: {}", error);

bitmap.flip_y();

let mut output = File::create("A-letter-msdf.png").unwrap();
bitmap.write_png(&mut output).unwrap();

let mut preview = Bitmap::<Gray<f32>>::new(width * 10, height * 10);
bitmap.render(&mut preview, Default::default());

let mut output = File::create("A-letter-preview.png").unwrap();
preview.write_png(&mut output).unwrap();
```
 */

mod bitmap;
mod bound;
mod config;
mod contour;
mod correct;
mod edge;
mod generate;
mod interop;
mod polarity;
mod render;
mod scanline;
mod segment;
mod shape;
mod vector;

pub(crate) use msdfgen_sys as ffi;

pub use bitmap::*;
pub use bound::*;
pub use config::*;
pub use contour::*;
pub use correct::*;
pub use edge::*;
pub use generate::*;
pub use interop::*;
pub use polarity::*;
pub use render::*;
pub use scanline::*;
pub use segment::*;
pub use shape::*;
pub use vector::*;

// Run via: cargo test --features "png,ttf-parser"
// or: cargo test --features "all" etc.
#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
mod test {
    use all_asserts::assert_lt;
    #[cfg(feature = "freetype-rs")]
    use freetype as freetype_rs;
    use std::fs::File;
    #[cfg(feature = "ttf-parser")]
    use ttf_parser::Face;

    use material_icons::{icon_to_char, Icon, FONT};
    use notosans::REGULAR_TTF;

    use crate::{
        Bitmap, ErrorCorrectionConfig, FillRule, FontExt, Gray, Range, MID_VALUE, OVERLAP_SUPPORT, Shape,
    };

    #[cfg(any(feature = "ttf-parser", feature = "freetype-rs"))]
    #[cfg(feature = "png")]
    fn test_font_shape(
        pfx: &str,
        name: &str,
        mut shape: Shape,
        width: u32,
        height: u32,
        expected_error: f64,
    ) {
        if !shape.validate() {
            panic!("Invalid shape");
        }
        shape.normalize();

        let bound = shape.get_bound();

        let mut bitmap = Bitmap::new(width, height);

        println!("bound: {:?}", bound);

        shape.edge_coloring_simple(3.0, 0);

        let framing = bound
            .autoframe(width, height, Range::Px(4.0), None)
            .unwrap();

        println!("framing: {:?}", framing);

        let config = ErrorCorrectionConfig::default();

        shape.generate_msdf(&mut bitmap, &framing, &config, OVERLAP_SUPPORT);
        shape.correct_sign(&mut bitmap, &framing, FillRule::default());
        let error = shape.estimate_error(&mut bitmap, &framing, 4, FillRule::default());

        bitmap.flip_y();

        let mut output = File::create(&format!("{}-{}-msdf.png", pfx, name)).unwrap();
        bitmap.write_png(&mut output).unwrap();

        drop(output);

        let mut preview = Bitmap::<Gray<f32>>::new(width * 10, height * 10);

        bitmap.render(&mut preview, Default::default(), MID_VALUE);

        let mut output = File::create(&format!("{}-{}-preview.png", pfx, name)).unwrap();
        preview.write_png(&mut output).unwrap();

        drop(output);

        assert_lt!(error, expected_error);
    }

    #[cfg(feature = "ttf-parser")]
    #[cfg(feature = "png")]
    fn test_font_char_ttf_parser(
        name: &str,
        font: &[u8],
        chr: char,
        width: u32,
        height: u32,
        expected_error: f64,
    ) {
        let font = Face::from_slice(font, 0).unwrap();
        let glyph = font.glyph_index(chr).unwrap();
        let shape = font.glyph_shape(glyph).unwrap();

        test_font_shape("ttf-parser", name, shape, width, height, expected_error);
    }

    #[cfg(feature = "freetype-rs")]
    #[cfg(feature = "png")]
    fn test_font_char_freetype_rs(
        name: &str,
        font: &[u8],
        chr: char,
        width: u32,
        height: u32,
        expected_error: f64,
    ) {
        let library = freetype_rs::Library::init().unwrap();
        let face = library.new_memory_face(font.to_vec(), 0).unwrap();
        face.set_pixel_sizes(width, height).unwrap();
        let glyph_index = face.get_char_index(chr as usize);
        let shape = face.glyph_shape(glyph_index).unwrap();

        test_font_shape("freetype", name, shape, width, height, expected_error);
    }

    fn test_font_char(
        name: &str,
        font: &[u8],
        chr: char,
        width: u32,
        height: u32,
        expected_error: f64,
    ) {
        #[cfg(feature = "ttf-parser")]
        #[cfg(feature = "png")]
        test_font_char_ttf_parser(name, font, chr, width, height, expected_error);

        #[cfg(feature = "freetype-rs")]
        #[cfg(feature = "png")]
        test_font_char_freetype_rs(name, font, chr, width, height, expected_error);
    }

    #[test]
    fn test_regular_ttf_upcase_a_letter() {
        test_font_char("A-letter", &REGULAR_TTF, 'A', 32, 32, 0.000016);
    }

    #[test]
    fn test_material_icon_fingerprint() {
        test_font_char(
            "fingerprint",
            &FONT,
            icon_to_char(Icon::Fingerprint),
            64,
            64,
            0.0015,
        );
    }
}
