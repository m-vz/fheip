use tfhe::integer::{RadixCiphertextBig, RadixClientKey, ServerKey};

use crate::image::{EncryptedImage, PlaintextImage};

pub mod key;
pub mod operations;

pub const NUM_BLOCKS: usize = 8;

pub type EncryptedImageData = RadixCiphertextBig;
pub type ServerKeyType = ServerKey;
pub type ClientKeyType = RadixClientKey;

pub fn encrypt_image(image: &PlaintextImage, key: &ClientKeyType) -> EncryptedImage {
    EncryptedImage::new(
        image.data.iter().map(|x| key.encrypt(*x as u64)).collect(),
        image.size.width,
        image.size.height,
        image.color_type,
    )
}

pub fn decrypt_image(image: &EncryptedImage, key: &ClientKeyType) -> PlaintextImage {
    PlaintextImage::new(
        image
            .data
            .iter()
            .map(|x| key.decrypt::<u64, _>(x) as u8)
            .collect::<Vec<u8>>(),
        image.size.width,
        image.size.height,
        image.color_type,
    )
}
