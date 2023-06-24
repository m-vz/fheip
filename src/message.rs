use serde::{Deserialize, Serialize};
use tfhe::shortint::CiphertextBig;

use crate::image::EncryptedImage;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Ping,
    Pong,
    Shutdown,
    Add(CiphertextBig, u8),
    AdditionResult(CiphertextBig),
    Image(EncryptedImage),
}

impl Message {
    pub(crate) fn expect_answer(&self) -> bool {
        match self {
            Message::Ping | Message::Add(_, _) => true,
            Message::Pong | Message::Shutdown | Message::AdditionResult(_) | Message::Image(_) => {
                false
            }
        }
    }
}
