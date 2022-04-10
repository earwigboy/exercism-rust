// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mut mana = None;
            if self.level >= 10 {
                mana = Some(100);
            }
            return Some(Player {
                health: 100,
                mana,
                level: self.level,
            });
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let damage = 2 * mana_cost;
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    return 0;
                }
                self.mana = u32::checked_sub(mana, mana_cost);
                damage
            }
            None => {
                match u32::checked_sub(self.health, mana_cost) {
                    Some(health) => self.health = health,
                    None => self.health = 0,
                }
                0
            }
        }
    }
}
