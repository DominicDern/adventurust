use crate::{
    character_message::CharacterMessage, initiative_queue::InitiativeMessage, window::WindowMessage,
};

pub enum MessageType {
    WindowMessage(WindowMessage),
    InitiativeMessage,
    CharacterMessage,
}

#[derive(Debug, Clone)]
pub enum Message {
    CharacterMessage(CharacterMessage),
    InitiativeMessage(InitiativeMessage),
    WindowMessage(WindowMessage),
}
