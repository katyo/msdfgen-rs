use crate::{ffi, EdgeColor, Point2};

/// Segment kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
pub enum SegmentKind {
    /// Linear segment
    Linear = ffi::msdfgen_SegmentKind_LINEAR,
    /// Quadratic segment
    Quadratic = ffi::msdfgen_SegmentKind_QUADRATIC,
    /// Cubic segment
    Cubic = ffi::msdfgen_SegmentKind_CUBIC,
}

/// Generic edge segment
pub trait EdgeSegment {
    /// Gets segment kind
    fn segment_kind(&self) -> SegmentKind;

    /// Gets starting point of segment
    fn start_point(&self) -> &Point2<f64>;

    /// Gets starting point of segment (mutable)
    fn start_point_mut(&mut self) -> &mut Point2<f64>;

    /// Gets control point of segment
    fn control_point(&self, num: u8) -> Option<&Point2<f64>>;

    /// Gets control point of segment (mutable)
    fn control_point_mut(&mut self, num: u8) -> Option<&mut Point2<f64>>;

    /// Gets end point of segment
    fn end_point(&self) -> &Point2<f64>;

    /// Gets end point of segment (mutable)
    fn end_point_mut(&mut self) -> &mut Point2<f64>;

    /// Gets edge color of segment
    fn edge_color(&self) -> &EdgeColor;

    /// Gets edge color of segment (mutable)
    fn edge_color_mut(&mut self) -> &mut EdgeColor;

    fn into_raw(self) -> *mut ffi::msdfgen_EdgeSegment;
}

/// Linear segment object
#[repr(transparent)]
pub struct LinearSegment {
    raw: ffi::msdfgen_LinearSegment,
}

impl LinearSegment {
    /// Creates new linear segment
    pub fn new(
        start_point: impl Into<Point2<f64>>,
        end_point: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_LinearSegment::new(
                start_point.into().into_raw(),
                end_point.into().into_raw(),
                edge_color.into().into_raw(),
            )
        };
        Self { raw }
    }
}

impl EdgeSegment for LinearSegment {
    fn segment_kind(&self) -> SegmentKind {
        SegmentKind::Linear
    }

    fn start_point(&self) -> &Point2<f64> {
        unsafe { core::mem::transmute(&self.raw.p[0]) }
    }

    fn start_point_mut(&mut self) -> &mut Point2<f64> {
        unsafe { core::mem::transmute(&mut self.raw.p[0]) }
    }

    fn control_point(&self, _num: u8) -> Option<&Point2<f64>> {
        None
    }

    fn control_point_mut(&mut self, _num: u8) -> Option<&mut Point2<f64>> {
        None
    }

    fn end_point(&self) -> &Point2<f64> {
        unsafe { core::mem::transmute(&self.raw.p[1]) }
    }

    fn end_point_mut(&mut self) -> &mut Point2<f64> {
        unsafe { core::mem::transmute(&mut self.raw.p[1]) }
    }

    fn edge_color(&self) -> &EdgeColor {
        unsafe { core::mem::transmute(&self.raw._base.color) }
    }

    fn edge_color_mut(&mut self) -> &mut EdgeColor {
        unsafe { core::mem::transmute(&mut self.raw._base.color) }
    }

    fn into_raw(self) -> *mut ffi::msdfgen_EdgeSegment {
        Box::into_raw(Box::new(self.raw)) as _
    }
}

/// Quadratic segment object
#[repr(transparent)]
pub struct QuadraticSegment {
    raw: ffi::msdfgen_QuadraticSegment,
}

impl QuadraticSegment {
    /// Creates new quadratic curve segment
    pub fn new(
        start_point: impl Into<Point2<f64>>,
        control_point: impl Into<Point2<f64>>,
        end_point: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_QuadraticSegment::new(
                start_point.into().into_raw(),
                control_point.into().into_raw(),
                end_point.into().into_raw(),
                edge_color.into().into_raw(),
            )
        };
        Self { raw }
    }
}

impl EdgeSegment for QuadraticSegment {
    fn segment_kind(&self) -> SegmentKind {
        SegmentKind::Quadratic
    }

    fn start_point(&self) -> &Point2<f64> {
        unsafe { core::mem::transmute(&self.raw.p[0]) }
    }

    fn start_point_mut(&mut self) -> &mut Point2<f64> {
        unsafe { core::mem::transmute(&mut self.raw.p[0]) }
    }

    fn control_point(&self, num: u8) -> Option<&Point2<f64>> {
        match num {
            0 => Some(unsafe { core::mem::transmute(&self.raw.p[1]) }),
            _ => None,
        }
    }

    fn control_point_mut(&mut self, num: u8) -> Option<&mut Point2<f64>> {
        match num {
            0 => Some(unsafe { core::mem::transmute(&mut self.raw.p[1]) }),
            _ => None,
        }
    }

    fn end_point(&self) -> &Point2<f64> {
        unsafe { core::mem::transmute(&self.raw.p[2]) }
    }

    fn end_point_mut(&mut self) -> &mut Point2<f64> {
        unsafe { core::mem::transmute(&mut self.raw.p[2]) }
    }

    fn edge_color(&self) -> &EdgeColor {
        unsafe { core::mem::transmute(&self.raw._base.color) }
    }

    fn edge_color_mut(&mut self) -> &mut EdgeColor {
        unsafe { core::mem::transmute(&mut self.raw._base.color) }
    }

    fn into_raw(self) -> *mut ffi::msdfgen_EdgeSegment {
        Box::into_raw(Box::new(self.raw)) as _
    }
}

/// Quadratic segment object
#[repr(transparent)]
pub struct CubicSegment {
    raw: ffi::msdfgen_CubicSegment,
}

impl CubicSegment {
    /// Creates new cubic curve segment
    pub fn new(
        start_point: impl Into<Point2<f64>>,
        control_point1: impl Into<Point2<f64>>,
        control_point2: impl Into<Point2<f64>>,
        end_point: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_CubicSegment::new(
                start_point.into().into_raw(),
                control_point1.into().into_raw(),
                control_point2.into().into_raw(),
                end_point.into().into_raw(),
                edge_color.into().into_raw(),
            )
        };
        Self { raw }
    }
}

impl EdgeSegment for CubicSegment {
    fn segment_kind(&self) -> SegmentKind {
        SegmentKind::Cubic
    }

    fn start_point(&self) -> &Point2<f64> {
        unsafe { core::mem::transmute(&self.raw.p[0]) }
    }

    fn start_point_mut(&mut self) -> &mut Point2<f64> {
        unsafe { core::mem::transmute(&mut self.raw.p[0]) }
    }

    fn control_point(&self, num: u8) -> Option<&Point2<f64>> {
        match num {
            0 => Some(unsafe { core::mem::transmute(&self.raw.p[1]) }),
            1 => Some(unsafe { core::mem::transmute(&self.raw.p[2]) }),
            _ => None,
        }
    }

    fn control_point_mut(&mut self, num: u8) -> Option<&mut Point2<f64>> {
        match num {
            0 => Some(unsafe { core::mem::transmute(&mut self.raw.p[1]) }),
            1 => Some(unsafe { core::mem::transmute(&mut self.raw.p[2]) }),
            _ => None,
        }
    }

    fn end_point(&self) -> &Point2<f64> {
        unsafe { core::mem::transmute(&self.raw.p[3]) }
    }

    fn end_point_mut(&mut self) -> &mut Point2<f64> {
        unsafe { core::mem::transmute(&mut self.raw.p[3]) }
    }

    fn edge_color(&self) -> &EdgeColor {
        unsafe { core::mem::transmute(&self.raw._base.color) }
    }

    fn edge_color_mut(&mut self) -> &mut EdgeColor {
        unsafe { core::mem::transmute(&mut self.raw._base.color) }
    }

    fn into_raw(self) -> *mut ffi::msdfgen_EdgeSegment {
        Box::into_raw(Box::new(self.raw)) as _
    }
}
