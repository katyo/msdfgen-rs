use crate::{ffi, Bound, Contour, Polarity, Scanline};

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
        unsafe { &mut *(self.raw.addContour2() as *mut Contour) }
    }

    /// Normalizes the shape geometry for distance field generation
    pub fn normalize(&mut self) {
        unsafe {
            self.raw.normalize();
        }
    }

    /// Performs basic checks to determine if the object represents a valid shape
    pub fn validate(&self) -> bool {
        unsafe { self.raw.validate() }
    }

    /// Adjusts the bounding box to fit the shape
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

    /// Gets the bounding box to fit the shape
    pub fn get_bound(&self) -> Bound<f64> {
        let mut bound = Bound::default();
        self.bound(&mut bound);
        bound
    }

    /// Adjusts the bounding box to fit the shape border's mitered corners
    pub fn bound_miters(
        &self,
        bounds: &mut Bound<f64>,
        border: f64,
        miter_limit: f64,
        polarity: Polarity,
    ) {
        unsafe {
            self.raw.boundMiters(
                &mut bounds.left,
                &mut bounds.bottom,
                &mut bounds.right,
                &mut bounds.top,
                border,
                miter_limit,
                polarity as _,
            )
        }
    }

    /// Gets the bounding box to fit the shape border's mitered corners
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

    /// Outputs the scanline that intersects the shape at y
    pub fn scanline(&self, y: f64) -> Scanline {
        let mut scanline = Scanline::default();

        unsafe {
            self.raw.scanline(scanline.as_raw_mut(), y);
        }

        scanline
    }

    /// Assigns colors to edges of the shape in accordance to the multi-channel distance field technique. May split some edges if necessary. angleThreshold specifies the maximum angle (in radians) to be considered a corner, for example 3 (~172 degrees). Values below 1/2 PI will be treated as the external angle.
    pub fn edge_coloring_simple(&mut self, angle_threshold: f64, seed: u64) {
        unsafe { ffi::msdfgen_edgeColoringSimple(&mut self.raw, angle_threshold, seed) }
    }
}
