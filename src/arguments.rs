use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    /// A command to run
    #[clap(subcommand)]
    pub command: Command,
    /// The server address
    #[arg(short, long, default_value = "127.0.0.1:34347")]
    pub address: String,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start a server
    Server,
    /// Ping the server
    Ping,
    /// Tell the server to shut down
    Shutdown,
    /// Send an image to the server
    Load(LoadCommand),
    /// Rescale the image stored on the server
    Rescale(RescaleCommand),
    /// Invert the image stored on the server
    Invert,
    /// Turn the image stored on the server into grayscale
    Grayscale,
}

#[derive(Debug, Args)]
pub struct LoadCommand {
    /// The path to the image
    pub file: PathBuf,
}

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("interpolation").required(true).args(&["bilinear", "nearest"])))]
pub struct RescaleCommand {
    /// Use bilinear interpolation
    #[arg(long)]
    pub bilinear: bool,
    // Use nearest-neighbour interpolation
    #[arg(long)]
    pub nearest: bool,
    /// The new width of the image
    pub width: u16,
    /// The new height of the image
    pub height: u16,
}
