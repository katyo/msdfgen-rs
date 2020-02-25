#[cfg(feature = "ttf-parser")]
mod ttf_parser;

#[cfg(feature = "font")]
mod font;

#[cfg(feature = "freetype-rs")]
mod freetype_rs;

use crate::Shape;

#[cfg(feature = "ttf-parser")]
pub use self::ttf_parser::*;

#[cfg(feature = "font")]
pub use self::font::*;

#[cfg(feature = "freetype-rs")]
pub use self::freetype_rs::*;

/// Extensions for font objects
pub trait FontExt {
    type Glyph;

    /// Crates a shape for specific glyph of font
    fn glyph_shape(&self, glyph: Self::Glyph) -> Option<Shape>;
}
