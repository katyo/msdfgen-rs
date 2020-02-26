use core::ops::{Sub, Neg};

/// Bounds of shape or contour
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Bounds<T> {
    pub left: T,
    pub bottom: T,
    pub right: T,
    pub top: T,
}

impl<T> Bounds<T> {
    /// Creates new bounds
    pub fn new(left: T, bottom: T, right: T, top: T) -> Self {
        Self { left, bottom, right, top }
    }

    /// Gets the width of bounds
    pub fn width(&self) -> T
    where
        T: Sub<T, Output = T> + Neg<Output = T> + PartialOrd + Default + Copy,
    {
        abs(self.right - self.left)
    }

    /// Gets the height of bounds
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
    if a > b { a } else { b }
}
