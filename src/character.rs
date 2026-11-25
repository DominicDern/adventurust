use std::hash::{Hash, Hasher};

use crate::{actor::Actor, condition::Condition, health::Health};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ID {
    pub id: u64,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Character {
    name: String,
    id: ID,
    ac: u16,
    health: Health,
    speed: u16,
    resistances: Vec<String>,
    conditions: Vec<Condition>,
}

impl Character {
    pub fn from_file(file_path: &str) -> Self {
        use serde::Deserialize;
        use std::io::Read;

        #[derive(Deserialize, Debug)]
        struct Stats {
            health: (u16, u16, u16, u16, u16, u16, bool),
            speed: u16,
            ac: u16,
        }

        #[derive(Deserialize)]
        struct CharacterTOML {
            name: String,
            stats: Stats,
            // TODO add conditions to this
            //resistances: Option<Vec<String>>,
            //conditions: Option<Vec<Condition>>,
        }
        use std::fs::File;
        let mut file = File::open(file_path).expect("unable to open character file");
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("unable to read character file contents");
        let character_toml: CharacterTOML =
            toml::from_str(&file_contents).expect("failed to deserialize character file");
        let health: Health = Health {
            primary_hp: character_toml.stats.health.0,
            primary_max_hp: character_toml.stats.health.1,
            secondary_hp: character_toml.stats.health.2,
            secondary_max_hp: character_toml.stats.health.3,
            temp_hp: character_toml.stats.health.4,
            temp_max_hp: character_toml.stats.health.5,
            wild_shape: character_toml.stats.health.6,
        };
        let speed: u16 = character_toml.stats.speed;
        let name = character_toml.name;
        use std::hash::DefaultHasher;
        #[derive(Hash)]
        struct TempCharacter {
            name: String,
            health: Health,
            speed: u16,
            ac: u16,
            resistances: Vec<String>,
            conditions: Vec<Condition>,
        }

        let mut hasher = DefaultHasher::new();
        let temp_character = TempCharacter {
            name: name.clone(),
            health,
            speed,
            ac: character_toml.stats.ac,
            // TODO change to nonconstant values
            resistances: Vec::new(),
            conditions: Vec::new(),
        };
        temp_character.hash(&mut hasher);
        let id = ID {
            id: hasher.finish(),
        };

        Character {
            name,
            id,
            health,
            speed,
            ac: character_toml.stats.ac,
            // TODO change to nonconstant values
            resistances: Vec::new(),
            conditions: Vec::new(),
        }
    }
    pub fn new(
        name: String,
        health: Health,
        speed: u16,
        ac: u16,
        resistances: Vec<String>,
        conditions: Vec<Condition>,
    ) -> Self {
        use std::hash::DefaultHasher;
        #[derive(Hash)]
        struct TempCharacter {
            name: String,
            health: Health,
            speed: u16,
            ac: u16,
            resistances: Vec<String>,
            conditions: Vec<Condition>,
        }

        let mut hasher = DefaultHasher::new();
        let temp_character = TempCharacter {
            name: name.clone(),
            health,
            speed,
            ac,
            // TODO change to non-constant values
            resistances: resistances.clone(),
            conditions: conditions.clone(),
        };
        temp_character.hash(&mut hasher);
        let id = ID {
            id: hasher.finish(),
        };

        Character {
            name,
            id,
            health,
            speed,
            ac,
            resistances,
            conditions,
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        use rand::{Rng, rng};
        let default_character_health = Health::default();
        let mut rng = rng();
        let id = ID {
            id: rng.random::<u64>(),
        };
        Character {
            name: "default character".to_string(),
            id,
            health: default_character_health,
            speed: 0,
            ac: 0,
            resistances: Vec::new(),
            conditions: Vec::new(),
        }
    }
}

impl Actor for Character {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_id(self) -> ID {
        self.id
    }

    fn get_health(&self) -> (u16, u16, Option<(u16, u16)>) {
        self.health.get_health()
    }

    fn set_health(&mut self, health: Health) {
        self.health = health;
    }

    fn get_ac(&self) -> u16 {
        self.ac
    }

    fn set_ac(&mut self, ac: u16) {
        self.ac = ac;
    }

    fn get_resistances(&self) -> Vec<String> {
        self.resistances.clone()
    }

    fn set_resistances(&mut self, resistances: Vec<String>) {
        self.resistances = resistances;
    }

    fn add_resistance(&mut self, resistance: String) {
        self.resistances.push(resistance);
    }

    fn get_conditions(&self) -> Vec<Condition> {
        self.conditions.clone()
    }

    fn set_conditions(&mut self, conditions: Vec<Condition>) {
        self.conditions = conditions;
    }

    fn add_condition(&mut self, condition: Condition) {
        self.conditions.push(condition);
    }
}
