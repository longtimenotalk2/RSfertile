use super::Game;
use super::board::map::map_find::Dir;
use super::board::tile::entity::Building;
use std::io;


impl Game {
    fn parse_cmd(&mut self, cmd : &str) {
        match cmd.trim() {
            "s" => self.cmd_move(&Dir::R),
            "x" => self.cmd_move(&Dir::DR),
            "z" => self.cmd_move(&Dir::DL),
            "a" => self.cmd_move(&Dir::L),
            "q" => self.cmd_move(&Dir::UL),
            "w" => self.cmd_move(&Dir::UR),
            "f" => self.cmd_found(Building::Hovel),
            "b" => self.cmd_build(),
            "e" => self.cmd_end(),
            "u" => self.undo(),
            "p" => self.cmd_pick(),
            _ => self.cmd_invalid(),
        }
    }

    pub fn main_loop(&mut self) {
        loop{
            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).expect("fail to read line");
            self.parse_cmd(&cmd);
            if self.board().get_all_power() >= 20 {
                println!("Victory! you got 20+ power")
            }
        }
    }
}