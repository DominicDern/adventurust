use std::{default, fmt::Debug};

fn main() {
    println!("Hello dipshit");
}

#[derive(Debug, Clone, Copy)]
struct Health {
    primary_hp: u16,
    primary_max_hp: u16,
    secondary_hp: u16,
    secondary_max_hp: u16,
    temp_hp: u16,
    temp_max_hp: u16,
    wild_shape: bool,
}

impl Health {
    fn get_health(&self) -> (u16, u16, Option<(u16, u16)>) {
        let (hp, max_hp, temporary_hp): (u16, u16, Option<(u16, u16)>);
        if self.wild_shape {
            hp = self.secondary_hp;
            max_hp = self.secondary_max_hp;
            if self.temp_hp > 0 {
                temporary_hp = Some((self.temp_hp, self.temp_max_hp));
            } else {
                temporary_hp = None;
            }
        } else {
            (hp, max_hp) = (self.primary_hp, self.primary_max_hp);
            if self.temp_hp > 0 {
                temporary_hp = Some((self.temp_hp, self.temp_max_hp));
            } else {
                temporary_hp = None;
            }
        }
        (hp, max_hp, temporary_hp)
    }
}

impl Default for Health {
    fn default() -> Self {
        Health {
            primary_hp: 1,
            primary_max_hp: 1,
            secondary_hp: 0,
            secondary_max_hp: 0,
            temp_hp: 0,
            temp_max_hp: 0,
            wild_shape: false,
        }
    }
}

trait Actor {
    fn get_name(&self) -> String;
    fn get_health(&self) -> (u16, u16, Option<(u16, u16)>);
    fn get_ac(&self) -> u16;
    fn get_resistances(&self) -> Vec<String>;
}

#[derive(Debug, Clone)]
struct Condition {
    name: String,
    description: String,
    turn: u16,
}

impl Condition {
    fn add_turn(&mut self) {
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

#[derive(Debug, Clone)]
struct Character {
    name: String,
    health: Health,
    ac: u16,
    resistances: Vec<String>,
    statuses: Vec<Condition>,
}

impl Actor for Character {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_health(&self) -> (u16, u16, Option<(u16, u16)>) {
        self.health.get_health()
    }

    fn get_ac(&self) -> u16 {
        self.ac.clone()
    }

    fn get_resistances(&self) -> Vec<String> {
        self.resistances.clone()
    }
}

impl Default for Character {
    fn default() -> Self {
        let default_character_health = Health::default();
        Character {
            name: "default character".to_string(),
            health: default_character_health,
            ac: 1,
            resistances: vec!["none".to_string()],
            statuses: vec![Condition::default()],
        }
    }
}

#[derive(Debug)]
struct InitiativeQueue {
    queue: Vec<Character>,
    position: u16,
}

impl InitiativeQueue {
    /// Creates a inititive queue with pre rolled inititive numbers
    fn new_pre_rolled(mut actors: Vec<(Character, u16)>) -> Self {
        let mut queue: Vec<(Character, u16)> = vec![];
        match actors.pop() {
            Some(character_roll) => queue = vec![character_roll],
            None => {
                return InitiativeQueue {
                    queue: vec![Character::default()],
                    position: u16::MAX,
                };
            }
        };
        for actor in actors.iter_mut() {
            let mut index = 0;
            for added_actor in queue.iter_mut() {
                if actor.1 == added_actor.1 {
                    queue.insert(index + 1, actor.clone());
                } else if actor.1 < added_actor.1 {
                }
            }
            index += 1;
        }
        print!("{:?}", queue);
        todo!()
    }
}

impl InitiativeQueue {
    // TODO add rolling inititive
    fn add(&mut self, actor: Character, initiative: u16) {
        self.queue.push(actor);
        todo!()
    }

    fn next_turn(&mut self) {
        // TODO add status effect turn addition
        if self.position == self.queue.len() as u16 - 1 {
            self.position = 0;
        } else {
            self.position += 1;
        }
    }
}
