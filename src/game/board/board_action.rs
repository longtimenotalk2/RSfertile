use super::Board;
use super::tile::entity::{Terrian, Landform, Building};
use super::map::map_find::{Pos, Dir};

impl Board {
    pub fn set_terrian(&mut self, pos : &Pos, terrian: Terrian) {
        self.map.tile_mut(pos).set_terrian(terrian);
    }
    
    pub(super) fn set_landform(&mut self, pos : &Pos, landform : Landform) {
        self.map.tile_mut(pos).set_landform(landform);
    }

    pub fn can_move(&self, pos : &Pos, dir : &Dir) -> bool {
        match self.map.find_dir(pos, dir) {
            Some(p) => self.map.tile(&p).can_step(),
            None => false,
        }
    }

    pub fn mvcost_dir(&self, pos : &Pos, dir : &Dir) -> f64{
        if self.can_move(pos, dir) {
            let p = self.map.find_dir(pos, dir).unwrap();
            self.map.tile(&p).mvcost()
        }else{
            panic!("target can not move, cant cal mvcost")
        }
    }

    pub(super) fn refresh_all(&mut self) {
        for pos in self.map.find_all(){
            self.map.tile_mut(&pos).refresh();
        }
    }

    pub fn found(&mut self, pos : &Pos, building : Building) {
        self.map.tile_mut(pos).found(building);
    }

    pub fn build(&mut self, pos: &Pos, mount : i64) {
        let is_finish = self.map.tile_mut(pos).build(mount);
        if is_finish {
            match self.map.tile(pos).get_building() {
                Building::Hovel => self.add_hovel_to_list(pos),
                _ => (),
            }
        }
    }
}