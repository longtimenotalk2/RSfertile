//#![allow(unused)]

pub mod board;
mod game_command;
mod game_in;
mod game_test;

use board::Board;

#[derive(Clone)]
pub struct Game {
    boards: Vec<Board>,
}

impl Game {
    fn board_mut(&mut self) -> &mut Board {
        self.boards.last_mut().unwrap()
    }

    fn board(&self) -> &Board {
        self.boards.last().unwrap()
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            boards: vec![Board::new_std()],
        }
    }
    pub fn show(&self) {
        self.board().show_adv();
    }
}
