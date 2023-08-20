use crate::{ffi, CubicSegment, EdgeSegment, LinearSegment, Point2, QuadraticSegment, SegmentKind};

/// Edge color enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
pub enum EdgeColor {
    Black = ffi::msdfgen_EdgeColor_BLACK,
    Blue = ffi::msdfgen_EdgeColor_BLUE,
    Cyan = ffi::msdfgen_EdgeColor_CYAN,
    Green = ffi::msdfgen_EdgeColor_GREEN,
    Magenta = ffi::msdfgen_EdgeColor_MAGENTA,
    Red = ffi::msdfgen_EdgeColor_RED,
    White = ffi::msdfgen_EdgeColor_WHITE,
    Yellow = ffi::msdfgen_EdgeColor_YELLOW,
}

impl Default for EdgeColor {
    fn default() -> Self {
        EdgeColor::White
    }
}

impl EdgeColor {
    pub(crate) fn into_raw(self) -> ffi::msdfgen_EdgeColor {
        self as _
    }
}

/// Edge holder object
#[repr(transparent)]
pub struct EdgeHolder {
    raw: ffi::msdfgen_EdgeHolder,
}

impl Default for EdgeHolder {
    /// Creates new edge holder object
    fn default() -> Self {
        let raw = unsafe { ffi::msdfgen_EdgeHolder::new() };
        Self { raw }
    }
}

impl Clone for EdgeHolder {
    /// Clones edge holder object
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::msdfgen_EdgeHolder::new5(&self.raw) };
        Self { raw }
    }
}

impl Drop for EdgeHolder {
    fn drop(&mut self) {
        unsafe {
            self.raw.destruct();
        }
    }
}

impl<T: EdgeSegment> From<T> for EdgeHolder {
    fn from(segment: T) -> Self {
        Self::new(segment)
    }
}

impl EdgeHolder {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_EdgeHolder {
        &self.raw
    }

    /// Created new edge with segment
    pub fn new(segment: impl EdgeSegment) -> Self {
        let raw = unsafe { ffi::msdfgen_EdgeHolder::new1(segment.into_raw()) };
        Self { raw }
    }

    /// Set segment to edge
    pub fn set(&mut self, segment: impl EdgeSegment) {
        unsafe { ffi::msdfgen_EdgeHolder_setSegment(&mut self.raw, segment.into_raw()) }
    }

    /// Creates line edge (linear segment)
    pub fn new_linear(
        start_point: impl Into<Point2<f64>>,
        end_point: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_EdgeHolder::new2(
                start_point.into().into_raw(),
                end_point.into().into_raw(),
                edge_color.into().into_raw(),
            )
        };
        Self { raw }
    }

    /// Creates quadratic edge (quadratic curve segment)
    pub fn new_quadratic(
        start_point: impl Into<Point2<f64>>,
        control_point: impl Into<Point2<f64>>,
        end_point: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_EdgeHolder::new3(
                start_point.into().into_raw(),
                control_point.into().into_raw(),
                end_point.into().into_raw(),
                edge_color.into().into_raw(),
            )
        };
        Self { raw }
    }

    /// Creates cubic edge (cubic curve segment)
    pub fn new_cubic(
        start_point: impl Into<Point2<f64>>,
        control_point1: impl Into<Point2<f64>>,
        control_point2: impl Into<Point2<f64>>,
        end_point: impl Into<Point2<f64>>,
        edge_color: impl Into<EdgeColor>,
    ) -> Self {
        let raw = unsafe {
            ffi::msdfgen_EdgeHolder::new4(
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

impl core::ops::Deref for EdgeHolder {
    type Target = dyn EdgeSegment;

    fn deref(&self) -> &Self::Target {
        let raw_segment = unsafe { &*self.raw.edgeSegment };

        let segment_kind: SegmentKind =
            unsafe { core::mem::transmute(ffi::msdfgen_EdgeSegment_getKind(raw_segment)) };

        match segment_kind {
            SegmentKind::Linear => unsafe {
                core::mem::transmute::<_, &LinearSegment>(raw_segment)
            },
            SegmentKind::Quadratic => unsafe {
                core::mem::transmute::<_, &QuadraticSegment>(raw_segment)
            },
            SegmentKind::Cubic => unsafe { core::mem::transmute::<_, &CubicSegment>(raw_segment) },
        }
    }
}

impl core::ops::DerefMut for EdgeHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let raw_segment = unsafe { &mut *self.raw.edgeSegment };

        let segment_kind: SegmentKind =
            unsafe { core::mem::transmute(ffi::msdfgen_EdgeSegment_getKind(raw_segment)) };

        match segment_kind {
            SegmentKind::Linear => unsafe {
                core::mem::transmute::<_, &mut LinearSegment>(raw_segment)
            },
            SegmentKind::Quadratic => unsafe {
                core::mem::transmute::<_, &mut QuadraticSegment>(raw_segment)
            },
            SegmentKind::Cubic => unsafe {
                core::mem::transmute::<_, &mut CubicSegment>(raw_segment)
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn edge_holder_new_linear() {
        let edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.segment_kind(), SegmentKind::Linear);
    }

    #[test]
    fn edge_holder_new_quadratic() {
        let edge =
            EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.segment_kind(), SegmentKind::Quadratic);
    }

    #[test]
    fn edge_holder_new_cubic() {
        let edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::default(),
        );

        assert_eq!(edge.segment_kind(), SegmentKind::Cubic);
    }

    #[test]
    fn edge_holder_new_linear_segment() {
        let edge = EdgeHolder::new(LinearSegment::new(
            (0.0, 1.0),
            (1.0, 0.0),
            EdgeColor::default(),
        ));

        assert_eq!(edge.segment_kind(), SegmentKind::Linear);
    }

    #[test]
    fn edge_holder_new_quadratic_segment() {
        let edge = EdgeHolder::new(QuadraticSegment::new(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            EdgeColor::default(),
        ));

        assert_eq!(edge.segment_kind(), SegmentKind::Quadratic);
    }

    #[test]
    fn edge_holder_new_cubic_segment() {
        let edge = EdgeHolder::new(CubicSegment::new(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::default(),
        ));

        assert_eq!(edge.segment_kind(), SegmentKind::Cubic);
    }

    #[test]
    fn get_start_point() {
        let edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.start_point(), &Point2::new(0.0, 1.0));

        let edge =
            EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.start_point(), &Point2::new(0.0, 1.0));

        let edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::default(),
        );

        assert_eq!(edge.start_point(), &Point2::new(0.0, 1.0));
    }

    #[test]
    fn set_start_point() {
        let mut edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.start_point(), &Point2::new(0.0, 1.0));
        *edge.start_point_mut() = (1.0, 1.0).into();
        assert_eq!(edge.start_point(), &Point2::new(1.0, 1.0));

        let mut edge =
            EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.start_point(), &Point2::new(0.0, 1.0));
        *edge.start_point_mut() = (1.0, 1.0).into();
        assert_eq!(edge.start_point(), &Point2::new(1.0, 1.0));

        let mut edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::default(),
        );

        assert_eq!(edge.start_point(), &Point2::new(0.0, 1.0));
        *edge.start_point_mut() = (1.0, 1.0).into();
        assert_eq!(edge.start_point(), &Point2::new(1.0, 1.0));
    }

    #[test]
    fn get_end_point() {
        let edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.end_point(), &Point2::new(1.0, 0.0));

        let edge =
            EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.end_point(), &Point2::new(1.0, 0.0));

        let edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::default(),
        );

        assert_eq!(edge.end_point(), &Point2::new(1.0, 1.0));
    }

    #[test]
    fn set_end_point() {
        let mut edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.end_point(), &Point2::new(1.0, 0.0));
        *edge.end_point_mut() = (1.0, 1.0).into();
        assert_eq!(edge.end_point(), &Point2::new(1.0, 1.0));

        let mut edge =
            EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.end_point(), &Point2::new(1.0, 0.0));
        *edge.end_point_mut() = (1.0, 1.0).into();
        assert_eq!(edge.end_point(), &Point2::new(1.0, 1.0));

        let mut edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::default(),
        );

        assert_eq!(edge.end_point(), &Point2::new(1.0, 1.0));
        *edge.end_point_mut() = (0.0, 0.0).into();
        assert_eq!(edge.end_point(), &Point2::new(0.0, 0.0));
    }

    #[test]
    fn get_control_point() {
        let edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.control_point(0), None);
        assert_eq!(edge.control_point(1), None);
        assert_eq!(edge.control_point(2), None);

        let edge =
            EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.control_point(0), Some(&Point2::new(0.0, 0.0)));
        assert_eq!(edge.control_point(1), None);
        assert_eq!(edge.control_point(2), None);

        let edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::default(),
        );

        assert_eq!(edge.control_point(0), Some(&Point2::new(0.0, 0.0)));
        assert_eq!(edge.control_point(1), Some(&Point2::new(1.0, 0.0)));
        assert_eq!(edge.control_point(2), None);
    }

    #[test]
    fn set_control_point() {
        let mut edge =
            EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::default());

        assert_eq!(edge.control_point(0), Some(&Point2::new(0.0, 0.0)));
        *edge.control_point_mut(0).unwrap() = (1.0, 1.0).into();
        assert_eq!(edge.control_point(0), Some(&Point2::new(1.0, 1.0)));

        let mut edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::default(),
        );

        assert_eq!(edge.control_point(0), Some(&Point2::new(0.0, 0.0)));
        *edge.control_point_mut(0).unwrap() = (0.0, 1.0).into();
        assert_eq!(edge.control_point(0), Some(&Point2::new(0.0, 1.0)));

        assert_eq!(edge.control_point(1), Some(&Point2::new(1.0, 0.0)));
        *edge.control_point_mut(1).unwrap() = (0.0, 1.0).into();
        assert_eq!(edge.control_point(1), Some(&Point2::new(0.0, 1.0)));
    }

    #[test]
    fn get_edge_color() {
        let edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::Blue);
        assert_eq!(edge.edge_color(), &EdgeColor::Blue);

        let edge = EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::Red);

        assert_eq!(edge.edge_color(), &EdgeColor::Red);

        let edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::Magenta,
        );

        assert_eq!(edge.edge_color(), &EdgeColor::Magenta);
    }

    #[test]
    fn set_edge_color() {
        let mut edge = EdgeHolder::new_linear((0.0, 1.0), (1.0, 0.0), EdgeColor::Blue);
        assert_eq!(edge.edge_color(), &EdgeColor::Blue);
        *edge.edge_color_mut() = EdgeColor::Red;
        assert_eq!(edge.edge_color(), &EdgeColor::Red);

        let mut edge =
            EdgeHolder::new_quadratic((0.0, 1.0), (0.0, 0.0), (1.0, 0.0), EdgeColor::Red);

        assert_eq!(edge.edge_color(), &EdgeColor::Red);
        *edge.edge_color_mut() = EdgeColor::Green;
        assert_eq!(edge.edge_color(), &EdgeColor::Green);

        let mut edge = EdgeHolder::new_cubic(
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            EdgeColor::Magenta,
        );

        assert_eq!(edge.edge_color(), &EdgeColor::Magenta);
        *edge.edge_color_mut() = EdgeColor::White;
        assert_eq!(edge.edge_color(), &EdgeColor::White);
    }
}
