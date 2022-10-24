use super::Map;
use super::map_find::Pos;

impl Map {
    pub fn get_power(&self, pos : &Pos) -> i64 {
        if self.tile(pos).is_hovel(){
            let mut power : i64 = 0;
            for t in self.adj_tiles(pos) {
                if t.can_produce_food() {
                    power += 1
                }
            }
            power
        }else{
            0
        }
    }

    pub fn sow_from_pos(&mut self, pos : &Pos) -> Result<bool, &str> {
        if self.tile(pos).is_hovel() {
            for p in self.find_adjs(pos) {
                match self.tile(&p).can_sow() {
                    Err(s) => (),
                    Ok(()) => {
                    return self.tile_mut(&p).sow().map(|a| true)
                    },
                }
            }
            Ok(false)
        }else{
            Err("can not sow from a tile without hovel")
        }
    }

    pub fn get_all_power(&self) -> i64 {
        let mut power : i64 = 0;
        for pos in &self.hovels_pos {
            power += self.get_power(pos);
        }
        power
    }

    pub fn sow_all(&mut self) -> Result<i64, &str> {
        let mut count : i64 = 0;
        for pos in self.hovels_pos.clone() {
            match self.sow_from_pos(&pos) {
                Err(s) => return Err(s),
                Ok(b) => {
                    if b {
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }
}