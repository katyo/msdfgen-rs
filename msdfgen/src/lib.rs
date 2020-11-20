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
use msdfgen_lib; // forces linking with msdfgen library
use std::fs::File;
use notosans::REGULAR_TTF as FONT;
use ttf_parser::Font;
use msdfgen::{FontExt, Bitmap, Gray, Range, EDGE_THRESHOLD, OVERLAP_SUPPORT};

let font = Font::from_data(&FONT, 0).unwrap();

let glyph = font.glyph_index('A').unwrap();

let mut shape = font.glyph_shape(glyph).unwrap();

let width = 32;
let height = 32;

let bounds = shape.get_bounds();
let framing = bounds.autoframe(width, height, Range::Px(4.0), None).unwrap();

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

mod vector;
mod bitmap;
mod bounds;
mod segment;
mod edge;
mod contour;
mod scanline;
mod shape;
mod generate;
mod correct;
mod render;
mod interop;

#[cfg(test)]
use msdfgen_lib as _;

pub(crate) use msdfgen_sys as ffi;

pub use self::vector::*;
pub use self::bitmap::*;
pub use self::bounds::*;
pub use self::segment::*;
pub use self::edge::*;
pub use self::contour::*;
pub use self::scanline::*;
pub use self::shape::*;
pub use self::generate::*;
pub use self::correct::*;
pub use self::render::*;
pub use self::interop::*;

// Run via: cargo test --features "png,ttf-parser"
// or: cargo test --features "all" etc.
#[cfg(test)]
#[allow(dead_code)]
mod test {
    use std::fs::File;
    #[cfg(feature = "ttf-parser")]
    use ttf_parser::Font;
    #[cfg(feature = "freetype-rs")]
    use freetype as freetype_rs;
    use all_asserts::assert_lt;

    use notosans::REGULAR_TTF;
    use material_icons::{Icon, icon_to_char, FONT};

    use crate::{FontExt, Bitmap, Range, Gray, FillRule, EDGE_THRESHOLD, OVERLAP_SUPPORT};

    #[cfg(feature = "ttf-parser")]
    #[cfg(feature = "png")]
    fn test_font_char_ttf_parser(name: &str, font: &[u8], chr: char, width: u32, height: u32, expected_error: f64) {
        let font = Font::from_data(font, 0).unwrap();
        let glyph = font.glyph_index(chr).unwrap();
        let mut shape = font.glyph_shape(glyph).unwrap();

        if !shape.validate() {
            panic!("Invalid shape");
        }
        shape.normalize();

        let bounds = shape.get_bounds();

        let mut bitmap = Bitmap::new(width, height);

        println!("bounds: {:?}", bounds);

        shape.edge_coloring_simple(3.0, 0);

        let framing = bounds.autoframe(width, height, Range::Px(4.0), None).unwrap();

        println!("framing: {:?}", framing);

        shape.generate_msdf(&mut bitmap, &framing, EDGE_THRESHOLD, OVERLAP_SUPPORT);
        shape.correct_sign(&mut bitmap, &framing, FillRule::default());
        let error = shape.estimate_error(&mut bitmap, &framing, 4, FillRule::default());

        assert_lt!(error, expected_error);

        bitmap.flip_y();

        let mut output = File::create(&format!("ttf-parser-{}-msdf.png", name)).unwrap();
        bitmap.write_png(&mut output).unwrap();

        let mut preview = Bitmap::<Gray<f32>>::new(width * 10, height * 10);

        bitmap.render(&mut preview, Default::default());

        let mut output = File::create(&format!("ttf-parser-{}-preview.png", name)).unwrap();
        preview.write_png(&mut output).unwrap();
    }

    #[cfg(feature = "freetype-rs")]
    #[cfg(feature = "png")]
    fn test_font_char_freetype_rs(name: &str, font: &[u8], chr: char, width: u32, height: u32, expected_error: f64) -> freetype_rs::FtResult<()> {

        let library = freetype_rs::Library::init()?;
        let face = library.new_memory_face(font.to_vec(), 0)?;
        face.set_pixel_sizes(width, height)?;
        let glyph_index = face.get_char_index(chr as usize);
        let mut shape = face.glyph_shape(glyph_index).unwrap();

        if !shape.validate() {
            panic!("Invalid shape");
        }
        shape.normalize();

        let bounds = shape.get_bounds();

        let mut bitmap = Bitmap::new(width, height);

        println!("bounds: {:?}", bounds);

        shape.edge_coloring_simple(3.0, 0);

        let framing = bounds.autoframe(width, height, Range::Px(4.0), None).unwrap();

        println!("framing: {:?}", framing);

        shape.generate_msdf(&mut bitmap, &framing, EDGE_THRESHOLD, OVERLAP_SUPPORT);
        shape.correct_sign(&mut bitmap, &framing, FillRule::default());
        let error = shape.estimate_error(&mut bitmap, &framing, 4, FillRule::default());

        assert_lt!(error, expected_error);

        bitmap.flip_y();

        let mut output = File::create(&format!("freetype-{}-msdf.png", name)).unwrap();
        bitmap.write_png(&mut output).unwrap();

        let mut preview = Bitmap::<Gray<f32>>::new(width * 10, height * 10);

        bitmap.render(&mut preview, Default::default());

        let mut output = File::create(&format!("freetype-{}-preview.png", name)).unwrap();
        preview.write_png(&mut output).unwrap();

        Ok(())
    }

    fn test_font_char(name: &str, font: &[u8], chr: char, width: u32, height: u32, expected_error: f64) {

        #[cfg(feature = "ttf-parser")]
        #[cfg(feature = "png")]
        test_font_char_ttf_parser(name, font, chr, width, height, expected_error);

        #[cfg(feature = "freetype-rs")]
        #[cfg(feature = "png")]
        test_font_char_freetype_rs(name, font, chr, width, height, expected_error).unwrap();
    }

    #[test]
    fn test_regular_ttf_upcase_a_letter() {
        test_font_char("A-letter", &REGULAR_TTF, 'A', 32, 32, 0.000016);
    }

    #[test]
    fn test_material_icon_fingerprint() {
        test_font_char("fingerprint", &FONT, icon_to_char(Icon::Fingerprint), 64, 64, 0.0015);
    }
}
