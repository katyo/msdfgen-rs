use crate::{ffi, Bounds, Contour, Scanline};

/// Shape object
#[repr(transparent)]
pub struct Shape {
    raw: ffi::msdfgen_Shape,
}

impl Default for Shape {
    /// Create new blanked shape with no contours
    fn default() -> Self {
        let raw = unsafe { ffi::msdfgen_Shape::new() };
        Self { raw }
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        unsafe { ffi::msdfgen_Shape_destructor(&mut self.raw) }
    }
}

impl Shape {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_Shape {
        &self.raw
    }

    /// Adds a contour
    pub fn add_contour(&mut self, contour: &Contour) {
        unsafe { self.raw.addContour(contour.as_raw()) }
    }

    /// Adds a blank contour and returns its reference
    pub fn add_contour_mut(&mut self) -> &mut Contour {
        unsafe { core::mem::transmute(self.raw.addContour2()) }
    }

    /// Normalizes the shape geometry for distance field generation
    pub fn normalize(&mut self) {
        unsafe { self.raw.normalize(); }
    }

    /// Performs basic checks to determine if the object represents a valid shape
    pub fn validate(&self) -> bool {
        unsafe { self.raw.validate() }
    }

    /// Adjusts the bounding box to fit the shape
    pub fn bounds(&self, bounds: &mut Bounds<f64>) {
        unsafe { self.raw.bounds(
            &mut bounds.left,
            &mut bounds.bottom,
            &mut bounds.right,
            &mut bounds.top,
        ) }
    }

    /// Gets the bounding box to fit the shape
    pub fn get_bounds(&self) -> Bounds<f64> {
        let mut bounds = Bounds::default();
        self.bounds(&mut bounds);
        bounds
    }

    /// Adjusts the bounding box to fit the shape border's mitered corners
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

    /// Gets the bounding box to fit the shape border's mitered corners
    pub fn get_miter_bounds(&self, border: f64, miter_limit: f64) -> Bounds<f64> {
        let mut bounds = Bounds::default();
        self.miter_bounds(&mut bounds, border, miter_limit);
        bounds
    }

    /// Outputs the scanline that intersects the shape at y
    pub fn scanline(&self, y: f64) -> Scanline {
        let mut scanline = Scanline::default();

        unsafe { self.raw.scanline(scanline.as_raw_mut(), y); }

        scanline
    }

    /// Assigns colors to edges of the shape in accordance to the multi-channel distance field technique. May split some edges if necessary. angleThreshold specifies the maximum angle (in radians) to be considered a corner, for example 3 (~172 degrees). Values below 1/2 PI will be treated as the external angle.
    pub fn edge_coloring_simple(&mut self, angle_threshold: f64, seed: u64) {
        unsafe { ffi::msdfgen_edgeColoringSimple(
            &mut self.raw,
            angle_threshold,
            seed,
        ) }
    }
}
