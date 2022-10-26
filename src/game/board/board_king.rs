use super::map::map_find::{Dir, Pos};
use super::map::tile::entity::{Manmade, Resource};
use super::map::tile::Tile;
use super::Board;
use crate::constant::*;
use crate::error::CtrlErr;

#[derive(Clone)]
pub(super) struct King {
    pos: Pos,
    food: i64,
    wood: i64,
}

impl King {
    pub(super) fn new(pos: Pos) -> Self {
        Self {
            pos,
            food: 5,
            wood: 5,
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

    fn use_food(&mut self) -> Result<(), &'static str> {
        if self.food > 0 {
            self.food -= 1;
            Ok(())
        } else {
            Err("No food in the inventory")
        }
    }

    fn use_wood(&mut self) -> Result<(), &'static str> {
        if self.wood > 0 {
            self.wood -= 1;
            Ok(())
        } else {
            Err("No food in the inventory")
        }
    }

    fn set_pos(&mut self, pos: &Pos) {
        self.pos = pos.clone();
    }
}

impl Board {
    pub fn king_can_move(&self, dir: &Dir) -> Result<(), CtrlErr> {
        self.map.can_move(self.king.get_pos(), dir)
    }

    pub fn king_move(&mut self, dir: &Dir) -> Result<(), CtrlErr> {
        let mvcost = self.map.mvcost_dir(self.king.get_pos(), dir)?;
        // move
        let p = self.map.find_dir(self.king.get_pos(), dir).unwrap();
        self.king.set_pos(&p);
        //cpcost
        self.pass_cp((mvcost * C_F) as i64);
        Ok(())
    }

    pub fn king_can_pick(&self) -> Result<Resource, CtrlErr> {
        self.map.can_pick(&self.king.get_pos())
    }

    pub fn king_pick(&mut self) -> Result<(), CtrlErr> {
        let r = self.map.pick(&self.king.get_pos())?;
        self.pass_cp(C_I);
        match r {
            Resource::Food => {
                self.king.food += 1;
            }
            Resource::Wood => {
                self.king.wood += 1;
            }
        }
        Ok(())
    }

    pub fn king_can_found(&self) -> Result<(), CtrlErr> {
        self.map.can_found(self.king.get_pos())
    }

    pub fn king_found(&mut self, manmade: Manmade) -> Result<(), CtrlErr> {
        self.map.found(&self.king.get_pos(), manmade)?;
        self.pass_cp(C_I);
        Ok(())
    }

    pub fn king_end(&mut self) {
        let cpr = MAX_CP - self.cp;
        self.pass_cp(cpr);
    }

    pub fn king_can_build(&self) -> Result<(), CtrlErr> {
        self.map.can_build(&self.king.get_pos())?;
        if self.king.get_wood() > 0 {
            if self.king.get_food() > 0 {
                Ok(())
            }else{
                Err(CtrlErr::LackResource(Resource::Food))
            }
        } else {
            Err(CtrlErr::LackResource(Resource::Wood))
        }
    }

    pub fn king_build(&mut self) -> Result<bool, CtrlErr> {
        self.king_can_build()?;
        self.king.use_food().unwrap();
        self.king.use_wood().unwrap();
        self.pass_cp(C_I);
        self.map.build(&self.king.get_pos())
    }

    pub fn king_can_saw(&self) -> Result<(), CtrlErr> {
        self.map.can_saw(self.king.get_pos())?;
        if self.king.get_food() > 0 {
            Ok(())
        }else{
            Err(CtrlErr::LackResource(Resource::Food))
        }
        
    }

    pub fn king_saw(&mut self) -> Result<(), CtrlErr> {
        self.king_can_saw()?;
        self.map.saw(self.king.get_pos());
        self.king.use_food().unwrap();
        self.king.wood += 1;
        Ok(())
    }
    
}
