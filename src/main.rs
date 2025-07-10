mod actor;
mod character;
mod condition;
mod health;
mod initiative_queue;

use character::Character;
use initiative_queue::InitiativeQueue;

fn main() {
    let aaron = Character::from_file("src/example_characters/aaron_nightbringer.toml");
    let lyra = Character::from_file("src/example_characters/lyra_shadebottom.toml");
    let mut queue = InitiativeQueue::new_pre_rolled(vec![(&aaron, 1)]);
    queue.add(&lyra, 2);
    println!("{:?}", queue.get_queue());
    queue.next_turn();
    println!("{:?}", queue.get_queue())
}
