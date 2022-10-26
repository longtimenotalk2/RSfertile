use super::map_find::Pos;
use super::Map;
use crate::error::CtrlErr;

impl Map {
    pub fn get_power(&self, pos: &Pos) -> i64 {
        if self.tile(pos).is_hovel() {
            let mut power: i64 = 1;
            for t in self.adj_tiles(pos) {
                if t.can_produce_food() {
                    power += 1
                }
            }
            power
        } else {
            0
        }
    }

    pub fn sow_from_pos(&mut self, pos: &Pos) -> Result<bool, CtrlErr> {
        self.tile(pos).can_give_sow()?;
        for p in self.find_adjs(pos) {
            match self.tile(&p).can_sow() {
                Err(s) => (),
                Ok(()) => return self.tile_mut(&p).sow().map(|a| true),
            }
        }
        Ok(false)
    }

    pub fn get_all_power(&self) -> i64 {
        let mut power: i64 = 0;
        for pos in &self.hovels_pos {
            power += self.get_power(pos);
        }
        power
    }

    pub fn sow_all(&mut self) -> Result<i64, CtrlErr> {
        let mut count: i64 = 0;
        for pos in self.hovels_pos.clone() {
            let b = self.sow_from_pos(&pos)?;
            if b {
                count += 1;
            }
        }
        Ok(count)
    }
}