mod actor;
mod character;
mod condition;
mod health;
mod initiative_queue;

use character::Character;

fn main() {
    let aaron = Character::from_file("src/example_characters/aaron_nightbringer.toml");
    println!("{:?}", aaron);
}
