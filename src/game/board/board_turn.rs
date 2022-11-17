use super::map::Map;
use super::Board;
use crate::constant::*;

impl Board {
    fn start_turn(&mut self) {
        self.turn += 1;
        self.manpower.inject(self.get_all_power());
        self.map.refresh_all();
    }

    fn end_turn(&mut self) {
        self.plan_main();
        if self.turn % 4 == 0 {
            self.map.sow_all();
        }
    }

    pub fn pass_cp(&mut self, cpc: i64) {
        self.cp += cpc;
        while self.cp >= MAX_CP {
            self.cp -= MAX_CP;
            self.end_turn();
            self.start_turn();
        }
    }

    pub fn get_all_power(&self) -> i64 {
        self.map.get_all_power()
    }
}
