use std::sync::Arc;
use std::thread;

use crate::crypt::{EncryptedImageData, ServerKeyType};

const ONE_THIRD: f32 = 1.0 / 3.0;

pub fn average_three(x: [&EncryptedImageData; 3], key: &ServerKeyType) -> EncryptedImageData {
    weight_multiplication(&add_three(x, key), ONE_THIRD, key)
}

pub fn add_three(x: [&EncryptedImageData; 3], key: &ServerKeyType) -> EncryptedImageData {
    key.unchecked_add(&key.unchecked_add(x[0], x[1]), x[2])
}

pub fn invert_u8(x: &EncryptedImageData, key: &ServerKeyType) -> EncryptedImageData {
    key.neg_parallelized(&key.scalar_sub_parallelized(x, 255_u64))
}

pub fn bicubic_interpolation(
    a: EncryptedImageData,
    b: EncryptedImageData,
    c: EncryptedImageData,
    d: EncryptedImageData,
    x_weight: f32,
    y_weight: f32,
    key: Arc<ServerKeyType>,
) -> EncryptedImageData {
    let e_key = key.clone();
    let e = thread::spawn(move || linear_interpolation(a, b, x_weight, e_key));

    let f_key = key.clone();
    let f = thread::spawn(move || linear_interpolation(c, d, x_weight, f_key));

    let (e, f) = (e.join().unwrap(), f.join().unwrap());

    linear_interpolation(e, f, y_weight, key)
}

pub fn linear_interpolation(
    x: EncryptedImageData,
    y: EncryptedImageData,
    weight: f32,
    key: Arc<ServerKeyType>,
) -> EncryptedImageData {
    let one_minus_weight = 1.0 - weight;

    let x_key = key.clone();
    let x_scaled = thread::spawn(move || weight_multiplication(&x, one_minus_weight, &x_key));

    let y_key = key.clone();
    let y_scaled = thread::spawn(move || weight_multiplication(&y, weight, &y_key));

    let (x_scaled, y_scaled) = (x_scaled.join().unwrap(), y_scaled.join().unwrap());

    key.add_parallelized(&x_scaled, &y_scaled)
}

pub fn weight_multiplication(
    x: &EncryptedImageData,
    weight: f32,
    key: &ServerKeyType,
) -> EncryptedImageData {
    if weight == 1.0 {
        return x.clone();
    }
    if weight == 0.0 {
        return key.unchecked_small_scalar_mul_parallelized(x, 0);
    }

    let weight = weight_to_integer(weight);
    key.unchecked_scalar_right_shift_parallelized(&key.scalar_mul_parallelized(x, weight), 8)
}

fn weight_to_integer(weight: f32) -> u64 {
    (f64::from(weight) * f64::from(1 << 8)).round() as u64
}
