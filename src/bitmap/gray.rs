use super::Pixel;

/// Gray color
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Gray<T> {
    pub v: T,
}

impl<T> Gray<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}

impl From<Gray<u8>> for Gray<f32> {
    fn from(Gray { v }: Gray<u8>) -> Self {
        Self::new(
            (v as f32) / 255.0,
        )
    }
}

impl From<Gray<f32>> for Gray<u8> {
    fn from(Gray { v }: Gray<f32>) -> Self {
        Self::new(
            (v.min(1.0).max(0.0) * 255.0) as u8,
        )
    }
}

impl Pixel for Gray<f32> {
    type Component = f32;

    fn components(&self) -> &[Self::Component] {
        unsafe { core::slice::from_raw_parts(
            &self.v,
            1,
        ) }
    }

    fn components_mut(&mut self) -> &mut [Self::Component] {
        unsafe { core::slice::from_raw_parts_mut(
            &mut self.v,
            1,
        ) }
    }

    fn invert(&mut self) {
        self.components_mut().iter_mut().for_each(|component| {
            *component = 1.0 - *component;
        });
    }
}

impl Pixel for Gray<u8> {
    type Component = u8;

    fn components(&self) -> &[Self::Component] {
        unsafe { core::slice::from_raw_parts(
            &self.v,
            1,
        ) }
    }

    fn components_mut(&mut self) -> &mut [Self::Component] {
        unsafe { core::slice::from_raw_parts_mut(
            &mut self.v,
            1,
        ) }
    }

    fn invert(&mut self) {
        self.components_mut().iter_mut().for_each(|component| {
            *component = 255 - *component;
        });
    }
}
