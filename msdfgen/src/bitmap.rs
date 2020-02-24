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

/// RGB color
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGB<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl From<RGB<u8>> for RGB<f32> {
    fn from(RGB { r, g, b }: RGB<u8>) -> Self {
        Self::new(
            (r as f32) / 255.0,
            (g as f32) / 255.0,
            (b as f32) / 255.0,
        )
    }
}

impl From<RGB<f32>> for RGB<u8> {
    fn from(RGB { r, g, b }: RGB<f32>) -> Self {
        Self::new(
            (r.min(1.0).max(0.0) * 255.0) as u8,
            (g.min(1.0).max(0.0) * 255.0) as u8,
            (b.min(1.0).max(0.0) * 255.0) as u8,
        )
    }
}

/// Bitmap
#[repr(C)]
pub struct Bitmap<T> {
    pixels: *mut T,
    width: u32,
    height: u32,
}

fn bitmap_layout<T>(width: u32, height: u32) -> std::alloc::Layout {
    let pixel_size = core::mem::size_of::<T>();
    let pixel_align = core::mem::size_of::<usize>();

    unsafe { std::alloc::Layout::from_size_align_unchecked(
        (width * height) as usize * pixel_size,
        pixel_align,
    ) }
}

impl<T> Bitmap<T> {
    /// Create new bitmap with specified size
    pub fn new(width: u32, height: u32) -> Self {
        let layout = bitmap_layout::<T>(width, height);
        let pixels = unsafe { std::alloc::alloc(layout) as *mut T };

        Self {
            pixels,
            width,
            height,
        }
    }

    /// Get width of bitmap in pixels
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height of bitmap in pixels
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get pixel data slice for reading
    pub fn pixels(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(
            self.pixels,
            (self.width * self.height) as usize,
        ) }
    }

    /// Get pixel data slice for writing
    pub fn pixels_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(
            self.pixels,
            (self.width * self.height) as usize,
        ) }
    }

    /// Get pixel with specified coordinates
    pub fn pixel(&self, x: u32, y: u32) -> &T {
        let index = x + y * self.width();
        &self.pixels()[index as usize]
    }

    /// Get pixel with specified coordinates
    pub fn pixel_mut(&mut self, x: u32, y: u32) -> &mut T {
        let index = x + y * self.width();
        &mut self.pixels_mut()[index as usize]
    }
}

impl<T> Drop for Bitmap<T> {
    fn drop(&mut self) {
        let layout = bitmap_layout::<T>(self.width, self.height);
        unsafe { std::alloc::dealloc(self.pixels as *mut u8, layout) }
    }
}

impl<T> Bitmap<T> {
    pub(crate) fn as_raw_mut(&mut self) -> *mut u8 {
        unsafe { core::mem::transmute(self) }
    }
}
