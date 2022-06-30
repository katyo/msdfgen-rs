use crate::{ffi, Bitmap, Gray, RGB};

pub const MID_VALUE: f32 = 0.5;

/// Rendering target helper trait
pub trait RenderTarget<Source>: Sized {
    fn render(target: &mut Bitmap<Self>, source: &Bitmap<Source>, px_range: f64, mid_value: f32);
}

impl RenderTarget<Gray<f32>> for Gray<f32> {
    fn render(
        target: &mut Bitmap<Self>,
        source: &Bitmap<Gray<f32>>,
        px_range: f64,
        mid_value: f32,
    ) {
        unsafe {
            ffi::msdfgen_renderSDF(target.as_raw_mut(), source.as_raw(), px_range, mid_value);
        }
    }
}

impl RenderTarget<Gray<f32>> for RGB<f32> {
    fn render(
        target: &mut Bitmap<Self>,
        source: &Bitmap<Gray<f32>>,
        px_range: f64,
        mid_value: f32,
    ) {
        unsafe {
            ffi::msdfgen_renderSDF1(target.as_raw_mut(), source.as_raw(), px_range, mid_value);
        }
    }
}

impl RenderTarget<RGB<f32>> for Gray<f32> {
    fn render(target: &mut Bitmap<Self>, source: &Bitmap<RGB<f32>>, px_range: f64, mid_value: f32) {
        unsafe {
            ffi::msdfgen_renderSDF2(target.as_raw_mut(), source.as_raw(), px_range, mid_value);
        }
    }
}

impl RenderTarget<RGB<f32>> for RGB<f32> {
    fn render(target: &mut Bitmap<Self>, source: &Bitmap<RGB<f32>>, px_range: f64, mid_value: f32) {
        unsafe {
            ffi::msdfgen_renderSDF3(target.as_raw_mut(), source.as_raw(), px_range, mid_value);
        }
    }
}

impl<S> Bitmap<S> {
    pub fn render<T: RenderTarget<S>>(
        &self,
        target: &mut Bitmap<T>,
        px_range: f64,
        mid_value: f32,
    ) {
        T::render(target, self, px_range, mid_value)
    }
}
