#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Health {
    pub primary_hp: u16,
    pub primary_max_hp: u16,
    pub secondary_hp: u16,
    pub secondary_max_hp: u16,
    pub temp_hp: u16,
    pub temp_max_hp: u16,
    pub wild_shape: bool,
}

impl Health {
    pub fn get_health(&self) -> (u16, u16, Option<(u16, u16)>) {
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
