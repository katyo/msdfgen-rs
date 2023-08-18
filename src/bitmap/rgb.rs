use bytemuck::{Pod, Zeroable};

use super::Pixel;

/// Rgb color
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

unsafe impl<T: Zeroable> Zeroable for Rgb<T> {}
unsafe impl<T: Pod> Pod for Rgb<T> {}

impl<T> Rgb<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl From<Rgb<u8>> for Rgb<f32> {
    fn from(Rgb { r, g, b }: Rgb<u8>) -> Self {
        Self::new(
            (r as f32) * (1.0 / 255.0),
            (g as f32) * (1.0 / 255.0),
            (b as f32) * (1.0 / 255.0),
        )
    }
}

impl From<Rgb<f32>> for Rgb<u8> {
    fn from(Rgb { r, g, b }: Rgb<f32>) -> Self {
        Self::new(
            (r.min(1.0).max(0.0) * 255.0) as u8,
            (g.min(1.0).max(0.0) * 255.0) as u8,
            (b.min(1.0).max(0.0) * 255.0) as u8,
        )
    }
}

impl Pixel for Rgb<f32> {
    type Component = f32;

    fn components(&self) -> &[Self::Component] {
        unsafe { core::slice::from_raw_parts(&self.r, 3) }
    }

    fn components_mut(&mut self) -> &mut [Self::Component] {
        unsafe { core::slice::from_raw_parts_mut(&mut self.r, 3) }
    }

    fn invert(&mut self) {
        self.components_mut().iter_mut().for_each(|component| {
            *component = 1.0 - *component;
        });
    }
}

impl Pixel for Rgb<u8> {
    type Component = u8;

    fn components(&self) -> &[Self::Component] {
        unsafe { core::slice::from_raw_parts(&self.r, 3) }
    }

    fn components_mut(&mut self) -> &mut [Self::Component] {
        unsafe { core::slice::from_raw_parts_mut(&mut self.r, 3) }
    }

    fn invert(&mut self) {
        self.components_mut().iter_mut().for_each(|component| {
            *component = 255 - *component;
        });
    }
}
