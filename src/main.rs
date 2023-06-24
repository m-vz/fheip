use crate::encryption::generate_keys;
use log::info;
use std::error::Error;
use std::path::Path;
use std::thread;

use crate::message::Message;

mod client;
mod encryption;
mod exploration;
mod image;
mod message;
mod server;

const ADDRESS: &str = "127.0.0.1:34347";

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    image::load_png(Path::new("data/test-4x4.png"))?;

    let (client_key, server_key) = generate_keys();
    let join_handle = thread::spawn(|| server::Server::new(server_key).start(ADDRESS).unwrap());

    let client = client::Client::new(ADDRESS, client_key);
    client.send_message(Message::Ping)?;
    let result = client.send_message(Message::Add(client.encrypt(6), 1))?;
    if let Some(Message::AdditionResult(number)) = result {
        info!("6 + 1 = {}", client.decrypt(&number));
    }
    client.send_message(Message::Shutdown)?;

    join_handle.join().unwrap();

    Ok(())
}
