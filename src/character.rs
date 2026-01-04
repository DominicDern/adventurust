use std::hash::{Hash, Hasher};

use crate::{actor::Actor, condition::Condition, health::Health};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ID {
    id: u64,
}

impl ID {
    fn get_id(character_name: &String) -> Self {
        use std::hash::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        character_name.hash(&mut hasher);
        Self {
            id: hasher.finish(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Character {
    pub name: String,
    pub id: ID,
    health: Health,
    ac: u16,
    speed: Vec<(String, u16)>,
    stats: Vec<(String, u16, u16)>,
    immunities: Vec<String>,
    senses: Vec<String>,
    // passive_perception: u16,
    // actions:
    // traits: Vec<(String, String)>,
    // languages: Vec<String>,
    // resistances: Option<Vec<String>>,
    // conditions: Option<Vec<Condition>>,
}

impl Character {
    pub fn from_file(file_path: &str) -> Self {
        use serde::Deserialize;
        use std::io::Read;

        #[derive(Deserialize, Debug)]
        struct Info {
            health: (u16, u16, u16, u16, u16, u16, bool),
            ac: u16,
            stats: Vec<(String, u16, u16)>,
            speed: Vec<(String, u16)>,
            senses: Vec<String>,
            immunities: Vec<String>,
        }

        #[derive(Deserialize)]
        struct CharacterTOML {
            name: String,
            info: Info,
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
        let name = character_toml.name;
        let character_health: Health = Health {
            primary_hp: character_toml.info.health.0,
            primary_max_hp: character_toml.info.health.1,
            secondary_hp: character_toml.info.health.2,
            secondary_max_hp: character_toml.info.health.3,
            temp_hp: character_toml.info.health.4,
            temp_max_hp: character_toml.info.health.5,
            wild_shape: character_toml.info.health.6,
        };
        let mut character_stats: Vec<(String, u16, u16)> = Vec::new();
        for stat in character_toml.info.stats {
            character_stats.push(stat);
        }

        use std::hash::DefaultHasher;
        #[derive(Hash)]
        struct TempCharacter {
            name: String,
            health: Health,
            ac: u16,
            stats: Vec<(String, u16, u16)>,
            speed: Vec<(String, u16)>,
            senses: Vec<String>,
            resistances: Option<Vec<String>>,
            conditions: Option<Vec<Condition>>,
        }

        let mut hasher = DefaultHasher::new();
        let temp_character = TempCharacter {
            name: name.clone(),
            health: character_health,
            ac: character_toml.info.ac,
            stats: character_stats.clone(),
            speed: character_toml.info.speed.clone(),
            senses: character_toml.info.senses.clone(),
            // TODO change to inconstant values
            resistances: None,
            conditions: None,
        };
        temp_character.hash(&mut hasher);
        let id = ID::get_id(&name);
        Character {
            name,
            id,
            health: character_health,
            ac: character_toml.info.ac,
            stats: character_stats,
            speed: character_toml.info.speed,
            senses: character_toml.info.senses,
            immunities: character_toml.info.immunities,
            // TODO change to inconstant values
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
        let character_stats = vec![
            ("Strength".to_string(), 10, 0),
            ("Inteligence".to_string(), 10, 0),
            ("Dexterity".to_string(), 10, 0),
            ("Wisdom".to_string(), 10, 0),
            ("Constitution".to_string(), 10, 0),
            ("Charisma".to_string(), 10, 0),
        ];
        let speed = vec![("Ground".to_string(), 10)];
        Character {
            name: "default character".to_string(),
            id,
            health: default_character_health,
            ac: 1,
            speed,
            stats: character_stats,
            senses: Vec::new(),
            immunities: Vec::new(),
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

    fn get_stats(&self) -> Vec<(String, u16, u16)> {
        self.stats.clone()
    }

    fn set_stats(&mut self, stats: Vec<(String, u16, u16)>) {
        self.stats = stats;
    }

    fn set_stat(&mut self, stat_name: String, value: u16, modifier: u16) {
        for stat in &mut self.stats {
            if stat.0 == stat_name {
                stat.1 = value;
                stat.2 = modifier;
                return;
            }
        }
        self.stats.push((stat_name, value, modifier));
    }
}
