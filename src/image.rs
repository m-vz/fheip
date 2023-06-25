use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::path::Path;

use log::debug;
use png::Decoder;
use serde::{Deserialize, Serialize};
use tfhe::shortint::{CiphertextBig, ClientKey};

pub mod rescaling;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Image<T> {
    pub data: Vec<T>,
    pub size: Size,
}

impl<T> Image<T> {
    pub fn new(data: Vec<T>, width: u16, height: u16) -> Self {
        Self {
            data,
            size: Size { width, height },
        }
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> Option<Vec<&T>> {
        let (x, y) = (x * 3, y * 3);
        let index = (x + y * self.size.width) as usize;

        vec![
            self.data.get(index),
            self.data.get(index + 1),
            self.data.get(index + 2),
        ]
        .into_iter()
        .collect::<Option<Vec<&T>>>()
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
        }
    }
}

impl Debug for PlaintextImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image ({:?}), data: {:?}", self.size, self.data)
    }
}

pub type EncryptedImage = Image<CiphertextBig>;

impl Debug for EncryptedImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encrypted image ({:?})", self.size)
    }
}
