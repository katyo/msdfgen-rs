use crate::ffi;

/// Generic 2D point
pub type Point2<T> = Vector2<T>;

/// Generic 2D vector
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl Vector2<f64> {
    pub(crate) fn into_raw(self) -> ffi::msdfgen_Vector2 {
        unsafe { core::mem::transmute(self) }
    }

    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_Vector2 {
        unsafe { core::mem::transmute(self) }
    }
}

impl<T: Copy> From<T> for Vector2<T> {
    fn from(val: T) -> Self {
        Self::new(val, val)
    }
}

impl<T: Copy> From<(T, T)> for Vector2<T> {
    fn from(val: (T, T)) -> Self {
        Self::new(val.0, val.1)
    }
}

impl<T: Copy> From<[T; 2]> for Vector2<T> {
    fn from(val: [T; 2]) -> Self {
        Self::new(val[0], val[1])
    }
}

impl<T: Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    pub fn reset(&mut self)
    where T: Default
    {
        self.x = T::default();
        self.y = T::default();
    }

    /*pub fn length(&mut self) -> T
    where T: core::ops::Mul<T, Output = T> + core::ops::Add<T, Output = T>
    {
        (self.x * self.x + self.y * self.y).sqrt()
    }*/
}

impl Vector2<f64> {
    pub fn length(&self) -> f64 {
        unsafe { self.as_raw().length() }
    }

    pub fn direction(&self) -> f64 {
        unsafe { self.as_raw().direction() }
    }

    pub fn normalize(&self, allow_zero: bool) -> Self {
        unsafe { core::mem::transmute(self.as_raw().normalize(allow_zero)) }
    }

    pub fn get_orthogonal(&self, polarity: bool) -> Self {
        unsafe { core::mem::transmute(self.as_raw().getOrthogonal(polarity)) }
    }

    pub fn get_orthonormal(&self, polarity: bool, allow_zero: bool) -> Self {
        unsafe { core::mem::transmute(self.as_raw().getOrthonormal(polarity, allow_zero)) }
    }

    pub fn project(&self, vector: &Self, positive: bool) -> Self {
        unsafe { core::mem::transmute(self.as_raw().project(vector.as_raw(), positive)) }
    }
}
