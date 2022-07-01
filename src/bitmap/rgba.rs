use super::Pixel;

/// RGB color
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Rgba<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Rgba<u8>> for Rgba<f32> {
    fn from(Rgba { r, g, b, a }: Rgba<u8>) -> Self {
        Self::new(
            (r as f32) * (1.0 / 255.0),
            (g as f32) * (1.0 / 255.0),
            (b as f32) * (1.0 / 255.0),
            (a as f32) * (1.0 / 255.0),
        )
    }
}

impl From<Rgba<f32>> for Rgba<u8> {
    fn from(Rgba { r, g, b, a }: Rgba<f32>) -> Self {
        Self::new(
            (r.min(1.0).max(0.0) * 255.0) as u8,
            (g.min(1.0).max(0.0) * 255.0) as u8,
            (b.min(1.0).max(0.0) * 255.0) as u8,
            (a.min(1.0).max(0.0) * 255.0) as u8,
        )
    }
}

impl Pixel for Rgba<f32> {
    type Component = f32;

    fn components(&self) -> &[Self::Component] {
        unsafe { core::slice::from_raw_parts(&self.r, 3) }
    }

    fn components_mut(&mut self) -> &mut [Self::Component] {
        unsafe { core::slice::from_raw_parts_mut(&mut self.r, 4) }
    }

    fn invert(&mut self) {
        self.components_mut().iter_mut().for_each(|component| {
            *component = 1.0 - *component;
        });
    }
}

impl Pixel for Rgba<u8> {
    type Component = u8;

    fn components(&self) -> &[Self::Component] {
        unsafe { core::slice::from_raw_parts(&self.r, 4) }
    }

    fn components_mut(&mut self) -> &mut [Self::Component] {
        unsafe { core::slice::from_raw_parts_mut(&mut self.r, 4) }
    }

    fn invert(&mut self) {
        self.components_mut().iter_mut().for_each(|component| {
            *component = 255 - *component;
        });
    }
}
