use std::error::Error;
use std::path::Path;
use std::thread;

use log::info;

use crate::client::Client;
use crate::encryption::generate_keys;
use crate::image::rescaling::InterpolationType;
use crate::image::{Image, Size};
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

    let (client_key, server_key) = generate_keys();
    let join_handle = thread::spawn(|| server::Server::new(server_key).start(ADDRESS).unwrap());
    let client = Client::new(ADDRESS, client_key);
    let image = Image::load(Path::new("data/test-4x4.png"))?;
    let encrypted_image = client.encrypt_image(&image);

    client.send_message(Message::Image(encrypted_image))?;
    for scale in [(2, 2), (3, 3), (8, 8)] {
        info!("Rescaling {}x{}...", scale.0, scale.1);
        let answer = client.send_message(Message::Rescale(
            Size {
                width: scale.0,
                height: scale.1,
            },
            InterpolationType::Nearest,
        ))?;
        if let Some(Message::Image(image)) = answer {
            info!("Decrypted: {:?}", client.decrypt_image(&image));
        }
    }

    client.send_message(Message::Shutdown)?;
    join_handle.join().unwrap();

    Ok(())
}

#[allow(unused)]
fn test_addition(client: &Client) -> Result<(), Box<dyn Error>> {
    client.send_message(Message::Ping)?;
    let result = client.send_message(Message::Add(client.encrypt(6), 1))?;
    if let Some(Message::AdditionResult(number)) = result {
        info!("6 + 1 = {}", client.decrypt(&number));
    }

    Ok(())
}
