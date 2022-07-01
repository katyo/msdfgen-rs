#![doc = include_str!("../README.md")]

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
        Bitmap, ErrorCorrectionConfig, FillRule, FontExt, Gray, Range, Shape, MID_VALUE,
        OVERLAP_SUPPORT,
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
