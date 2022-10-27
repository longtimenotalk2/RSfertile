use super::board::map::map_find::Dir;
use super::board::map::tile::entity::Manmade;
use super::Game;
use crate::error::CtrlErr;

const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[m";

fn refuse(txt: &str) {
    println!("{}Refuse : {}{}", RED, txt, RESET);
}

fn refuse_err(err: &CtrlErr, act : &str) {
    refuse(&err.str(act));
}

impl Game {
    fn update(&mut self) {
        let board_new = self.board().clone();
        self.boards.push(board_new);
    }

    pub(super) fn cmd_undo(&mut self) {
        if self.boards.len() > 1 {
            self.boards.pop();
            self.show();
        } else {
            refuse("Initial state, can not undo")
        }
    }

    pub(super) fn cmd_move(&mut self, dir: &Dir) {
        if let Err(e) = self.board().king_can_move(dir) {
            refuse_err(&e, "move");
            return;
        }
        self.update();
        self.board_mut().king_move(dir).expect("panic in king move");
        self.show();
    }

    pub(super) fn cmd_pick(&mut self) {
        if let Err(e) = self.board().king_can_pick() {
            refuse_err(&e, "pick");
            return;
        }
        self.update();
        self.board_mut().king_pick().expect("panic in king puck");
        self.show();
    }

    pub(super) fn cmd_found(&mut self, manmade: Manmade) {
        if let Err(e) = self.board().king_can_found(){
            refuse_err(&e, "found");
            return;
        }
        self.update();
        self.board_mut()
            .king_found(manmade)
            .expect("panic in king found");
        self.show();
    }

    pub(super) fn cmd_build(&mut self)  {
        if let Err(e) = self.board().king_can_build(){
            refuse_err(&e, "build");
            return;
        }
        self.update();
        self.board_mut().king_build().expect("panic in king build");
        self.show();
    }

    pub(super) fn cmd_build_to_finish(&mut self) {
        if let Err(e) = self.board().king_can_build(){
            refuse_err(&e, "build");
            return;
        }
        self.update();
        self.board_mut().king_build().expect("panic in king build");
        while let Ok(_) = self.board().king_can_build(){
            self.board_mut().king_build().expect("panic in king build");
        }
        self.show();
    }
    
    pub(super) fn cmd_saw(&mut self) {
        if let Err(e) = self.board().king_can_saw(){
            refuse_err(&e, "saw");
            return;
        }
        self.update();
        self.board_mut().king_saw().expect("panic in king saw");
        self.show();
    }

    pub(super) fn cmd_work(&mut self) {
        if let Err(CtrlErr::WrongPlacement(_)) = self.board().king_can_pick() {
            if let Err(CtrlErr::WrongPlacement(_)) = self.board().king_can_saw() {
                if let Err(CtrlErr::WrongPlacement(p)) = self.board().king_can_build() {
                    refuse_err(&CtrlErr::WrongPlacement(p.clone()), "pick/saw/build");
                }else{
                    self.cmd_build();
                }
            }else{
                self.cmd_saw();
            }
        }else{
            self.cmd_pick();
        }
    }

    pub(super) fn cmd_end(&mut self) {
        self.update();
        self.board_mut().king_end();
        self.show();
    }

    pub(super) fn cmd_invalid(&self) {
        refuse("Invalid input");
    }
}
