use crate::encryption::generate_keys;
use std::error::Error;
use std::thread;

use crate::message::Message;

mod client;
mod encryption;
mod exploration;
mod message;
mod server;

const ADDRESS: &str = "127.0.0.1:34347";

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let (client_key, server_key) = generate_keys();
    let server_join_handle = thread::spawn(|| server::start(ADDRESS, server_key).unwrap());

    let client = client::Client::new(ADDRESS, client_key);
    client.send_message(Message::Ping)?;
    client.send_message(Message::Shutdown)?;

    server_join_handle.join().unwrap();

    Ok(())
}
