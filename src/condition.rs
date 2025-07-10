use serde::Deserialize;
#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct Condition {
    name: String,
    description: String,
    turn: u16,
}

impl Condition {
    pub fn add_turn(&mut self) {
        self.turn += 1;
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition {
            name: "default condition name".to_string(),
            description: "default condition dsecription".to_string(),
            turn: 0,
        }
    }
}
