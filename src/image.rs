use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::path::Path;

use log::{debug, trace};
use png::Decoder;
use serde::{Deserialize, Serialize};
use tfhe::shortint::{CiphertextBig, ClientKey};

pub mod rescaling;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
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
    data: Vec<T>,
    size: Size,
}

impl<T> Image<T> {
    pub fn new(data: Vec<T>, width: u32, height: u32) -> Self {
        Self {
            data,
            size: Size { width, height },
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&T> {
        self.data.get((x + y * self.size.width) as usize)
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
            info.width,
            info.height,
        );
        debug!("Loaded {:?}", image);
        trace!("Image data: {:?}", image.data);

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
            size: Size {
                width: self.size.width,
                height: self.size.height,
            },
        }
    }
}

impl Debug for PlaintextImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image ({:?})", self.size)
    }
}

pub type EncryptedImage = Image<CiphertextBig>;

impl Debug for EncryptedImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encrypted image ({:?})", self.size)
    }
}
