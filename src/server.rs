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
/// * `stream`: The TCP stream to handle
///
fn handle_connection(stream: TcpStream) -> Result<bool, Box<dyn Error>> {
    let message: Message = bincode::deserialize_from(&stream)?;
    info!("Received {:?}", message);
    match message {
        Message::Ping => send_message(Message::Pong, &stream)?,
        Message::Pong => {}
        Message::Shutdown => {
            send_message(Message::Shutdown, &stream)?;
            return Ok(true);
        }
    }

    Ok(false)
}

fn send_message(message: Message, stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    bincode::serialize_into(stream, &message)?;

    Ok(())
}
