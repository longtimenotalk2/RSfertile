use super::board::map::map_find::{Dir, Pos};
use super::Game;

impl Game {
    pub fn test(&mut self) {
        self.show();
        // self.board_mut().show_scale_test();
        
        self.main_loop();
    }
}
