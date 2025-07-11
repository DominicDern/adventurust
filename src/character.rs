use iced::advanced::image::Image;
use std::{io::Read, path::Path};

use crate::{actor::Actor, condition::Condition, health::Health};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Character {
    name: String,
    art: String,
    health: Health,
    ac: u16,
    resistances: Option<Vec<String>>,
    conditions: Option<Vec<Condition>>,
}

impl Character {
    pub fn from_file(file_path: &str) -> Self {
        use serde::Deserialize;

        #[derive(Deserialize, Debug)]
        struct Stats {
            health: (u16, u16, u16, u16, u16, u16, bool),
            ac: u16,
        }

        #[derive(Deserialize)]
        struct CharacterTOML {
            name: String,
            art: String,
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
        let character_health: Health = Health {
            primary_hp: character_toml.stats.health.0,
            primary_max_hp: character_toml.stats.health.1,
            secondary_hp: character_toml.stats.health.2,
            secondary_max_hp: character_toml.stats.health.3,
            temp_hp: character_toml.stats.health.4,
            temp_max_hp: character_toml.stats.health.5,
            wild_shape: character_toml.stats.health.6,
        };
        Character {
            name: character_toml.name,
            art: character_toml.art,
            health: character_health,
            ac: character_toml.stats.ac,
            // TODO change to nonconstant values
            resistances: None,
            conditions: None,
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        let default_character_health = Health::default();
        Character {
            name: "default character".to_string(),
            art: String::new(),
            health: default_character_health,
            ac: 1,
            resistances: None,
            conditions: None,
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

    fn get_health(&self) -> (u16, u16, Option<(u16, u16)>) {
        self.health.get_health()
    }

    fn set_health(&mut self, health: Health) {
        self.health = health;
    }

    fn get_ac(&self) -> u16 {
        self.ac.clone()
    }

    fn set_ac(&mut self, ac: u16) {
        self.ac = ac;
    }

    fn get_resistances(&self) -> Option<Vec<String>> {
        self.resistances.clone()
    }

    fn set_resistances(&mut self, resistances: Option<Vec<String>>) {
        self.resistances = resistances;
    }

    fn add_resistance(&mut self, resistance: String) {
        match self.resistances.as_mut() {
            Some(resistances) => {
                resistances.push(resistance);
            }
            None => {
                self.resistances = Some(vec![resistance]);
            }
        }
    }

    fn get_conditions(&self) -> Option<Vec<Condition>> {
        self.conditions.clone()
    }

    fn set_conditions(&mut self, conditions: Option<Vec<Condition>>) {
        self.conditions = conditions;
    }

    fn add_condition(&mut self, condition: Condition) {
        match self.conditions.as_mut() {
            Some(conditions) => {
                conditions.push(condition);
            }
            None => {
                self.conditions = Some(vec![condition]);
            }
        }
    }
}
