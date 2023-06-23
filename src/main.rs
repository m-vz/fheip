use std::error::Error;
use std::thread;

use crate::message::Message;

mod client;
mod exploration;
mod message;
mod server;

const ADDRESS: &str = "127.0.0.1:34347";

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let server_join_handle = thread::spawn(|| server::start(ADDRESS).unwrap());
    let connection = client::Connection::new(ADDRESS);

    assert_eq!(connection.send_message(Message::Ping)?, Some(Message::Pong));
    connection.send_message(Message::Shutdown)?;

    server_join_handle.join().unwrap();

    Ok(())
}
