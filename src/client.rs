use std::error::Error;
use std::net::TcpStream;

use crate::image::{EncryptedImage, Image};
use log::info;
use tfhe::shortint::{CiphertextBig, ClientKey};

use crate::message::Message;

#[derive(Debug)]
pub struct Client {
    address: String,
    key: ClientKey,
}

impl Client {
    /// Create a new connection to the given address.
    ///
    /// # Examples
    ///
    /// ```
    /// let connection = Connection::new("127.0.0.1:34347");
    /// ```
    pub fn new(address: &str, key: ClientKey) -> Self {
        Self {
            address: address.to_string(),
            key,
        }
    }

    /// Send a message to the server and wait for a response.
    ///
    /// Returns the message extracted from the response if there is one.
    ///
    /// # Examples
    ///
    /// ```
    /// # let connection = client::Connection::new("127.0.0.1:34347");
    /// let answer = connection.send_message(Message::Ping).unwrap();
    /// ```
    pub fn send_message(&self, message: Message) -> Result<Option<Message>, Box<dyn Error>> {
        let stream = TcpStream::connect(&self.address)?;

        info!("Sending {:?}", message);
        bincode::serialize_into(&stream, &message)?;
        if message.expect_answer() {
            let answer = bincode::deserialize_from(&stream)?;
            info!("Received answer {:?}", answer);

            return Ok(Some(answer));
        }

        Ok(None)
    }

    pub fn encrypt_image(&self, image: &Image) -> EncryptedImage {
        image.encrypt(&self.key)
    }

    pub fn encrypt(&self, number: u8) -> CiphertextBig {
        self.key.encrypt(number as u64)
    }

    pub fn decrypt(&self, number: &CiphertextBig) -> u8 {
        self.key.decrypt(number) as u8
    }
}
