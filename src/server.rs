use std::error::Error;
use std::net::{TcpListener, TcpStream};

use log::info;
use tfhe::shortint::ServerKey;

use crate::image::rescaling::rescale;
use crate::image::EncryptedImage;
use crate::message::Message;

#[derive(Debug)]
pub struct Server {
    key: ServerKey,
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
    pub fn new(key: ServerKey) -> Self {
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
            let message: Message = bincode::deserialize_from(&stream)?;
            info!("Received {:?}", message);

            if !match message {
                Message::Rescale(_, _) => self.check_image(&stream)?,
                _ => true,
            } {
                continue;
            }

            match message {
                Message::Ping => self.send_message(Message::Pong, &stream)?,
                Message::Shutdown => break,
                Message::Add(number, scalar) => self.send_message(
                    Message::AdditionResult(self.key.unchecked_scalar_add(&number, scalar)),
                    &stream,
                )?,
                Message::Image(image) => self.image = Some(image),
                Message::Rescale(size, interpolation_type) => {
                    if let Some(image) = &self.image {
                        let rescaled_image = rescale(image, size, interpolation_type);
                        self.send_message(Message::Image(rescaled_image), &stream)?;
                    }
                }
                Message::Pong | Message::AdditionResult(_) | Message::NoImage => {}
            }
        }
        info!("Shutting down");

        Ok(())
    }

    fn send_message(&self, message: Message, stream: &TcpStream) -> Result<(), Box<dyn Error>> {
        bincode::serialize_into(stream, &message)?;

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
