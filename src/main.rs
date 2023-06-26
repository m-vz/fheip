use std::error::Error;
use std::path::Path;

use clap::Parser;
use log::info;

use crate::arguments::{Arguments, Command, LoadCommand};
use crate::client::Client;
use crate::crypt::key::load_or_generate_keys;
use crate::image::rescaling::InterpolationType;
use crate::image::{EncryptedImage, Image, Size};
use crate::message::Message;
use crate::server::Server;

mod arguments;
mod client;
mod crypt;
mod exploration;
mod image;
mod message;
mod server;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let arguments = Arguments::parse();
    let address = arguments.address;

    match arguments.command {
        Command::Server => {
            let (_, server_key) = load_or_generate_keys(
                Path::new("data/keys/client"),
                Path::new("data/keys/server"),
            )?;
            Server::new(server_key).start(address.as_str())?;
        }
        command => {
            let (client_key, _) = load_or_generate_keys(
                Path::new("data/keys/client"),
                Path::new("data/keys/server"),
            )?;
            let client = Client::new(address.as_str(), client_key);

            match command {
                Command::Ping => {
                    let answer = client.send_message(Message::Ping)?;
                    if let Some(Message::Pong) = answer {
                        info!("Server reached");
                    }
                }
                Command::Shutdown => {
                    client.send_message(Message::Shutdown)?;
                }
                Command::Load(LoadCommand { file }) => {
                    let image = Image::load(file.as_path())?;

                    client.send_message(Message::Image(client.encrypt_image(&image)))?;
                }
                Command::Rescale(rescale_command) => {
                    let interpolation_type = if rescale_command.bilinear {
                        InterpolationType::Bilinear
                    } else {
                        InterpolationType::Nearest
                    };
                    info!(
                        "Rescaling {}x{}...",
                        rescale_command.width, rescale_command.height
                    );

                    let answer = client.send_message(Message::Rescale(
                        Size {
                            width: rescale_command.width,
                            height: rescale_command.height,
                        },
                        interpolation_type,
                    ))?;
                    if let Some(Message::Image(image)) = answer {
                        decrypt_and_save(
                            &client,
                            &image,
                            format!(
                                "data/output/rescaled-{:?}-{}x{}.png",
                                interpolation_type, rescale_command.width, rescale_command.height
                            )
                            .as_str(),
                        )?;
                    }
                }
                Command::Invert => {
                    let answer = client.send_message(Message::Invert)?;
                    if let Some(Message::Image(image)) = answer {
                        decrypt_and_save(&client, &image, "data/output/inverted.png")?;
                    }
                }
                Command::Grayscale => {
                    let answer = client.send_message(Message::Grayscale)?;
                    if let Some(Message::Image(image)) = answer {
                        decrypt_and_save(&client, &image, "data/output/grayscale.png")?;
                    }
                }
                Command::Server => unreachable!(),
            }
        }
    }

    Ok(())
}

fn decrypt_and_save(
    client: &Client,
    image: &EncryptedImage,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let decrypted_image = client.decrypt_image(image);
    info!("Decrypted: {:?}", decrypted_image);

    decrypted_image.save(Path::new(path))?;

    Ok(())
}
