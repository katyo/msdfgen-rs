use crate::{ffi, Bitmap, Gray, RGB, Shape, Vector2};

/// Generate signed distance field for shape
pub fn generate_sdf(
    output: &mut Bitmap<Gray<f32>>,
    shape: &Shape,
    range: f64,
    scale: impl Into<Vector2<f64>>,
    translate: impl Into<Vector2<f64>>,
    overlap_support: bool,
) {
    unsafe {
        ffi::msdfgen_generateSDF(
            output.as_raw_mut(),
            shape.as_raw(),
            range,
            scale.into().as_raw(),
            translate.into().as_raw(),
            overlap_support,
        )
    }
}

/// Generate pseudo signed distance field for shape
pub fn generate_pseudo_sdf(
    output: &mut Bitmap<Gray<f32>>,
    shape: &Shape,
    range: f64,
    scale: impl Into<Vector2<f64>>,
    translate: impl Into<Vector2<f64>>,
    overlap_support: bool,
) {
    unsafe {
        ffi::msdfgen_generatePseudoSDF(
            output.as_raw_mut(),
            shape.as_raw(),
            range,
            scale.into().as_raw(),
            translate.into().as_raw(),
            overlap_support,
        )
    }
}

/// Generate signed distance field for shape (legacy)
pub fn generate_sdf_legacy(
    output: &mut Bitmap<Gray<f32>>,
    shape: &Shape,
    range: f64,
    scale: impl Into<Vector2<f64>>,
    translate: impl Into<Vector2<f64>>,
) {
    unsafe {
        ffi::msdfgen_generateSDF_legacy(
            output.as_raw_mut(),
            shape.as_raw(),
            range,
            scale.into().as_raw(),
            translate.into().as_raw(),
        )
    }
}

/// Generate multi-channel signed distance field for shape
pub fn generate_msdf(
    output: &mut Bitmap<RGB<f32>>,
    shape: &Shape,
    range: f64,
    scale: impl Into<Vector2<f64>>,
    translate: impl Into<Vector2<f64>>,
    edge_threshold: f64,
    overlap_support: bool,
) {
    unsafe {
        ffi::msdfgen_generateMSDF(
            output.as_raw_mut(),
            shape.as_raw(),
            range,
            scale.into().as_raw(),
            translate.into().as_raw(),
            edge_threshold,
            overlap_support,
        )
    }
}

/// Generate multi-channel signed distance field for shape (legacy)
pub fn generate_msdf_legacy(
    output: &mut Bitmap<RGB<f32>>,
    shape: &Shape,
    range: f64,
    scale: impl Into<Vector2<f64>>,
    translate: impl Into<Vector2<f64>>,
    edge_threshold: f64,
) {
    unsafe {
        ffi::msdfgen_generateMSDF_legacy(
            output.as_raw_mut(),
            shape.as_raw(),
            range,
            scale.into().as_raw(),
            translate.into().as_raw(),
            edge_threshold,
        )
    }
}
