use crate::condition::Condition;
use crate::health::Health;

pub trait Actor {
    fn get_name(&self) -> String;
    fn get_health(&self) -> (u16, u16, Option<(u16, u16)>);
    fn get_ac(&self) -> u16;
    fn get_resistances(&self) -> Option<Vec<String>>;
    fn get_conditions(&self) -> Option<Vec<Condition>>;
    fn set_name(&mut self, name: String);
    fn set_health(&mut self, health: Health);
    fn set_ac(&mut self, ac: u16);
    fn set_resistances(&mut self, resistances: Option<Vec<String>>);
    fn set_conditions(&mut self, conditions: Option<Vec<Condition>>);
    fn add_resistance(&mut self, resistance: String);
    fn add_condition(&mut self, condition: Condition);
}
