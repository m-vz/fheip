use log::info;
use std::error::Error;
use std::net::TcpStream;
use std::thread;

use crate::message::Message;

mod exploration;
mod message;
mod server;

const ADDRESS: &str = "127.0.0.1:34347";

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let server_join_handle = thread::spawn(|| server::start(ADDRESS).unwrap());

    let stream = TcpStream::connect(ADDRESS)?;
    bincode::serialize_into(&stream, &Message::Ping)?;
    let answer: Message = bincode::deserialize_from(&stream)?;
    info!("Received {:?}", answer);
    assert_eq!(answer, Message::Pong);

    let stream = TcpStream::connect(ADDRESS)?;
    bincode::serialize_into(&stream, &Message::Shutdown)?;

    server_join_handle.join().unwrap();

    Ok(())
}
