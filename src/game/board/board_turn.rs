use super::Board;
use super::map::Map;
use crate::constant::*;

impl Board {
    fn start_turn(&mut self) {
        self.turn += 1;
        self.map.refresh_all();
    }
    
    fn end_turn(&mut self) {
        if self.turn % 4 == 0 {
            self.map.sow_all();
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

    pub fn get_all_power(&self) -> i64 {
        self.map.get_all_power()
    }
}








