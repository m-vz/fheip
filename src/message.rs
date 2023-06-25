use serde::{Deserialize, Serialize};

use crate::image::rescaling::InterpolationType;
use crate::image::{EncryptedImage, Size};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    /// Check if the server is alive.
    Ping,
    /// The result of a ping.
    Pong,
    /// Shut down the server.
    Shutdown,
    /// Send an image on the server to do operations on.
    Image(EncryptedImage),
    /// Rescale the stored image to a new size using the given interpolation type.
    Rescale(Size, InterpolationType),
    /// Invert the stored image.
    Invert,
    /// There is no image stored on the server.
    NoImage,
}

impl Message {
    pub(crate) fn expect_answer(&self) -> bool {
        match self {
            Message::Ping | Message::Rescale(_, _) | Message::Invert => true,
            Message::Pong | Message::Shutdown | Message::Image(_) | Message::NoImage => false,
        }
    }
}
