/*!
# Safe bindings to msdfgen library


 */

mod vector;
mod bitmap;
mod bounds;
mod edge;
mod contour;
mod scanline;
mod shape;
mod generate;

#[cfg(test)]
use msdfgen_lib as _;

pub(crate) use msdfgen_sys as ffi;

pub use self::vector::*;
pub use self::bitmap::*;
pub use self::bounds::*;
pub use self::edge::*;
pub use self::contour::*;
pub use self::scanline::*;
pub use self::shape::*;
pub use self::generate::*;
