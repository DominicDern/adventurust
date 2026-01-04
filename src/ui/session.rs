use std::collections::HashMap;

use crate::character::{Character, ID};
use crate::initiative_queue::InitiativeQueue;

#[derive(Default, Clone)]
pub struct SessionState {
    pub queues: HashMap<String, InitiativeQueue>,
    pub current_queue: Option<String>,
    pub actors: Option<HashMap<ID, Character>>,
    pub current_sheet: Option<Character>,
    pub party: Option<Vec<ID>>,
}

impl SessionState {
    pub fn new(party: Option<Vec<ID>>) -> Self {
        Self {
            queues: HashMap::new(),
            current_queue: None,
            actors: None,
            current_sheet: None,
            party,
        }
    }
}

impl SessionState {
    pub fn add_character_to_current_queue(&mut self, character: &ID, roll: &u16, in_battle: &bool) {
        match self.current_queue.clone() {
            Some(current_queue) => match self.queues.get_mut(&current_queue) {
                Some(queue) => {
                    queue.add(character.clone(), roll.clone(), in_battle.clone());
                }
                None => {
                    let default_queue = InitiativeQueue::new_pre_rolled(Vec::new()).unwrap();
                    self.queues
                        .insert("Default InitiativeQueue".to_string(), default_queue);
                }
            },
            None => {
                println!("No current queue");
            }
        }
    }
}
