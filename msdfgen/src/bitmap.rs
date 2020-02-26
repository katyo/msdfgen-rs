mod gray;
mod rgb;

pub use self::gray::*;
pub use self::rgb::*;

#[cfg(feature = "png")]
mod png;

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

    /// Get pixel data slice for reading from
    pub fn pixels(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(
            self.pixels,
            (self.width * self.height) as usize,
        ) }
    }

    /// Get pixel data slice for writing to
    pub fn pixels_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(
            self.pixels,
            (self.width * self.height) as usize,
        ) }
    }

    /// Get raw pixels data for reading from
    pub fn raw_pixels(&self) -> &[u8] {
        let pixels = self.pixels();

        unsafe {
            core::slice::from_raw_parts(
                pixels.as_ptr() as _,
                pixels.len() * core::mem::size_of::<T>()
            )
        }
    }

    /// Get raw pixels data for writing to
    pub fn raw_pixels_mut(&mut self) -> &mut [u8] {
        let pixels = self.pixels_mut();

        unsafe {
            core::slice::from_raw_parts_mut(
                pixels.as_mut_ptr() as _,
                pixels.len() * core::mem::size_of::<T>()
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

    /// Convert bitmap data type
    pub fn convert<R>(&self) -> Bitmap<R>
    where
        T: Copy,
        R: From<T>,
    {
        let mut bitmap = Bitmap::<R>::new(self.width(), self.height());
        bitmap.pixels_mut().iter_mut().zip(self.pixels().iter())
            .for_each(|(out_pixel, in_pixel)|
                      *out_pixel = From::from(*in_pixel));
        bitmap
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

impl<'a, A, T> From<&'a Bitmap<A>> for Bitmap<T>
where
    A: Copy,
    T: From<A>,
{
    fn from(bitmap: &'a Bitmap<A>) -> Self {
        bitmap.convert()
    }
}
