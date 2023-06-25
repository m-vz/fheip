use log::trace;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::crypt::operations::bicubic_interpolation;
use crate::crypt::ServerKeyType;
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum InterpolationType {
    Nearest,
    Bilinear,
}

pub fn rescale(
    image: &EncryptedImage,
    key: &ServerKeyType,
    new_size: Size,
    interpolation_type: InterpolationType,
) -> EncryptedImage {
    match interpolation_type {
        InterpolationType::Nearest => nearest(image, new_size),
        InterpolationType::Bilinear => bilinear(image, key, new_size),
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

fn bilinear(image: &EncryptedImage, key: &ServerKeyType, new_size: Size) -> EncryptedImage {
    let scale = Scale::from_sizes(&image.size.minus_one(), &new_size.minus_one());
    let mut rescaled_data = Vec::with_capacity((new_size.width * new_size.height) as usize);

    for y in 0..new_size.height {
        for x in 0..new_size.width {
            // the following illustrates the values to calculate the bilinear interpolation:
            //
            //            a         e       b
            //            *---------*-------* - y_bounds.0
            //            |         |       |
            //            |       g * (x,y) |
            //            |         |       |
            //            |         |       |
            //            |         |       |
            //            *---------*-------* - y_bounds.1
            //            c         f       d
            // x_bounds.0 |                 | x_bounds.1

            trace!("Pixel: ({}, {})", x, y);

            let (x, y) = (x as f32 * scale.width, y as f32 * scale.height);
            let (x_bounds, y_bounds) = (
                (x.floor() as u16, x.ceil() as u16),
                (y.floor() as u16, y.ceil() as u16),
            );
            let (x_weight, y_weight) = (x - x_bounds.0 as f32, y - y_bounds.0 as f32);
            let (a, b, c, d) = (
                image.get_pixel(x_bounds.0, y_bounds.0).unwrap(),
                image.get_pixel(x_bounds.1, y_bounds.0).unwrap(),
                image.get_pixel(x_bounds.0, y_bounds.1).unwrap(),
                image.get_pixel(x_bounds.1, y_bounds.1).unwrap(),
            );

            let key = Arc::new(key.clone());
            let components: u16 = image.color_type.into();
            let components = components as usize;
            let mut pixel = Vec::with_capacity(components);
            for i in 0..components {
                trace!("Component: {}", i);
                pixel.push(bicubic_interpolation(
                    a[i].clone(),
                    b[i].clone(),
                    c[i].clone(),
                    d[i].clone(),
                    x_weight,
                    y_weight,
                    key.clone(),
                ));
            }
            rescaled_data.extend(pixel);
        }
    }

    Image::new(
        rescaled_data,
        new_size.width,
        new_size.height,
        image.color_type,
    )
}
