use super::Board;
use super::tile::entity::Building;
use crate::constant::*;

impl Board {
    fn start_turn(&mut self) {
        self.turn += 1;
        self.refresh_all();
    }
    
    fn end_turn(&mut self) {
        if self.turn % 4 == 0 {
            self.sow_all()
        }
    }

    pub fn pass_cp (&mut self, cpc:i64) {
        self.cp += cpc;
        while self.cp >= MAX_CP{
            self.cp -= MAX_CP;
            self.end_turn();
            self.start_turn();
        }
    }

    
}