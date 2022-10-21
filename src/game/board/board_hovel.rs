use super::Board;
use super::tile::entity::Building;
use super::map::map_find::Pos;

impl Board {
    pub(super) fn add_hovel_to_list(&mut self, pos : &Pos){
    self.hovels_pos.push(pos.clone());
    }

    pub fn get_all_power(&self) -> i64 {
        let mut power : i64 = 0;
        for pos in &self.hovels_pos {
            power += self.map.get_power(pos);
        }
        power
    }

    pub(super) fn sow_all(&mut self) {
        for p in &self.hovels_pos {
            self.map.sow_from_pos(&p);
        }
    }
}