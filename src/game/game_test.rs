use super::board::map::map_find::{Dir, Pos};
use super::Game;

impl Game {
    pub fn test(&mut self) {
        self.show();
        self.board().show_distance(&Pos::new(3,4));
        self.main_loop();
    }
}
