use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use log::info;
use tfhe::integer::{gen_keys_radix, RadixCiphertextBig, RadixClientKey, ServerKey};
use tfhe::shortint::prelude::PARAM_MESSAGE_2_CARRY_2;

use crate::image::{EncryptedImage, PlaintextImage};

pub const NUM_BLOCKS: usize = 4;

pub type EncryptedImageData = RadixCiphertextBig;
pub type ServerKeyType = ServerKey;
pub type ClientKeyType = RadixClientKey;

pub fn generate_keys() -> (RadixClientKey, ServerKey) {
    info!("Generating keys");
    gen_keys_radix(&PARAM_MESSAGE_2_CARRY_2, NUM_BLOCKS)
}

pub fn generate_keys_to_file(
    client_key_path: &Path,
    server_key_path: &Path,
) -> Result<(RadixClientKey, ServerKey), Box<dyn Error>> {
    let (client_key, server_key) = generate_keys();

    let client_key_path = client_key_path.with_extension("key");
    let server_key_path = server_key_path.with_extension("key");
    info!(
        "Storing keys to {:?} and {:?}",
        client_key_path, server_key_path
    );

    let (client_key_file, server_key_file) = (
        File::create(client_key_path)?,
        File::create(server_key_path)?,
    );
    let (client_key_writer, server_key_writer) = (
        BufWriter::new(client_key_file),
        BufWriter::new(server_key_file),
    );
    bincode::serialize_into(client_key_writer, &client_key)?;
    bincode::serialize_into(server_key_writer, &server_key)?;

    Ok((client_key, server_key))
}

pub fn load_or_generate_keys(
    client_key_path: &Path,
    server_key_path: &Path,
) -> Result<(RadixClientKey, ServerKey), Box<dyn Error>> {
    let server_key_path = server_key_path.with_extension("key");
    let client_key_path = client_key_path.with_extension("key");

    if let (Ok(true), Ok(true)) = (client_key_path.try_exists(), server_key_path.try_exists()) {
        info!(
            "Loading keys from {:?} and {:?}",
            client_key_path, server_key_path
        );

        let (client_key_file, server_key_file) =
            (File::open(client_key_path)?, File::open(server_key_path)?);
        let (client_key_reader, server_key_reader) = (
            BufReader::new(client_key_file),
            BufReader::new(server_key_file),
        );
        return Ok((
            bincode::deserialize_from(client_key_reader)?,
            bincode::deserialize_from(server_key_reader)?,
        ));
    }

    info!("Keys not found, generating new keys");
    generate_keys_to_file(&client_key_path, &server_key_path)
}

pub fn encrypt_image(image: &PlaintextImage, key: &RadixClientKey) -> EncryptedImage {
    EncryptedImage::new(
        image.data.iter().map(|x| key.encrypt(*x as u64)).collect(),
        image.size.width,
        image.size.height,
        image.color_type,
    )
}

pub fn decrypt_image(image: &EncryptedImage, key: &RadixClientKey) -> PlaintextImage {
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
