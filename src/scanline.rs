use crate::ffi;

/// Fill rule dictates how intersection total is interpreted during rasterization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
pub enum FillRule {
    Negative = ffi::msdfgen_FillRule_FILL_NEGATIVE,
    NonZero = ffi::msdfgen_FillRule_FILL_NONZERO,
    Odd = ffi::msdfgen_FillRule_FILL_ODD,
    Positive = ffi::msdfgen_FillRule_FILL_POSITIVE,
}

impl Default for FillRule {
    fn default() -> Self {
        FillRule::NonZero
    }
}

impl FillRule {
    pub(crate) fn into_raw(self) -> std::ffi::c_int {
        self as _
    }
}

/// Scanline object
pub struct Scanline {
    raw: ffi::msdfgen_Scanline,
}

impl Default for Scanline {
    fn default() -> Self {
        let raw = unsafe { ffi::msdfgen_Scanline::new() };
        Self { raw }
    }
}

impl Drop for Scanline {
    fn drop(&mut self) {
        unsafe { ffi::msdfgen_Scanline_destructor(&mut self.raw) }
    }
}

impl Scanline {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_Scanline {
        &self.raw
    }

    pub(crate) fn as_raw_mut(&mut self) -> &mut ffi::msdfgen_Scanline {
        &mut self.raw
    }

    pub fn overlap(a: &Scanline, b: &Scanline, x_from: f64, x_to: f64, fill_rule: FillRule) -> f64 {
        unsafe {
            ffi::msdfgen_Scanline::overlap(
                a.as_raw(),
                b.as_raw(),
                x_from,
                x_to,
                fill_rule.into_raw(),
            )
        }
    }

    /// Returns the number of intersections left of x
    pub fn count_intersections(&self, x: f64) -> i32 {
        unsafe { self.raw.countIntersections(x) }
    }

    /// Returns the total sign of intersections left of x
    pub fn sum_intersections(&self, x: f64) -> i32 {
        unsafe { self.raw.sumIntersections(x) }
    }

    /// Decides whether the scanline is filled at x based on fill rule
    pub fn filled(&self, x: f64, fill_rule: FillRule) -> bool {
        unsafe { self.raw.filled(x, fill_rule.into_raw()) }
    }
}
