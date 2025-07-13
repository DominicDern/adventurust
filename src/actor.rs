use crate::character::ID;
use crate::condition::Condition;
use crate::health::Health;

pub trait Actor {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn get_id(self) -> ID;
    fn get_health(&self) -> (u16, u16, Option<(u16, u16)>);
    fn set_health(&mut self, health: Health);
    fn get_ac(&self) -> u16;
    fn set_ac(&mut self, ac: u16);
    fn get_resistances(&self) -> Option<Vec<String>>;
    fn set_resistances(&mut self, resistances: Option<Vec<String>>);
    fn add_resistance(&mut self, resistance: String);
    fn get_conditions(&self) -> Option<Vec<Condition>>;
    fn set_conditions(&mut self, conditions: Option<Vec<Condition>>);
    fn add_condition(&mut self, condition: Condition);
}
