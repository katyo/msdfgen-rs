use crate::{ffi, Bitmap, FillRule, Framing, Gray, MsdfGeneratorConfig, Rgb, Rgba, Shape};

/// Sign correction helper trait
pub trait SignCorrection: Sized {
    fn correct_sign(
        bitmap: &mut Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        fill_rule: FillRule,
    );
}

/// Error correction helper trait
pub trait MsdfErrorCorrection: Sized {
    fn correct_msdf_error(
        bitmap: &mut Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        config: &MsdfGeneratorConfig,
    );
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
                framing.projection.as_raw(),
                fill_rule as _,
            );
        }
    }
}

impl SignCorrection for Rgb<f32> {
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
                framing.projection.as_raw(),
                fill_rule as _,
            );
        }
    }
}

impl SignCorrection for Rgba<f32> {
    fn correct_sign(
        bitmap: &mut Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        fill_rule: FillRule,
    ) {
        unsafe {
            ffi::msdfgen_distanceSignCorrection2(
                bitmap.as_raw_mut(),
                shape.as_raw(),
                framing.projection.as_raw(),
                fill_rule as _,
            );
        }
    }
}

impl MsdfErrorCorrection for Rgb<f32> {
    fn correct_msdf_error(
        bitmap: &mut Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        config: &MsdfGeneratorConfig,
    ) {
        unsafe {
            ffi::msdfgen_msdfErrorCorrection(
                bitmap.as_raw_mut(),
                shape.as_raw(),
                framing.projection.as_raw(),
                framing.range,
                config.as_raw(),
            );
        }
    }
}

impl MsdfErrorCorrection for Rgba<f32> {
    fn correct_msdf_error(
        bitmap: &mut Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        config: &MsdfGeneratorConfig,
    ) {
        unsafe {
            ffi::msdfgen_msdfErrorCorrection1(
                bitmap.as_raw_mut(),
                shape.as_raw(),
                framing.projection.as_raw(),
                framing.range,
                config.as_raw(),
            );
        }
    }
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
                framing.projection.as_raw(),
                scanlines_per_row as _,
                fill_rule as _,
            )
        }
    }
}

impl ErrorEstimation for Rgb<f32> {
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
                framing.projection.as_raw(),
                scanlines_per_row as _,
                fill_rule as _,
            )
        }
    }
}

impl ErrorEstimation for Rgba<f32> {
    fn estimate_error(
        bitmap: &Bitmap<Self>,
        shape: &Shape,
        framing: &Framing<f64>,
        scanlines_per_row: u32,
        fill_rule: FillRule,
    ) -> f64 {
        unsafe {
            ffi::msdfgen_estimateSDFError2(
                bitmap.as_raw(),
                shape.as_raw(),
                framing.projection.as_raw(),
                scanlines_per_row as _,
                fill_rule as _,
            )
        }
    }
}

impl Shape {
    /// Fixes the sign of the input signed distance field, so that it matches the shape's rasterized fill.
    pub fn correct_sign<T: SignCorrection>(
        &self,
        mut bitmap: impl AsMut<Bitmap<T>>,
        framing: impl AsRef<Framing<f64>>,
        fill_rule: FillRule,
    ) {
        T::correct_sign(bitmap.as_mut(), self, framing.as_ref(), fill_rule);
    }

    /// Corrects MSDF error
    pub fn correct_msdf_error<T: MsdfErrorCorrection>(
        &self,
        mut bitmap: impl AsMut<Bitmap<T>>,
        framing: impl AsRef<Framing<f64>>,
        config: impl AsRef<MsdfGeneratorConfig>,
    ) {
        T::correct_msdf_error(bitmap.as_mut(), self, framing.as_ref(), config.as_ref());
    }

    /// Estimates the portion of the area that will be filled incorrectly when rendering using the SDF.
    pub fn estimate_error<T: ErrorEstimation>(
        &self,
        bitmap: impl AsRef<Bitmap<T>>,
        framing: impl AsRef<Framing<f64>>,
        scanlines_per_row: u32,
        fill_rule: FillRule,
    ) -> f64 {
        T::estimate_error(
            bitmap.as_ref(),
            self,
            framing.as_ref(),
            scanlines_per_row,
            fill_rule,
        )
    }
}
