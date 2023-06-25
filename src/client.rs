use std::error::Error;
use std::io::BufWriter;
use std::net::TcpStream;

use log::info;

use crate::crypt::{decrypt_image, encrypt_image, ClientKeyType};
use crate::image::{EncryptedImage, PlaintextImage};
use crate::message::Message;

#[derive(Debug)]
pub struct Client {
    address: String,
    key: ClientKeyType,
}

impl Client {
    /// Create a new connection to the given address.
    ///
    /// # Examples
    ///
    /// ```
    /// let connection = Connection::new("127.0.0.1:34347");
    /// ```
    pub fn new(address: &str, key: ClientKeyType) -> Self {
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
        let writer = BufWriter::new(&stream);

        info!("Sending {:?}", message);
        bincode::serialize_into(writer, &message)?;
        if message.expect_answer() {
            let answer = bincode::deserialize_from(&stream)?;
            info!("Received answer {:?}", answer);

            return Ok(Some(answer));
        }

        Ok(None)
    }

    pub fn encrypt_image(&self, image: &PlaintextImage) -> EncryptedImage {
        encrypt_image(image, &self.key)
    }

    pub fn decrypt_image(&self, image: &EncryptedImage) -> PlaintextImage {
        decrypt_image(image, &self.key)
    }
}
