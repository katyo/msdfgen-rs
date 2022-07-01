mod gray;
mod rgb;
mod rgba;

pub use gray::*;
pub use rgb::*;
pub use rgba::*;

#[cfg(feature = "png")]
mod png;

#[cfg(feature = "png")]
pub use self::png::PngColorType;

/// Bitmap pixel
pub trait Pixel {
    type Component;

    fn components(&self) -> &[Self::Component];
    fn components_mut(&mut self) -> &mut [Self::Component];

    fn invert(&mut self);
}

/// Bitmap object
#[repr(C)]
pub struct Bitmap<T> {
    pixels: *mut T,
    width: u32,
    height: u32,
}

unsafe impl<T> Send for Bitmap<T> {}

impl<T> AsRef<Bitmap<T>> for Bitmap<T> {
    fn as_ref(&self) -> &Bitmap<T> {
        self
    }
}

impl<T> AsMut<Bitmap<T>> for Bitmap<T> {
    fn as_mut(&mut self) -> &mut Bitmap<T> {
        self
    }
}

impl<T> Clone for Bitmap<T> {
    fn clone(&self) -> Self {
        let mut new = Self::new(self.width, self.height);

        new.raw_pixels_mut().copy_from_slice(self.raw_pixels());

        new
    }
}

impl<T> Bitmap<T> {
    /// Create new bitmap with specified size
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;

        let mut pixels = Vec::with_capacity(size);
        unsafe {
            pixels.set_len(size);
        }

        let pixels = core::mem::ManuallyDrop::new(pixels).as_mut_ptr();

        Self {
            pixels,
            width,
            height,
        }
    }
}

impl<T> Drop for Bitmap<T> {
    fn drop(&mut self) {
        let size = (self.width * self.height) as usize;
        let _pixels = unsafe { Vec::from_raw_parts(self.pixels, size, size) };
    }
}

impl<T> Bitmap<T> {
    /// Get width of bitmap in pixels
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height of bitmap in pixels
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get pixel data slice for reading from
    pub fn pixels(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.pixels, (self.width * self.height) as usize) }
    }

    /// Get pixel data slice for writing to
    pub fn pixels_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.pixels, (self.width * self.height) as usize) }
    }

    /// Get raw pixels data for reading from
    pub fn raw_pixels(&self) -> &[u8] {
        let pixels = self.pixels();

        unsafe {
            core::slice::from_raw_parts(
                pixels.as_ptr() as _,
                pixels.len() * core::mem::size_of::<T>(),
            )
        }
    }

    /// Get raw pixels data for writing to
    pub fn raw_pixels_mut(&mut self) -> &mut [u8] {
        let pixels = self.pixels_mut();

        unsafe {
            core::slice::from_raw_parts_mut(
                pixels.as_mut_ptr() as _,
                pixels.len() * core::mem::size_of::<T>(),
            )
        }
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

    /// Invert pixels colors
    pub fn invert(&mut self)
    where
        T: Pixel,
    {
        self.pixels_mut().iter_mut().for_each(|pixel| {
            pixel.invert();
        })
    }

    /// Flip pixels around y axis
    pub fn flip_x(&mut self) {
        let width = self.width();
        let height = self.height();
        let pixels = self.pixels_mut();

        for y in 0..height {
            for x in 0..width / 2 {
                let nx = width - x - 1;
                unsafe {
                    core::ptr::swap(
                        &mut pixels[(x + y * width) as usize],
                        &mut pixels[(nx + y * width) as usize],
                    );
                }
            }
        }
    }

    /// Flip pixels around y axis
    pub fn flip_y(&mut self) {
        let width = self.width();
        let height = self.height();
        let pixels = self.pixels_mut();

        for y in 0..height / 2 {
            for x in 0..width {
                let ny = height - y - 1;
                unsafe {
                    core::ptr::swap(
                        &mut pixels[(x + y * width) as usize],
                        &mut pixels[(x + ny * width) as usize],
                    );
                }
            }
        }
    }

    /// Convert bitmap data type
    pub fn convert<R>(&self) -> Bitmap<R>
    where
        T: Copy,
        R: From<T>,
    {
        let mut bitmap = Bitmap::<R>::new(self.width(), self.height());
        bitmap
            .pixels_mut()
            .iter_mut()
            .zip(self.pixels().iter())
            .for_each(|(out_pixel, in_pixel)| *out_pixel = From::from(*in_pixel));
        bitmap
    }
}

impl<T> Bitmap<T> {
    pub(crate) fn as_raw(&self) -> *const u8 {
        unsafe { core::mem::transmute(self) }
    }

    pub(crate) fn as_raw_mut(&mut self) -> *mut u8 {
        unsafe { core::mem::transmute(self) }
    }
}

impl<'a, A, T> From<&'a Bitmap<A>> for Bitmap<T>
where
    A: Copy,
    T: From<A>,
{
    fn from(bitmap: &'a Bitmap<A>) -> Self {
        bitmap.convert()
    }
}
