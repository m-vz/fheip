use log::info;
use std::error::Error;
use std::net::TcpStream;

use crate::message::Message;

#[derive(Debug)]
pub struct Connection {
    address: String,
}

impl Connection {
    /// Create a new connection to the given address.
    ///
    /// # Arguments
    ///
    /// * `address`: The address to connect to
    ///
    /// # Examples
    ///
    /// ```
    /// let connection = Connection::new("127.0.0.1:34347");
    /// ```
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    /// Send a message to the server and wait for a response.
    ///
    /// Returns the message from the response.
    ///
    /// # Arguments
    ///
    /// * `message`: The message to send
    ///
    /// # Examples
    ///
    /// ```
    /// # let connection = client::Connection::new("127.0.0.1:34347");
    /// let answer = connection.send_message(Message::Ping).unwrap();
    /// ```
    pub fn send_message(&self, message: Message) -> Result<Option<Message>, Box<dyn Error>> {
        let stream = TcpStream::connect(&self.address)?;

        bincode::serialize_into(&stream, &message)?;
        if message.expect_answer() {
            let answer = bincode::deserialize_from(&stream)?;
            info!("Received answer {:?}", answer);

            return Ok(Some(answer));
        }

        Ok(None)
    }
}
