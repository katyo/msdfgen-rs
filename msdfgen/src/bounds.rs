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
    /// Create new bounds object
    pub fn new(left: T, bottom: T, right: T, top: T) -> Self {
        Self { left, bottom, right, top }
    }
}
