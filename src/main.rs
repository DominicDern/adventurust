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
    let jarn = Character::from_file("src/example_characters/jarn_baker.toml");
    let mut queue = InitiativeQueue::new_pre_rolled(vec![(&aaron, 1)]);
    queue.add(&jarn, 4);
    queue.add(&lyra, 3);
    println!("{:?}", queue.get_queue());
    queue.next_turn();
    println!("");
    println!("{:?}", queue.get_queue());
}
