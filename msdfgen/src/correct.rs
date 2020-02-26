use crate::{ffi, Shape, Bitmap, Gray, RGB, Framing, FillRule};

/// Sign correction helper trait
pub trait SignCorrection: Sized {
    fn correct_sign(
        bitmap: &mut Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        fill_rule: FillRule,
    );
}

impl SignCorrection for Gray<f32> {
    fn correct_sign(
        bitmap: &mut Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        fill_rule: FillRule,
    ) {
        unsafe {
            ffi::msdfgen_distanceSignCorrection(
                bitmap.as_raw_mut(),
                shape.as_raw(),
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                fill_rule as _,
            );
        }
    }
}

impl SignCorrection for RGB<f32> {
    fn correct_sign(
        bitmap: &mut Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        fill_rule: FillRule,
    ) {
        unsafe {
            ffi::msdfgen_distanceSignCorrection1(
                bitmap.as_raw_mut(),
                shape.as_raw(),
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                fill_rule as _,
            );
        }
    }
}

impl Shape {
    /// Fixes the sign of the input signed distance field, so that it matches the shape's rasterized fill.
    pub fn correct_sign<T: SignCorrection>(
        &self,
        bitmap: &mut Bitmap<T>,
        framing: &Framing<f64>,
        fill_rule: FillRule,
    ) {
        T::correct_sign(bitmap, self, framing, fill_rule);
    }
}

/// Error estimation helper trait
pub trait ErrorEstimation: Sized {
    fn estimate_error(
        bitmap: &Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        scanlines_per_row: u32,
        fill_rule: FillRule,
    ) -> f64;
}

impl ErrorEstimation for Gray<f32> {
    fn estimate_error(
        bitmap: &Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        scanlines_per_row: u32,
        fill_rule: FillRule,
    ) -> f64 {
        unsafe {
            ffi::msdfgen_estimateSDFError(
                bitmap.as_raw(),
                shape.as_raw(),
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                scanlines_per_row as _,
                fill_rule as _,
            )
        }
    }
}

impl ErrorEstimation for RGB<f32> {
    fn estimate_error(
        bitmap: &Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        scanlines_per_row: u32,
        fill_rule: FillRule,
    ) -> f64 {
        unsafe {
            ffi::msdfgen_estimateSDFError1(
                bitmap.as_raw(),
                shape.as_raw(),
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                scanlines_per_row as _,
                fill_rule as _,
            )
        }
    }
}

impl Shape {
    /// Estimates the portion of the area that will be filled incorrectly when rendering using the SDF.
    pub fn estimate_error<T: ErrorEstimation>(
        &self,
        bitmap: &Bitmap<T>,
        framing: &Framing<f64>,
        scanlines_per_row: u32,
        fill_rule: FillRule,
    ) -> f64 {
        T::estimate_error(bitmap, self, framing, scanlines_per_row, fill_rule)
    }
}
