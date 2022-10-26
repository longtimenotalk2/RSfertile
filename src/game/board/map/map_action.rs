use super::map_find::{Dir, Pos};
use super::tile::entity::{Manmade, Resource};
use super::Map;
use crate::error::CtrlErr;

impl Map {
    pub fn can_move(&self, pos: &Pos, dir: &Dir) -> Result<(), CtrlErr> {
        match self.find_dir(pos, dir) {
            None => Err(CtrlErr::OOBoundary),
            Some(p) => self.tile(&p).can_step(),
        }
    }

    pub fn mvcost_dir(&self, pos: &Pos, dir: &Dir) -> Result<f64, CtrlErr> {
        self.can_move(pos, dir)?;
        self.tile(&self.find_dir(pos, dir).unwrap()).mvcost()
    }

    pub fn refresh_all(&mut self) {
        for pos in self.find_all() {
            self.tile_mut(&pos).refresh();
        }
    }

    pub fn can_found(&self, pos: &Pos) -> Result<(), CtrlErr> {
        self.tile(pos).can_found()
    }

    pub fn found(&mut self, pos: &Pos, manmade: Manmade) -> Result<(), CtrlErr> {
        self.tile_mut(pos).found(manmade)
    }

    pub fn can_build(&self, pos: &Pos) -> Result<(), CtrlErr> {
        self.tile(pos).can_build()
    }

    pub fn build(&mut self, pos: &Pos) -> Result<bool, CtrlErr> {
        let is_finish = self.tile_mut(pos).build()?;
        if is_finish && self.tile(pos).is_hovel() {
            self.hovels_pos.push(pos.clone())
        }
        Ok(is_finish)
    }

    pub fn can_pick(&self, pos: &Pos) -> Result<Resource, CtrlErr> {
        self.tile(pos).can_pick()
    }

    pub fn pick(&mut self, pos: &Pos) -> Result<Resource, CtrlErr> {
        self.tile_mut(pos).pick()
    }

    pub fn can_saw(&self, pos : &Pos) -> Result<Pos, CtrlErr> {
        self.tile(pos).can_saw()?;
        for p in self.find_adjs(pos) {
            if let Ok(Resource::Wood) = self.tile(&p).can_pick() {
                return Ok(p);
            }
        }
        Err(CtrlErr::Consumed)
    }

    pub fn saw(&mut self, pos : &Pos) -> Result<(), CtrlErr> {
        let p = self.can_saw(pos)?;
        match self.tile_mut(&p).pick() {
            Ok(Resource::Wood) => Ok(()),
            _ => panic!("Do not pick wood in sawing")
        }
    }
    
}
