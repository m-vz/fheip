use std::error::Error;
use std::net::{TcpListener, TcpStream};

use log::info;
use tfhe::shortint::ServerKey;

use crate::message::Message;

/// Start a server listening on the given address.
///
/// # Arguments
///
/// * `address`: The address to listen on
///
/// # Examples
///
/// ```
/// server::start("127.0.0.1:34347").unwrap();
/// ```
pub fn start(address: &str, key: ServerKey) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;
    info!("Server listening on {}", address);

    for stream in listener.incoming() {
        let stream = stream?;
        let message: Message = bincode::deserialize_from(&stream)?;
        info!("Received {:?}", message);

        match message {
            Message::Ping => send_message(Message::Pong, &stream)?,
            Message::Pong => {}
            Message::Shutdown => break,
        }
    }
    info!("Shutting down");

    Ok(())
}

fn send_message(message: Message, stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    bincode::serialize_into(stream, &message)?;

    Ok(())
}
