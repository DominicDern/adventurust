fn main() {
    println!("Hello dipshit");
}

#[derive(Debug)]
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

trait Actor {
    fn get_name(&self) -> String;
    fn get_health(&self) -> (u16, u16, Option<(u16, u16)>);
    fn get_ac(&self) -> u16;
    fn get_resistances(&self) -> Vec<String>;
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
struct IniativeQueue {
    queue: Vec<Character>,
    position: u16,
}

impl IniativeQueue {
    fn new(actors: Vec<Character>) -> Self {
        // TODO add rolling inititive to queue
        Self {
            queue: actors,
            position: 0,
        }
    }
}

impl IniativeQueue {
    // TODO add rolling inititive
    fn add(&mut self, actor: Character) {
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
