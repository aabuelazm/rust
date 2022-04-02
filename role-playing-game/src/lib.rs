pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        match self.level {
            lvl if lvl >= 10 => Some(Player {
                health: 100,
                mana: Some(100),
                level: lvl,
            }),
            lvl => Some(Player {
                health: 100,
                mana: None,
                level: lvl,
            }),
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if self.health < mana_cost {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
                0
            }
            Some(sp) if sp < mana_cost => 0,
            Some(sp) => {
                self.mana = Some(sp - mana_cost);
                mana_cost * 2
            }
        }
    }
}
