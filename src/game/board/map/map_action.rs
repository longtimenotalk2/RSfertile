use super::map_find::{Dir, Pos};
use super::Map;
use super::tile::entity::{Manmade, Resource};

impl Map {
    pub fn can_move(&self, pos: &Pos, dir: &Dir) -> Result<(), &str> {
        match self.find_dir(pos, dir) {
            None => Err("Destination out of the map"),
            Some(p) => self.tile(&p).can_step(),
        }
    }

    pub fn mvcost_dir(&self, pos: &Pos, dir: &Dir) -> Result<f64, &str> {
        match self.can_move(pos, dir) {
            Err(s) => Err(s),
            Ok(_) => self.tile(&self.find_dir(pos, dir).unwrap()).mvcost(),
        }
    }

    pub(super) fn refresh_all(&mut self) {
        for pos in self.find_all() {
            self.tile_mut(&pos).refresh();
        }
    }

    pub fn can_found(&self, pos : &Pos) -> Result<(), &str> {
        self.tile(pos).can_found()
    }

    pub fn found(&mut self, pos: &Pos, manmade: Manmade) -> Result<(), &str> {
        self.tile_mut(pos).found(manmade)
    }

    pub fn can_build(&self, pos : &Pos) -> Result<(), &str> {
        self.tile(pos).can_build()
    }

    pub fn build(&mut self, pos: &Pos) -> Result<bool, &str> {
        let result = self.tile_mut(pos).build();
        match result {
            Err(s) => Err(s),
            Ok(is_finish) => {
                if is_finish && self.tile(pos).is_hovel() {
                    self.hovels_pos.push(pos.clone())
                }
                Ok(is_finish)
            }
        }
    }

    pub fn can_pick(&self, pos : &Pos) -> Result<(), &str> {
        self.tile(pos).can_pick()
    }

    pub fn pick(&self, pos : &Pos) -> Result<Resource, &str> {
        self.tile_mut(pos).pick()
    }
}
