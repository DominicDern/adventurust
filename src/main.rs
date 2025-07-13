mod actor;
mod character;
mod condition;
mod health;
mod initiative_queue;

use std::collections::HashMap;

use crate::character::ID;
use actor::Actor;
use character::Character;
use health::Health;
use initiative_queue::InitiativeQueue;

struct State {
    initiative_queue: Option<InitiativeQueue>,
    actors: Option<HashMap<ID, Character>>,
    party: Option<Vec<ID>>,
}

enum Message {
    SetHealth(ID, Health),
}

impl Message {
    fn update(&mut self, message: Message) {
        match message {
            Self::SetHealth(character, health) => {}
        }
    }
}

fn main() {
    let mut ids = HashMap::new();

    let aaron = Character::from_file("src/example_characters/aaron_nightbringer.toml");
    let lyra = Character::from_file("src/example_characters/lyra_shadebottom.toml");
    let jarn = Character::from_file("src/example_characters/jarn_baker.toml");
    ids.insert(aaron.clone().get_id(), aaron.clone());
    ids.insert(lyra.clone().get_id(), lyra.clone());
    ids.insert(jarn.clone().get_id(), jarn.clone());
    println!("{:?}", ids);
    println!();
    let queue = InitiativeQueue::new_pre_rolled(vec![
        (aaron.get_id(), 12),
        (lyra.get_id(), 11),
        (jarn.get_id(), 10),
    ]);
    let mut queue = queue.unwrap();
    println!("{:?}", queue.get_queue());
    println!();
    queue.next_turn();
    queue.next_turn();
    queue.next_turn();
    // queue.add(lyra.get_id(), 11);
    println!("{:?}", queue.get_queue());
}
