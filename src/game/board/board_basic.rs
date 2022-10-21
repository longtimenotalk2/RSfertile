use super::Board;
use super::tile::Tile;
use super::tile::entity::{Building, Terrian, Landform};
use super::board_find::{Pos, Dir};


impl Board {
    pub fn tile(&self, pos : &Pos) -> &Tile {
        &self.tiles[pos.into_usize(self.n_col)]
    }

    pub(super) fn tile_mut(&mut self, pos : &Pos) -> &mut Tile {
        &mut self.tiles[pos.into_usize(self.n_col)]
    }

    pub fn to_tiles (&self, positions : &[Pos]) -> Vec<&Tile> {
        positions.iter().map(|p| self.tile(&p)).collect()
    }
        
    pub(super) fn set_terrian(&mut self, pos : &Pos, terrian: Terrian) {
        self.tile_mut(pos).set_terrian(terrian);
    }

    pub(super) fn set_landform(&mut self, pos : &Pos, landform : Landform) {
        self.tile_mut(pos).set_landform(landform);
    }

    pub fn can_move(&self, pos : &Pos, dir : &Dir) -> bool {
        match self.find_dir(pos, dir) {
            Some(p) => self.tile(&p).can_step(),
            None => false,
        }
    }

    pub fn mvcost_dir(&self, pos : &Pos, dir : &Dir) -> f64{
        if self.can_move(pos, dir) {
            let p = self.find_dir(pos, dir).unwrap();
            self.tile(&p).mvcost()
        }else{
            panic!("target can not move, cant cal mvcost")
        }
    }

    pub(super) fn refresh_all(&mut self) {
        for pos in self.find_all(){
            self.tile_mut(&pos).refresh();
        }
    }

    pub fn found(&mut self, pos : &Pos, building : Building) {
        self.tile_mut(pos).found(building);
    }

    pub fn build(&mut self, pos: &Pos, mount : i64) {
        let is_finish = self.tile_mut(pos).build(mount);
        if is_finish {
            match self.tile(pos).get_building() {
                Building::Hovel => self.add_hovel_to_list(pos),
                _ => (),
            }
        }
    }
    
}