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
        unsafe { core::mem::transmute(self.raw.addEdge2()) }
    }

    /// Adds segment as an edge to the contour.
    pub fn add_segment(&mut self, segment: impl EdgeSegment) {
        self.add_edge(&segment.into())
    }

    /// Adjusts the bounding box to fit the contour.
    pub fn bounds(&self) -> Bounds<f64> {
        let mut left = core::mem::MaybeUninit::uninit();
        let mut bottom = core::mem::MaybeUninit::uninit();
        let mut right = core::mem::MaybeUninit::uninit();
        let mut top = core::mem::MaybeUninit::uninit();

        unsafe { self.raw.bounds(
            left.as_mut_ptr(),
            bottom.as_mut_ptr(),
            right.as_mut_ptr(),
            top.as_mut_ptr(),
        ) }

        Bounds::new(
            unsafe { left.assume_init() },
            unsafe { bottom.assume_init() },
            unsafe { right.assume_init() },
            unsafe { top.assume_init() },
        )
    }

    /// Adjusts the bounding box to fit the contour border's mitered corners.
    pub fn miter_bounds(&self, border: f64, miter_limit: f64) -> Bounds<f64> {
        let mut left = core::mem::MaybeUninit::uninit();
        let mut bottom = core::mem::MaybeUninit::uninit();
        let mut right = core::mem::MaybeUninit::uninit();
        let mut top = core::mem::MaybeUninit::uninit();

        unsafe { self.raw.miterBounds(
            left.as_mut_ptr(),
            bottom.as_mut_ptr(),
            right.as_mut_ptr(),
            top.as_mut_ptr(),
            border,
            miter_limit,
        ) }

        Bounds::new(
            unsafe { left.assume_init() },
            unsafe { bottom.assume_init() },
            unsafe { right.assume_init() },
            unsafe { top.assume_init() },
        )
    }

    /// Computes the winding of the contour. Returns 1 if positive, -1 if negative.
    pub fn winding(&self) -> i32 {
        unsafe { self.raw.winding() }
    }
}
