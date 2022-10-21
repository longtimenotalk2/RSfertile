use super::Board;
use super::map::map_find::{Pos, Dir};
use super::tile::Tile;
use super::tile::entity::{Terrian, Landform, Building};
use crate::constant::*;

#[derive(Clone)]
pub(super) struct King {
    pos : Pos,
    food : i64,
    wood : i64,
}

impl King {
    pub(super) fn new(pos : Pos) -> Self {
        Self {
            pos,
            food : 5,
            wood : 5,
        }
       
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }

    pub fn get_food(&self) -> i64 {
        self.food
    }

    pub fn get_wood(&self) -> i64 {
        self.wood
    }

    fn use_food(&mut self) {
        self.food -= 1;
    }

    fn use_wood(&mut self) {
        self.wood -= 1;
    }

    fn set_pos(&mut self, pos : &Pos) {
        self.pos = pos.clone();
    }
}

impl Board {
    pub fn king_tile(&self) -> &Tile {
        self.map.tile(self.king.get_pos())
    }
    
    pub fn king_tile_mut(&mut self) -> &mut Tile {
        self.map.tile_mut(&self.king.get_pos().clone())
    }
    
    pub fn king_can_move(&self, dir : &Dir) -> bool {
       self.can_move(self.king.get_pos(), dir)
    }

    pub fn king_exe_move(&mut self, dir : &Dir) {
        if self.king_can_move(dir) {
            //cpcost
            let mvcost = self.mvcost_dir(self.king.get_pos(), dir);
            let cpcost : i64 = (mvcost*100.) as i64;
            self.pass_cp(cpcost);
            //move 
            let p = self.map.find_dir(self.king.get_pos(), dir).unwrap();
            self.king.set_pos(&p);
        }else{
            panic!("Invalid move")
        }
    }

    pub fn king_can_pick(&self) -> bool {
        self.map.tile(&self.king.get_pos()).can_pick()
    }

    pub fn king_exe_pick(&mut self) {
        if self.king_can_pick() {
            let mut tile = self.king_tile().clone();
            match tile.get_landform() {
                Landform::Tree => self.king.wood += 1,
                _ => match tile.get_building() {
                    Building::Farm => self.king.food += 1,
                    _ => panic!("Neither tree nor farm"),
                },
            }
            self.king_tile_mut().consume();
            self.pass_cp(COEFFICIENT);
        }else{
            panic!("can not pick")
        }
    }

    pub fn king_can_found(&self) -> bool {
        self.map.tile(self.king.get_pos()).can_found()
    }

    pub fn king_exe_found(&mut self, building : Building) {
        self.found(&self.king.get_pos().clone(), building);
        self.pass_cp(COEFFICIENT);
    }

    pub fn king_end(&mut self) {
        let cpr = MAX_CP - self.cp;
        self.pass_cp(cpr);
    }

    pub fn king_can_build(&self) -> bool {
        self.map.tile(self.king.get_pos()).can_build() && self.king.get_food() > 0 && self.king.get_wood() > 0
    }

    pub fn king_exe_build(&mut self) {
        if self.king_can_build(){
            self.build(&self.king.get_pos().clone(), 1);
            self.king.use_food();
            self.king.use_wood();
            self.pass_cp(COEFFICIENT);
        }else{
            panic!("King can not build")
        }
        
    }

}