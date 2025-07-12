mod actor;
mod character;
mod condition;
mod health;
mod initiative_queue;

use character::Character;
use health::Health;
use initiative_queue::InitiativeQueue;

struct State<'a> {
    initiative_queue: Option<InitiativeQueue<'a>>,
}

enum Message {
    SetHealth(u64, Health),
}

impl Message {
    fn update(&mut self, message: Message) {
        match message {
            Self::SetHealth(character, health) => {}
        }
    }
}

fn main() {
    let mut current_actor_id: u64;

    let aaron = Character::from_file("src/example_characters/aaron_nightbringer.toml");
    let lyra = Character::from_file("src/example_characters/lyra_shadebottom.toml");
    let jarn = Character::from_file("src/example_characters/jarn_baker.toml");
    let mut queue = InitiativeQueue::new_pre_rolled(vec![(&aaron, 1), (&lyra, 11), (&jarn, 10)]);
    println!("{:?}", queue.get_queue());
    queue.next_turn();
    println!("");
    println!("{:?}", queue.get_queue());
}
