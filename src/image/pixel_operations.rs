use crate::crypt::ServerKeyType;
use crate::image::{EncryptedImage, Image};

pub fn invert(image: &EncryptedImage, key: &ServerKeyType) -> EncryptedImage {
    Image::new(
        image
            .data
            .iter()
            .map(|x| key.neg_parallelized(&key.scalar_sub_parallelized(x, 255_u64)))
            .collect(),
        image.size.width,
        image.size.height,
        image.color_type,
    )
}
