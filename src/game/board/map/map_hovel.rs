use super::Map;
use super::super::tile::entity::Building;
use super::map_find::Pos;

impl Map {
    pub fn get_power(&self, pos : &Pos) -> i64 {
        match self.tile(pos).get_building() {
            Building::Hovel => {
                let mut power : i64 =  self.to_tiles(&self.find_adjs(pos)).iter().filter(|t| match t.get_building() {
                    Building::Farm => true,
                    _ => false
                }).count().try_into().unwrap();
                power + 1
            }
            _ => panic!("This pos is not a hovel")
        }
    }

    pub fn sow_from_pos(&mut self, pos : &Pos) -> bool{
        for p in self.find_adjs(pos) {
            if self.tile(&p).can_sow() {
                self.tile_mut(&p).sow();
                return true;
            }
        }
        false   
    }
}