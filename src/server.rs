use std::error::Error;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

use log::info;

use crate::crypt::ServerKeyType;
use crate::image::pixel_operations::{grayscale, invert};
use crate::image::rescaling::rescale;
use crate::image::EncryptedImage;
use crate::message::Message;

pub struct Server {
    key: ServerKeyType,
    image: Option<EncryptedImage>,
}

impl Server {
    /// Create a new server listening on the given address.
    ///
    /// # Examples
    ///
    /// ```
    /// server::new(server_key).unwrap();
    /// ```
    pub fn new(key: ServerKeyType) -> Self {
        Self { key, image: None }
    }

    /// Start a server listening on the given address.
    ///
    /// # Examples
    ///
    /// ```
    /// server::start("127.0.0.1:34347").unwrap();
    /// ```
    pub fn start(&mut self, address: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(address)?;
        info!("Server listening on {}", address);

        for stream in listener.incoming() {
            let stream = stream?;
            let reader = BufReader::new(&stream);
            let message: Message = bincode::deserialize_from(reader)?;
            let mut response = None;
            info!("Received {:?}", message);

            if !match message {
                Message::Rescale(_, _) | Message::Invert | Message::Grayscale => {
                    self.check_image(&stream)?
                }
                _ => true,
            } {
                continue;
            }

            match message {
                Message::Ping => self.send_message(Message::Pong, &stream)?,
                Message::Shutdown => break,
                Message::Image(image) => self.image = Some(image),
                Message::Rescale(size, interpolation_type) => {
                    if let Some(image) = &self.image {
                        response = Some(Message::Image(rescale(
                            image,
                            &self.key,
                            size,
                            interpolation_type,
                        )));
                    }
                }
                Message::Invert => {
                    if let Some(image) = &self.image {
                        response = Some(Message::Image(invert(image, &self.key)));
                    }
                }
                Message::Grayscale => {
                    if let Some(image) = &self.image {
                        if let Some(grayscale_image) = grayscale(image, &self.key) {
                            response = Some(Message::Image(grayscale_image));
                        }
                    }
                }
                Message::Pong | Message::NoImage => {}
            }

            if let Some(response_message) = response {
                self.send_message(response_message, &stream)?;
            }
        }
        info!("Shutting down");

        Ok(())
    }

    fn send_message(&self, message: Message, stream: &TcpStream) -> Result<(), Box<dyn Error>> {
        let writer = BufWriter::new(stream);
        bincode::serialize_into(writer, &message)?;

        Ok(())
    }

    fn check_image(&self, stream: &TcpStream) -> Result<bool, Box<dyn Error>> {
        if self.image.is_none() {
            info!("Has no image stored, informing client");
            self.send_message(Message::NoImage, stream)?;

            return Ok(false);
        }

        Ok(true)
    }
}
