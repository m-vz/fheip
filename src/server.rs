use std::error::Error;
use std::net::{TcpListener, TcpStream};

use log::info;

use crate::message::Message;

/// Start a server listening on the given address.
///
/// # Arguments
///
/// * `address`: The address to listen on.
///
/// # Examples
///
/// ```
/// server::start("127.0.0.1:34347").unwrap();
/// ```
pub fn start(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;

    info!("Server listening on {}", address);

    for stream in listener.incoming() {
        if handle_connection(stream?)? {
            info!("Shutting down");
            break;
        }
    }

    Ok(())
}

/// Handle a message received from a TCP connection.
///
/// Returns `true` if the server should shut down.
///
/// # Arguments
///
/// * `stream`: The TCP stream to handle.
///
fn handle_connection(stream: TcpStream) -> Result<bool, Box<dyn Error>> {
    info!("New connection from {}", stream.peer_addr()?);

    let message: Message = bincode::deserialize_from(&stream)?;
    info!("Received {:?}", message);
    match message {
        Message::Ping => bincode::serialize_into(&stream, &Message::Pong)?,
        Message::Pong => {}
        Message::Shutdown => return Ok(true),
    }

    Ok(false)
}
