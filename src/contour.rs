use crate::{ffi, Bound, EdgeHolder, EdgeSegment, Polarity};

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
        unsafe {
            ffi::msdfgen_Contour_destructor(&mut self.raw);
        }
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
    pub fn bound(&self, bound: &mut Bound<f64>) {
        unsafe {
            self.raw.bound(
                &mut bound.left,
                &mut bound.bottom,
                &mut bound.right,
                &mut bound.top,
            )
        }
    }

    /// Gets the bounding box to fit the contour.
    pub fn get_bound(&self) -> Bound<f64> {
        let mut bound = Bound::default();
        self.bound(&mut bound);
        bound
    }

    /// Adjusts the bounding box to fit the contour border's mitered corners.
    pub fn bound_miters(
        &self,
        bound: &mut Bound<f64>,
        border: f64,
        miter_limit: f64,
        polarity: Polarity,
    ) {
        unsafe {
            self.raw.boundMiters(
                &mut bound.left,
                &mut bound.bottom,
                &mut bound.right,
                &mut bound.top,
                border,
                miter_limit,
                polarity as _,
            )
        }
    }

    /// Gets the bounding box to fit the contour border's mitered corners.
    pub fn get_bound_miters(
        &self,
        border: f64,
        miter_limit: f64,
        polarity: Polarity,
    ) -> Bound<f64> {
        let mut bound = Bound::default();
        self.bound_miters(&mut bound, border, miter_limit, polarity);
        bound
    }

    /// Computes the winding of the contour. Returns 1 if positive, -1 if negative.
    pub fn winding(&self) -> i32 {
        unsafe { self.raw.winding() }
    }
}
