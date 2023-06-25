use std::error::Error;
use std::path::Path;
use std::thread;

use log::info;

use crate::client::Client;
use crate::encryption::load_or_generate_keys;
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

    let (client_key, server_key) =
        load_or_generate_keys(Path::new("data/keys/client"), Path::new("data/keys/server"))?;
    let join_handle = thread::spawn(|| server::Server::new(server_key).start(ADDRESS).unwrap());
    let client = Client::new(ADDRESS, client_key);
    let image = Image::load(Path::new("data/images/charmander-21x18.png"))?;
    let encrypted_image = client.encrypt_image(&image);

    client.send_message(Message::Image(encrypted_image))?;
    for scale in [(10, 9), (30, 18)] {
        info!("Rescaling {}x{}...", scale.0, scale.1);
        let answer = client.send_message(Message::Rescale(
            Size {
                width: scale.0,
                height: scale.1,
            },
            InterpolationType::Nearest,
        ))?;
        if let Some(Message::Image(image)) = answer {
            let decrypted_image = client.decrypt_image(&image);
            info!("Decrypted: {:?}", decrypted_image);
            decrypted_image.save(Path::new(
                format!(
                    "data/output/charmander-rescaled-{}x{}.png",
                    scale.0, scale.1
                )
                .as_str(),
            ))?;
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
