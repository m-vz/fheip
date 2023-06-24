use log::{debug, trace};
use png::Decoder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

pub fn load_png(file_path: &Path) -> Result<Image, Box<dyn Error>> {
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
