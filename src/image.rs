use log::{debug, trace};
use png::Decoder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::path::Path;
use tfhe::shortint::{CiphertextBig, ClientKey};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn load(file_path: &Path) -> Result<Image, Box<dyn Error>> {
        let decoder = Decoder::new(File::open(file_path)?);
        let mut reader = decoder.read_info()?;
        let mut buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buffer)?;
        let image = Image {
            data: buffer[..info.buffer_size()].to_vec(),
            width: info.width,
            height: info.height,
        };
        debug!("Loaded image ({}x{})", image.width, image.height);
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
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedImage {
    data: Vec<CiphertextBig>,
    width: u32,
    height: u32,
}

impl Debug for EncryptedImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EncryptedImage ({}x{})", self.width, self.height)
    }
}
