#[cfg(feature = "ttf-parser")]
mod ttf_parser;

#[cfg(feature = "font")]
mod font;

#[cfg(feature = "freetype-rs")]
mod freetype_rs;

use crate::Shape;

/// Extensions for font objects
pub trait FontExt {
    type Glyph;

    /// Crates a shape for specific glyph of font
    fn glyph_shape(&self, glyph: Self::Glyph) -> Option<Shape>;
}
