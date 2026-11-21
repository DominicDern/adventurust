use crate::character::ID;
use crate::health::Health;
use crate::window_message::WindowMessage;

pub enum MessageType {
    WindowMessage(WindowMessage),
    InitiativeMessage,
    CharacterMessage,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetHealth(ID, Health),
    WindowMessage(WindowMessage),
}
