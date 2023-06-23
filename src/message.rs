use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Ping,
    Pong,
    Shutdown,
}

impl Message {
    pub(crate) fn expect_answer(&self) -> bool {
        match self {
            Message::Ping => true,
            Message::Pong | Message::Shutdown => false,
        }
    }
}
