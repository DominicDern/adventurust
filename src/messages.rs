use crate::{character_message::CharacterMessage, window_message::WindowMessage};

pub enum MessageType {
    WindowMessage(WindowMessage),
    InitiativeMessage,
    CharacterMessage,
}

#[derive(Debug, Clone)]
pub enum Message {
    CharacterMessage(CharacterMessage),
    WindowMessage(WindowMessage),
}
