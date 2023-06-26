use crate::crypt::ServerKeyType;
use crate::image::{ColorType, EncryptedImage, Image};

pub fn invert(image: &EncryptedImage, key: &ServerKeyType) -> EncryptedImage {
    Image::new(
        match image.color_type {
            ColorType::GrayscaleAlpha | ColorType::Rgba => {
                let mut inverted_data = Vec::with_capacity(
                    (image.size.width * image.size.height * image.channel_count()) as usize,
                );

                for y in 0..image.size.height {
                    for x in 0..image.size.width {
                        let pixel = image.get_pixel(x, y).unwrap();
                        let mut pixel = pixel.iter();

                        // invert rgb/grayscale values
                        (0..image.channel_count() - 1).for_each(|_| {
                            inverted_data.push(key.neg_parallelized(
                                &key.scalar_sub_parallelized(pixel.next().unwrap(), 255_u64),
                            ))
                        });
                        // copy alpha value
                        inverted_data.push((*pixel.next().unwrap()).clone());
                    }
                }

                inverted_data
            }
            ColorType::Grayscale | ColorType::Indexed | ColorType::Rgb => image
                .data
                .iter()
                .map(|x| key.neg_parallelized(&key.scalar_sub_parallelized(x, 255_u64)))
                .collect(),
        },
        image.size.width,
        image.size.height,
        image.color_type,
    )
}
