use crate::{ffi, Point2};

/// Edge color enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EdgeColor {
    Black = ffi::msdfgen_EdgeColor_BLACK,
    Blue = ffi::msdfgen_EdgeColor_BLUE,
    Cyan = ffi::msdfgen_EdgeColor_CYAN,
    Green = ffi::msdfgen_EdgeColor_GREEN,
    Magenta = ffi::msdfgen_EdgeColor_MAGENTA,
    Red = ffi::msdfgen_EdgeColor_RED,
    White = ffi::msdfgen_EdgeColor_WHITE,
    Yellow = ffi::msdfgen_EdgeColor_YELLOW,
}

impl Default for EdgeColor {
    fn default() -> Self {
        EdgeColor::White
    }
}

impl EdgeColor {
    pub(crate) fn into_raw(self) -> ffi::msdfgen_EdgeColor {
        self as _
    }
}

/// Edge holder object
#[repr(transparent)]
pub struct EdgeHolder {
    raw: ffi::msdfgen_EdgeHolder,
}

impl Default for EdgeHolder {
    /// Creates new edge holder object
    fn default() -> Self {
        let raw = unsafe { ffi::msdfgen_EdgeHolder::new() };
        Self { raw }
    }
}

impl Clone for EdgeHolder {
    /// Clones edge holder object
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::msdfgen_EdgeHolder::new5(&self.raw) };
        Self { raw }
    }
}

impl Drop for EdgeHolder {
    fn drop(&mut self) {
        unsafe { self.raw.destruct(); }
    }
}

impl EdgeHolder {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_EdgeHolder {
        &self.raw
    }

    /// Creates line edge (linear segment)
    pub fn new_linear(
        point_0: impl Into<Point2<f64>>,
        point_1: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_EdgeHolder::new2(
                point_0.into().into_raw(),
                point_1.into().into_raw(),
                edge_color.into().into_raw(),
            )
        };
        Self { raw }
    }

    /// Creates conic edge (quadratic curve segment)
    pub fn new_conic(
        point_0: impl Into<Point2<f64>>,
        point_1: impl Into<Point2<f64>>,
        point_2: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_EdgeHolder::new3(
                point_0.into().into_raw(),
                point_1.into().into_raw(),
                point_2.into().into_raw(),
                edge_color.into().into_raw(),
            )
        };
        Self { raw }
    }

    /// Creates cubic edge (cubic curve segment)
    pub fn new_cubic(
        point_0: impl Into<Point2<f64>>,
        point_1: impl Into<Point2<f64>>,
        point_2: impl Into<Point2<f64>>,
        point_3: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_EdgeHolder::new4(
                point_0.into().into_raw(),
                point_1.into().into_raw(),
                point_2.into().into_raw(),
                point_3.into().into_raw(),
                edge_color.into().into_raw(),
            )
        };
        Self { raw }
    }

    /// Gets edge color
    pub fn color(&self) -> EdgeColor {
        unsafe { core::mem::transmute((*self.raw.edgeSegment).color) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn edge_holder_linear() {
        let _edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::default());
    }

    #[test]
    fn edge_holder_conic() {
        let _edge = EdgeHolder::new_conic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::default());
    }

    #[test]
    fn edge_holder_cubic() {
        let _edge = EdgeHolder::new_cubic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), (1.0, 1.0), EdgeColor::default());
    }
}
