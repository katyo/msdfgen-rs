use std::io::{Read, Write};
use png;
use super::{Bitmap, Gray, RGB};

pub trait PngColorType {
    type PngPixelType;
    const PNG_COLOR_TYPE: png::ColorType;
}

impl<T> PngColorType for Gray<T> {
    type PngPixelType = Gray<u8>;
    const PNG_COLOR_TYPE: png::ColorType = png::ColorType::Grayscale;
}

impl<T> PngColorType for RGB<T> {
    type PngPixelType = RGB<u8>;
    const PNG_COLOR_TYPE: png::ColorType = png::ColorType::RGB;
}

impl<T> Bitmap<T>
where
    T: PngColorType + Copy,
    T::PngPixelType: From<T>,
{
    /// Save bitmap as png
    pub fn write_png(&self, writer: impl Write) -> Result<(), png::EncodingError> {
        let mut encoder = png::Encoder::new(
            writer, self.width(), self.height(),
        );

        encoder.set_color(T::PNG_COLOR_TYPE);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        let bitmap = Bitmap::<T::PngPixelType>::from(self);

        writer.write_image_data(bitmap.raw_pixels())
    }
}

impl<T> Bitmap<T>
where
    T: PngColorType,
    T: From<<T as PngColorType>::PngPixelType>,
    T::PngPixelType: Copy,
{
    /// Load bitmap from png
    pub fn read_png(reader: impl Read) -> Result<Bitmap<T>, png::DecodingError> {
        let decoder = png::Decoder::new(reader);
        let (info, mut reader) = decoder.read_info()?;

        if info.bit_depth != png::BitDepth::Eight {
            return Err(png::DecodingError::Other("Bit depth should be 8".into()));
        }

        if info.color_type != T::PNG_COLOR_TYPE {
            return Err(png::DecodingError::Other("Color type mismatch".into()));
        }

        let mut bitmap = Bitmap::<T::PngPixelType>::new(info.width, info.height);

        reader.next_frame(bitmap.raw_pixels_mut())?;

        Ok(Bitmap::<T>::from(&bitmap))
    }
}
