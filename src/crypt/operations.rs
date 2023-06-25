use crate::crypt::{EncryptedImageData, ServerKeyType};
use log::trace;

pub fn bicubic_interpolation(
    a: &EncryptedImageData,
    b: &EncryptedImageData,
    c: &EncryptedImageData,
    d: &EncryptedImageData,
    x_weight: f32,
    y_weight: f32,
    key: &ServerKeyType,
) -> EncryptedImageData {
    trace!("Bilinear: e");
    let e = linear_interpolation(a, b, x_weight, key);
    trace!("Bilinear: f");
    let f = linear_interpolation(c, d, x_weight, key);
    trace!("Bilinear: g");
    linear_interpolation(&e, &f, y_weight, key)
}

pub fn linear_interpolation(
    x: &EncryptedImageData,
    y: &EncryptedImageData,
    weight: f32,
    key: &ServerKeyType,
) -> EncryptedImageData {
    let one_minus_weight = 1.0 - weight;
    trace!("Linear: x");
    let x_scaled = weight_multiplication(x, one_minus_weight, key);
    trace!("Linear: y");
    let y_scaled = weight_multiplication(y, weight, key);
    trace!("Linear: add");
    key.add_parallelized(&x_scaled, &y_scaled)
}

pub fn weight_multiplication(
    x: &EncryptedImageData,
    weight: f32,
    key: &ServerKeyType,
) -> EncryptedImageData {
    let weight = weight_to_integer(weight);
    key.scalar_right_shift_parallelized(&key.scalar_mul_parallelized(x, weight), 8)
}

fn weight_to_integer(weight: f32) -> u64 {
    (f64::from(weight) * f64::from(1 << 8)).round() as u64
}
