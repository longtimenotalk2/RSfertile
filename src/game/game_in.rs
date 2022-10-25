use super::board::map::map_find::Dir;
use super::board::map::tile::entity::Manmade;
use super::Game;
use std::io;

const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[m";
fn refuse(txt: &str) {
    println!("{}Refuse : {}{}", RED, txt, RESET);
}

impl Game {
    fn parse_cmd(&mut self, cmd: &str) {
        let result = match cmd.trim() {
            "s" => self.cmd_move(&Dir::R),
            "x" => self.cmd_move(&Dir::DR),
            "z" => self.cmd_move(&Dir::DL),
            "a" => self.cmd_move(&Dir::L),
            "q" => self.cmd_move(&Dir::UL),
            "w" => self.cmd_move(&Dir::UR),
            "fh" => self.cmd_found(Manmade::Hovel),
            "fs" => self.cmd_found(Manmade::Sawmill),
            "b" => self.cmd_build(),
            "e" => self.cmd_end(),
            "u" => self.cmd_undo(),
            "p" => self.cmd_pick().or(self.cmd_saw()),
            _ => self.cmd_invalid(),
        };
        match result {
            Err(s) => refuse(s),
            _ => (),
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).expect("fail to read line");
            self.parse_cmd(&cmd);
            if self.board().get_all_power() >= 20 {
                println!("Victory! you got 20+ power")
            }
        }
    }
}
