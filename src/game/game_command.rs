use super::Game;
use super::board::map::map_find::Dir;
use super::board::map::tile::entity::Manmade;

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
        match self.board(self).king_can_move(dir) {
            Err(s) => refuse(s),
            Ok(_) => {
                self.update()
                self.board_mut().king_move(dir).expect("panic in king move");
                self.show();
            }
        }
    }

    pub(super) fn cmd_pick(&mut self) {
        match self.board().king_can_pick() {
            Err(s) => refuse(s),
            Ok(_) => {
                self.update();
                self.board_mut().king_pick().expect("panic in king puck");
                self.show();
            }
        }
    }

    pub(super) fn cmd_found(&mut self, manmade: Manmade) {
        match self.board().king_can_found() {
            Err(s) => refuse(s),
            Ok(_) => {
                self.update();
                self.board_mut().king_found(manmade).expect("panic in king found");
                self.show();
            }
        }
    }

    pub(super) fn cmd_build(&mut self) {
        match self.board().king_can_build() {
            Err(s) => refuse(s),
            Ok(_) => {
                self.update();
                self.board_mut().king_build().expect("panic in king build");
                self.show()
            }
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







