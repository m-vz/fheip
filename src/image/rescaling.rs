use crate::crypt::ServerKeyType;
use serde::{Deserialize, Serialize};

use crate::image::{EncryptedImage, Image, Size};

#[derive(Debug)]
struct Scale {
    width: f32,
    height: f32,
}

impl Scale {
    fn from_sizes(from: &Size, to: &Size) -> Scale {
        Scale {
            width: f32::from(from.width) / f32::from(to.width),
            height: f32::from(from.height) / f32::from(to.height),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpolationType {
    Nearest,
}

pub fn rescale(
    image: &EncryptedImage,
    key: &ServerKeyType,
    new_size: Size,
    interpolation_type: InterpolationType,
) -> EncryptedImage {
    match interpolation_type {
        InterpolationType::Nearest => nearest(image, new_size),
    }
}

fn nearest(image: &EncryptedImage, new_size: Size) -> EncryptedImage {
    let scale = Scale::from_sizes(&image.size, &new_size);
    let mut rescaled_data = Vec::with_capacity((new_size.width * new_size.height) as usize);

    for y in 0..new_size.height {
        for x in 0..new_size.width {
            let (x, y) = (
                (x as f32 * scale.width) as u16,
                (y as f32 * scale.height) as u16,
            );
            let pixel = image.get_pixel(x, y);
            rescaled_data.extend(pixel.unwrap().into_iter().cloned());
        }
    }

    Image::new(
        rescaled_data,
        new_size.width,
        new_size.height,
        image.color_type,
    )
}
