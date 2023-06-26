use crate::crypt::operations::weight_multiplication;
use crate::crypt::ServerKeyType;
use crate::image::{ColorType, EncryptedImage, Image};
use log::trace;
use std::sync::Arc;

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

pub fn grayscale(image: &EncryptedImage, key: &ServerKeyType) -> Option<EncryptedImage> {
    match image.color_type {
        ColorType::Rgb | ColorType::Rgba => {
            let mut grayscale_data =
                Vec::with_capacity((image.size.width * image.size.height) as usize);
            let multiplication_key = Arc::new(key.clone());

            for y in 0..image.size.height {
                for x in 0..image.size.width {
                    trace!("Pixel: ({}, {})", x, y);

                    let pixel = image.get_pixel(x, y).unwrap();
                    const ONE_THIRD: f32 = 1.0 / 3.0;

                    let added = key.unchecked_add(&key.unchecked_add(pixel[0], pixel[1]), pixel[2]);
                    grayscale_data.push(weight_multiplication(
                        &added,
                        ONE_THIRD,
                        multiplication_key.clone(),
                    ));

                    if image.color_type == ColorType::Rgba {
                        grayscale_data.push(pixel[3].clone());
                    }
                }
            }

            Some(Image::new(
                grayscale_data,
                image.size.width,
                image.size.height,
                if image.color_type == ColorType::Rgba {
                    ColorType::GrayscaleAlpha
                } else {
                    ColorType::Grayscale
                },
            ))
        }
        _ => None,
    }
}
