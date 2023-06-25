use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::path::Path;

use log::debug;
use png::{ColorType, Decoder};
use serde::{Deserialize, Serialize};
use tfhe::shortint::{CiphertextBig, ClientKey};

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

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Image<T: Clone> {
    data: Vec<T>,
    size: Size,
    color_type: ColorType,
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
        let components = self.color_type.into();
        let (x, y) = (x * components, y * components);
        let index = (x + y * self.size.width) as usize;
        let mut pixel = Vec::new();

        for component in 0..components {
            pixel.push(self.data.get(index + component as usize));
        }

        pixel.into_iter().collect::<Option<Vec<&T>>>()
    }
}

pub type PlaintextImage = Image<u8>;

impl PlaintextImage {
    pub fn load(file_path: &Path) -> Result<PlaintextImage, Box<dyn Error>> {
        let decoder = Decoder::new(File::open(file_path)?);
        let mut reader = decoder.read_info()?;
        let mut buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buffer)?;
        let image = Image::new(
            buffer[..info.buffer_size()].to_vec(),
            info.width as u16,
            info.height as u16,
            info.color_type.into(),
        );
        debug!("Loaded {:?}", image);

        Ok(image)
    }

    pub fn encrypt(&self, key: &ClientKey) -> EncryptedImage {
        EncryptedImage {
            data: self
                .data
                .clone()
                .iter_mut()
                .map(|x| key.encrypt(*x as u64))
                .collect(),
            size: self.size,
            color_type: self.color_type,
        }
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

pub type EncryptedImage = Image<CiphertextBig>;

impl EncryptedImage {
    pub fn decrypt(&self, key: &ClientKey) -> PlaintextImage {
        PlaintextImage {
            data: self.data.iter().map(|x| key.decrypt(x) as u8).collect(),
            size: self.size,
            color_type: self.color_type,
        }
    }
}

impl Debug for EncryptedImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Encrypted image ({:?}, {:?})",
            self.size, self.color_type
        )
    }
}
