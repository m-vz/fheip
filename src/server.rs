use std::error::Error;
use std::net::{TcpListener, TcpStream};

use log::info;
use tfhe::shortint::ServerKey;

use crate::message::Message;

#[derive(Debug)]
pub struct Server {
    key: ServerKey,
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
        Self { key }
    }

    /// Start a server listening on the given address.
    ///
    /// # Examples
    ///
    /// ```
    /// server::start("127.0.0.1:34347").unwrap();
    /// ```
    pub fn start(&self, address: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(address)?;
        info!("Server listening on {}", address);

        for stream in listener.incoming() {
            let stream = stream?;
            let message: Message = bincode::deserialize_from(&stream)?;
            info!("Received {:?}", message);

            match message {
                Message::Ping => self.send_message(Message::Pong, &stream)?,
                Message::Shutdown => break,
                Message::Add(number, scalar) => self.send_message(
                    Message::AdditionResult(self.key.unchecked_scalar_add(&number, scalar)),
                    &stream,
                )?,
                Message::Pong | Message::AdditionResult(_) => {}
            }
        }
        info!("Shutting down");

        Ok(())
    }

    fn send_message(&self, message: Message, stream: &TcpStream) -> Result<(), Box<dyn Error>> {
        bincode::serialize_into(stream, &message)?;

        Ok(())
    }
}
