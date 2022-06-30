use crate::{Framing, Vector2};
use core::ops::{Neg, Sub};

/// Range specifier
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Range<T> {
    Unit(T),
    Px(T),
}

impl<T> Range<T> {
    /// Creates unit range
    pub fn unit(range: T) -> Self {
        Range::Unit(range)
    }

    /// Creates pixel range
    pub fn px(range: T) -> Self {
        Range::Px(range)
    }
}

impl<T> From<T> for Range<T> {
    fn from(range: T) -> Self {
        Range::Unit(range)
    }
}

impl<T> Default for Range<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::from(T::default())
    }
}

/// Bounding box of shape or contour
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Bound<T> {
    pub left: T,
    pub bottom: T,
    pub right: T,
    pub top: T,
}

impl<T> Bound<T> {
    /// Creates new bounding box
    pub fn new(left: T, bottom: T, right: T, top: T) -> Self {
        Self {
            left,
            bottom,
            right,
            top,
        }
    }

    /// Gets the width of bounding box
    pub fn width(&self) -> T
    where
        T: Sub<T, Output = T> + Neg<Output = T> + PartialOrd + Default + Copy,
    {
        abs(self.right - self.left)
    }

    /// Gets the height of bounding box
    pub fn height(&self) -> T
    where
        T: Sub<T, Output = T> + Neg<Output = T> + PartialOrd + Default + Copy,
    {
        abs(self.top - self.bottom)
    }

    /// Gets the size (max of width and height)
    pub fn size(&self) -> T
    where
        T: Sub<T, Output = T> + Neg<Output = T> + PartialOrd + Default + Copy,
    {
        max(self.width(), self.height())
    }
}

impl Bound<f64> {
    /// Autoframing
    ///
    /// Returns `None` means that frame cannot fit the specified pixel range.
    pub fn autoframe(
        &self,
        width: u32,
        height: u32,
        range: Range<f64>,
        scale: Option<Vector2<f64>>,
    ) -> Option<Framing<f64>> {
        let mut frame = Vector2::new(width as f64, height as f64);

        let mut left = self.left;
        let mut bottom = self.bottom;
        let mut right = self.right;
        let mut top = self.top;

        match range {
            Range::Unit(range) => {
                left -= 0.5 * range;
                bottom -= 0.5 * range;
                right += 0.5 * range;
                top += 0.5 * range;
            }
            Range::Px(range) => {
                if scale.is_none() {
                    frame -= range;
                }
            }
        }

        if left >= right || bottom >= top {
            left = 0.0;
            bottom = 0.0;
            right = 1.0;
            top = 1.0;
        }

        if frame.x <= 0.0 || frame.y <= 0.0 {
            return None;
        }

        let dims = Vector2::new(right - left, top - bottom);

        let mut res = Framing::default();

        if let Some(scale) = scale {
            res.translate = (frame / scale - dims) * 0.5 - Vector2::new(left, bottom);
        } else if dims.x * frame.y < dims.y * frame.x {
            res.translate
                .set(0.5 * (frame.x / frame.y * dims.y - dims.x) - left, -bottom);
            res.scale = (frame.y / dims.y).into();
        } else {
            res.translate
                .set(-left, 0.5 * (frame.y / frame.x * dims.x - dims.y) - bottom);
            res.scale = (frame.x / dims.x).into();
        }

        match range {
            Range::Px(range) => {
                if scale.is_none() {
                    res.translate += Vector2::from(range * 0.5) / res.scale;
                }

                res.range = range / min(res.scale.x, res.scale.y);
            }
            Range::Unit(range) => {
                res.range = range;
            }
        }

        Some(res)
    }
}

fn abs<T>(v: T) -> T
where
    T: Neg<Output = T> + PartialOrd + Default,
{
    if v < T::default() {
        -v
    } else {
        v
    }
}

fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd + Copy,
{
    if a > b {
        a
    } else {
        b
    }
}

fn min<T>(a: T, b: T) -> T
where
    T: PartialOrd + Copy,
{
    if a < b {
        a
    } else {
        b
    }
}
