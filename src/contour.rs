use crate::{ffi, EdgeHolder, EdgeSegment, Bounds};

/// Contour object
#[repr(transparent)]
pub struct Contour {
    raw: ffi::msdfgen_Contour,
}

impl Default for Contour {
    /// Creates new empty contour (with no edges)
    fn default() -> Self {
        let raw = unsafe { ffi::msdfgen_Contour_constructor() };
        Self { raw }
    }
}

impl Drop for Contour {
    fn drop(&mut self) {
        unsafe { ffi::msdfgen_Contour_destructor(&mut self.raw); }
    }
}

impl Contour {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_Contour {
        &self.raw
    }

    /// Adds an edge to the contour.
    pub fn add_edge(&mut self, edge: &EdgeHolder) {
        unsafe { self.raw.addEdge(edge.as_raw()) }
    }

    /// Creates a new edge in the contour and returns its reference.
    pub fn add_edge_mut(&mut self) -> &mut EdgeHolder {
        unsafe { &mut *(self.raw.addEdge2() as *mut EdgeHolder) }
    }

    /// Adds segment as an edge to the contour.
    pub fn add_segment(&mut self, segment: impl EdgeSegment) {
        self.add_edge(&segment.into())
    }

    /// Adjusts the bounding box to fit the contour.
    pub fn bounds(&self, bounds: &mut Bounds<f64>) {
        unsafe { self.raw.bounds(
            &mut bounds.left,
            &mut bounds.bottom,
            &mut bounds.right,
            &mut bounds.top,
        ) }
    }

    /// Gets the bounding box to fit the contour.
    pub fn get_bounds(&self) -> Bounds<f64> {
        let mut bounds = Bounds::default();
        self.bounds(&mut bounds);
        bounds
    }

    /// Adjusts the bounding box to fit the contour border's mitered corners.
    pub fn miter_bounds(&self, bounds: &mut Bounds<f64>, border: f64, miter_limit: f64) {
        unsafe { self.raw.miterBounds(
            &mut bounds.left,
            &mut bounds.bottom,
            &mut bounds.right,
            &mut bounds.top,
            border,
            miter_limit,
        ) }
    }

    /// Gets the bounding box to fit the contour border's mitered corners.
    pub fn get_miter_bounds(&self, border: f64, miter_limit: f64) -> Bounds<f64> {
        let mut bounds = Bounds::default();
        self.miter_bounds(&mut bounds, border, miter_limit);
        bounds
    }


    /// Computes the winding of the contour. Returns 1 if positive, -1 if negative.
    pub fn winding(&self) -> i32 {
        unsafe { self.raw.winding() }
    }
}
