use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use log::debug;
use png::{BitDepth, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use crate::crypt::EncryptedImageData;

pub mod pixel_operations;
pub mod rescaling;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum ColorType {
    Grayscale,
    Indexed,
    GrayscaleAlpha,
    Rgb,
    Rgba,
}

impl From<ColorType> for png::ColorType {
    fn from(value: ColorType) -> Self {
        match value {
            ColorType::Grayscale => png::ColorType::Grayscale,
            ColorType::Indexed => png::ColorType::Indexed,
            ColorType::GrayscaleAlpha => png::ColorType::GrayscaleAlpha,
            ColorType::Rgb => png::ColorType::Rgb,
            ColorType::Rgba => png::ColorType::Rgba,
        }
    }
}

impl From<png::ColorType> for ColorType {
    fn from(value: png::ColorType) -> Self {
        match value {
            png::ColorType::Grayscale => ColorType::Grayscale,
            png::ColorType::Indexed => ColorType::Indexed,
            png::ColorType::GrayscaleAlpha => ColorType::GrayscaleAlpha,
            png::ColorType::Rgb => ColorType::Rgb,
            png::ColorType::Rgba => ColorType::Rgba,
        }
    }
}

impl From<ColorType> for u16 {
    fn from(value: ColorType) -> Self {
        match value {
            ColorType::Grayscale | ColorType::Indexed => 1,
            ColorType::GrayscaleAlpha => 2,
            ColorType::Rgb => 3,
            ColorType::Rgba => 4,
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn minus_one(&self) -> Self {
        Self {
            width: if self.width > 0 { self.width - 1 } else { 0 },
            height: if self.height > 0 { self.height - 1 } else { 0 },
        }
    }
}

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Image<T: Clone> {
    pub data: Vec<T>,
    pub size: Size,
    pub color_type: ColorType,
}

impl<T: Clone> Image<T> {
    pub fn new(data: Vec<T>, width: u16, height: u16, color_type: ColorType) -> Self {
        Self {
            data,
            size: Size { width, height },
            color_type,
        }
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> Option<Vec<&T>> {
        let (x, y) = (x * self.channel_count(), y * self.channel_count());
        let index = (x + y * self.size.width) as usize;
        let mut pixel = Vec::new();

        for component in 0..self.channel_count() {
            pixel.push(self.data.get(index + component as usize));
        }

        pixel.into_iter().collect::<Option<Vec<&T>>>()
    }

    pub fn channel_count(&self) -> u16 {
        self.color_type.into()
    }
}

pub type PlaintextImage = Image<u8>;

impl PlaintextImage {
    pub fn load(file_path: &Path) -> Result<PlaintextImage, Box<dyn Error>> {
        let decoder = Decoder::new(File::open(file_path)?);
        let mut reader = decoder.read_info()?;
        let mut buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buffer)?;

        if info.bit_depth != BitDepth::Eight {
            unimplemented!("Only 8-bit images are supported");
        }

        let image = Image::new(
            buffer[..info.buffer_size()].to_vec(),
            info.width as u16,
            info.height as u16,
            info.color_type.into(),
        );
        debug!("Loaded {:?} from {:?}", image, file_path);

        Ok(image)
    }

    pub fn save(&self, file_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut writer = BufWriter::new(File::create(file_path)?);
        let mut encoder =
            Encoder::new(&mut writer, self.size.width as u32, self.size.height as u32);
        encoder.set_color(self.color_type.into());
        encoder.set_depth(BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // from https://docs.rs/png/0.17.9/png/#using-the-encoder
        encoder.set_source_chromaticities(png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        )); // from https://docs.rs/png/0.17.9/png/#using-the-encoder
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&self.data)?;
        debug!("Wrote {:?} to {:?}", self, file_path);

        Ok(())
    }
}

impl Debug for PlaintextImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image ({:?}, {:?}), data: {:?}",
            self.size, self.color_type, self.data
        )
    }
}

pub type EncryptedImage = Image<EncryptedImageData>;

impl Debug for EncryptedImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Encrypted image ({:?}, {:?})",
            self.size, self.color_type
        )
    }
}
