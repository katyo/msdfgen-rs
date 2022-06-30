use crate::{ffi, Bitmap, ErrorCorrectionConfig, Gray, Shape, Vector2, RGB};

/// Framing options
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Framing<T> {
    pub range: T,
    pub scale: Vector2<T>,
    pub translate: Vector2<T>,
}

impl<T> Framing<T> {
    pub fn new(
        range: impl Into<T>,
        scale: impl Into<Vector2<T>>,
        translate: impl Into<Vector2<T>>,
    ) -> Self {
        Self {
            range: range.into(),
            scale: scale.into(),
            translate: translate.into(),
        }
    }
}

/// Default overlap support
pub const OVERLAP_SUPPORT: bool = true;

impl Shape {
    /// Generate signed distance field for shape
    pub fn generate_sdf(
        &self,
        output: &mut Bitmap<Gray<f32>>,
        framing: &Framing<f64>,
        overlap_support: bool,
    ) {
        unsafe {
            ffi::msdfgen_generateSDF1(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                overlap_support,
            )
        }
    }

    /// Generate pseudo signed distance field for shape
    pub fn generate_pseudo_sdf(
        &self,
        output: &mut Bitmap<Gray<f32>>,
        framing: &Framing<f64>,
        overlap_support: bool,
    ) {
        unsafe {
            ffi::msdfgen_generatePseudoSDF1(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                overlap_support,
            )
        }
    }

    /// Generate signed distance field for shape (legacy)
    pub fn generate_sdf_legacy(&self, output: &mut Bitmap<Gray<f32>>, framing: &Framing<f64>) {
        unsafe {
            ffi::msdfgen_generateSDF_legacy(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
            )
        }
    }

    /// Generate pseudo signed distance field for shape (legacy)
    pub fn generate_pseudo_sdf_legacy(
        &self,
        output: &mut Bitmap<Gray<f32>>,
        framing: &Framing<f64>,
    ) {
        unsafe {
            ffi::msdfgen_generatePseudoSDF_legacy(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
            )
        }
    }

    /// Generate multi-channel signed distance field for shape
    pub fn generate_msdf(
        &self,
        output: &mut Bitmap<RGB<f32>>,
        framing: &Framing<f64>,
        error_correction_config: &ErrorCorrectionConfig,
        overlap_support: bool,
    ) {
        unsafe {
            ffi::msdfgen_generateMSDF1(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                error_correction_config.as_raw(),
                overlap_support,
            )
        }
    }

    /// Generate multi-channel signed distance field for shape (legacy)
    pub fn generate_msdf_legacy(
        &self,
        output: &mut Bitmap<RGB<f32>>,
        framing: &Framing<f64>,
        error_correction_config: &ErrorCorrectionConfig,
    ) {
        unsafe {
            ffi::msdfgen_generateMSDF_legacy(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                *error_correction_config.as_raw(),
            )
        }
    }

    /// Generate multi-channel signed distance field for shape with true distance in the alpha channel
    pub fn generate_mtsdf(
        &self,
        output: &mut Bitmap<RGB<f32>>,
        framing: &Framing<f64>,
        error_correction_config: &ErrorCorrectionConfig,
        overlap_support: bool,
    ) {
        unsafe {
            ffi::msdfgen_generateMTSDF1(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                error_correction_config.as_raw(),
                overlap_support,
            )
        }
    }

    /// Generate multi-channel signed distance field for shape with true distance in the alpha channel (legacy)
    pub fn generate_mtsdf_legacy(
        &self,
        output: &mut Bitmap<RGB<f32>>,
        framing: &Framing<f64>,
        error_correction_config: &ErrorCorrectionConfig,
    ) {
        unsafe {
            ffi::msdfgen_generateMTSDF_legacy(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                *error_correction_config.as_raw(),
            )
        }
    }
}
