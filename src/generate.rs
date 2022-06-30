use crate::{ffi, Bitmap, Gray, RGB, Shape, Vector2};

/// Framing options
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Framing<T> {
    pub range: T,
    pub scale: Vector2<T>,
    pub translate: Vector2<T>,
}

impl<T> Framing<T> {
    pub fn new(range: impl Into<T>, scale: impl Into<Vector2<T>>, translate: impl Into<Vector2<T>>) -> Self {
        Self {
            range: range.into(),
            scale: scale.into(),
            translate: translate.into(),
        }
    }
}

/// Default edge threshold
pub const EDGE_THRESHOLD: f64 = 1.001;

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
            ffi::msdfgen_generateSDF(
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
            ffi::msdfgen_generatePseudoSDF(
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
    pub fn generate_sdf_legacy(
        &self,
        output: &mut Bitmap<Gray<f32>>,
        framing: &Framing<f64>,
    ) {
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

    /// Generate multi-channel signed distance field for shape
    pub fn generate_msdf(
        &self,
        output: &mut Bitmap<RGB<f32>>,
        framing: &Framing<f64>,
        edge_threshold: f64,
        overlap_support: bool,
    ) {
        unsafe {
            ffi::msdfgen_generateMSDF(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                edge_threshold,
                overlap_support,
            )
        }
    }

    /// Generate multi-channel signed distance field for shape (legacy)
    pub fn generate_msdf_legacy(
        &self,
        output: &mut Bitmap<RGB<f32>>,
        framing: &Framing<f64>,
        edge_threshold: f64,
    ) {
        unsafe {
            ffi::msdfgen_generateMSDF_legacy(
                output.as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                edge_threshold,
            )
        }
    }
}
