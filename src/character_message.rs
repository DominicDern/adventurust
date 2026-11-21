use crate::character::ID;
use crate::health::Health;

#[derive(Debug, Clone)]
pub enum CharacterMessage {
    SetHealth(ID, Health),
}
