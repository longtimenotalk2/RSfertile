use super::Game;
use super::board::map::map_find::Dir;
use super::board::tile::entity::Building;

const RED : &str = "\u{1b}[31m";
const RESET : &str = "\u{1b}[m";

fn refuse(txt: &str) {
    println!("{}Refuse : {}{}", RED, txt, RESET);
}

impl Game {
    fn update(&mut self) {
        let board_new = self.board().clone();
        self.boards.push(board_new);
    }
    
    pub(super) fn undo(&mut self) {
        if self.boards.len() > 1{
            self.boards.pop();
            self.show();
        }else{
            refuse("initial state, can not undo")
        }
    }

    pub(super) fn cmd_move(&mut self, dir : &Dir){
        if self.board().king_can_move(dir) {
            self.update();
            self.board_mut().king_exe_move(dir);
            self.show();
        }else{
            refuse("can not move in this direction.")
        }
    }

    pub(super) fn cmd_pick(&mut self) {
        if self.board().king_can_pick() {
            self.update();
            self.board_mut().king_exe_pick();
            self.show();
        }else{
            refuse("no resource or comsumed this turn")
        }
    }

    pub(super) fn cmd_found(&mut self, building: Building) {
        if self.board().king_can_found() {
            self.update();
            self.board_mut().king_exe_found(building);
            self.show();
        }else{
            refuse("can nou found in this tile");
        }
    }

    pub(super) fn cmd_build(&mut self) {
        if self.board().king_can_build() {
            self.update();
            self.board_mut().king_exe_build();
            self.show();
        }else{
            refuse("can not build in this tile")
        }
    }

    pub(super) fn cmd_end(&mut self) {
        self.update();
        self.board_mut().king_end();
        self.show();
    }
    
    pub(super) fn cmd_invalid(&self) {
        refuse("invalid input")
    }

}