use super::Game;
use super::board::map::map_find::{Pos, Dir};

impl Game {
    pub fn test(&mut self) {
        self.show();
        self.main_loop();
   }
}