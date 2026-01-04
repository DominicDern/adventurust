use std::u16;

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
    fn get_stats(&self) -> Vec<(String, u16, u16)>;
    fn set_stats(&mut self, stats: Vec<(String, u16, u16)>);
    fn set_stat(&mut self, stat_name: String, value: u16, modifier: u16);
}
