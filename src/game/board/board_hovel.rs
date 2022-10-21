use super::Board;
use super::tile::entity::{Building};
use super::map::map_find::Pos;

impl Board {
    pub(super) fn add_hovel_to_list(&mut self, pos : &Pos){
    self.hovels_pos.push(pos.clone());
    }
    
    pub fn get_power(&self, pos : &Pos) -> i64 {
        match self.map.tile(pos).get_building() {
            Building::Hovel => {
                let mut power : i64 =  self.map.to_tiles(&self.map.find_adjs(pos)).iter().filter(|t| match t.get_building() {
                    Building::Farm => true,
                    _ => false
                }).count().try_into().unwrap();
                power + 1
            }
            _ => panic!("This pos is not a hovel")
        }
   }

    pub fn get_all_power(&self) -> i64 {
        let mut power : i64 = 0;
        for pos in &self.hovels_pos {
            power += self.get_power(pos);
        }
        power
    }

    fn sow_from_pos(&mut self, pos : &Pos) -> bool{
        for p in self.map.find_adjs(pos) {
            if self.map.tile(&p).can_sow() {
                self.map.tile_mut(&p).sow();
                return true;
            }
        }
        false   
    }

    pub(super) fn sow_all(&mut self) {
        for p in self.hovels_pos.clone() {
            self.sow_from_pos(&p);
        }
    }
}