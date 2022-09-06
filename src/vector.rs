use crate::ffi;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
    where
        T: Default,
    {
        self.x = T::default();
        self.y = T::default();
    }
}

impl<T> Add<T> for Vector2<T>
where
    T: AddAssign + Copy,
{
    type Output = Vector2<T>;

    fn add(mut self, v: T) -> Self::Output {
        self += v;
        self
    }
}

impl<T> AddAssign<T> for Vector2<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, v: T) {
        self.x += v;
        self.y += v;
    }
}

impl<T> Sub<T> for Vector2<T>
where
    T: SubAssign + Copy,
{
    type Output = Vector2<T>;

    fn sub(mut self, v: T) -> Self::Output {
        self -= v;
        self
    }
}

impl<T> SubAssign<T> for Vector2<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, v: T) {
        self.x -= v;
        self.y -= v;
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: MulAssign + Copy,
{
    type Output = Vector2<T>;

    fn mul(mut self, v: T) -> Self::Output {
        self *= v;
        self
    }
}

impl<T> MulAssign<T> for Vector2<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, v: T) {
        self.x *= v;
        self.y *= v;
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: DivAssign + Copy,
{
    type Output = Vector2<T>;

    fn div(mut self, v: T) -> Self::Output {
        self /= v;
        self
    }
}

impl<T> DivAssign<T> for Vector2<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, v: T) {
        self.x /= v;
        self.y /= v;
    }
}

impl<T> Add<Vector2<T>> for Vector2<T>
where
    T: AddAssign + Copy,
{
    type Output = Vector2<T>;

    fn add(mut self, v: Vector2<T>) -> Self::Output {
        self += v;
        self
    }
}

impl<T> AddAssign<Vector2<T>> for Vector2<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, v: Vector2<T>) {
        self.x += v.x;
        self.y += v.y;
    }
}

impl<T> Sub<Vector2<T>> for Vector2<T>
where
    T: SubAssign + Copy,
{
    type Output = Vector2<T>;

    fn sub(mut self, v: Vector2<T>) -> Self::Output {
        self -= v;
        self
    }
}

impl<T> SubAssign<Vector2<T>> for Vector2<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, v: Vector2<T>) {
        self.x -= v.x;
        self.y -= v.y;
    }
}

impl<T> Mul<Vector2<T>> for Vector2<T>
where
    T: MulAssign + Copy,
{
    type Output = Vector2<T>;

    fn mul(mut self, v: Vector2<T>) -> Self::Output {
        self *= v;
        self
    }
}

impl<T> MulAssign<Vector2<T>> for Vector2<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, v: Vector2<T>) {
        self.x *= v.x;
        self.y *= v.y;
    }
}

impl<T> Div<Vector2<T>> for Vector2<T>
where
    T: DivAssign + Copy,
{
    type Output = Vector2<T>;

    fn div(mut self, v: Vector2<T>) -> Self::Output {
        self /= v;
        self
    }
}

impl<T> DivAssign<Vector2<T>> for Vector2<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, v: Vector2<T>) {
        self.x /= v.x;
        self.y /= v.y;
    }
}

impl<T> Neg for Vector2<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Vector2<T>;

    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
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
