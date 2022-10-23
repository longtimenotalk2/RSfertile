use super::map_find::Pos;
use super::tile::entity::{Terrian, Placement, Natural, Manmade};
use super::Map;
use crate::constant::*;

// https://fuhao.xiao84.com/86855.html
// https://crates.io/crates/colorful

const RED: &str = "\u{1b}[31m";
const GREEN: &str = "\u{1b}[32m";
const UGREEN: &str = "\u{1b}[4;32m";
const YELLOW: &str = "\u{1b}[33m";
const UYELLOW: &str = "\u{1b}[4;33m";
const BLUE: &str = "\u{1b}[34m";
const RESET: &str = "\u{1b}[m";

impl Map {
    pub fn show_tile(&self, pos : &Pos, king_pos : &Pos) {
        let tile = self.tile(&pos);
        match tile.get_terrian() {
            Terrian::Sea => print!("{}Sea{}", BLUE, RESET),
            _ => {
                // Left
                match tile.get_terrian() {
                    Terrian::Plain => print!(" "),
                    Terrian::Sea => (),
                    Terrian::Hill => print!("H"),
                }
                // Middle
                if pos.eq(king_pos) {
                    match tile.get_process() {
                        Some(process) => print!("{}", process),
                        None => print!("@"),
                    }
                }else{
                    match tile.get_process() {
                        Some(process) => print!("{}{}{}", RED, process, RESET),
                        None => print!(" "),
                    }
                }
                //Right
                match tile.get_placement() {
                    Placement::Void => print!(" "),
                    Placement::Landform(n) => {
                        match n {
                            Natural::Tree => {
                                if tile.get_supply() {
                                    print!("{}T{}", GREEN, RESET)
                                } else {
                                    print!("{}T{}", UGREEN, RESET)
                                }
                            },
                            Natural::Farm => {
                                if tile.get_supply() {
                                    print!("{}f{}", YELLOW, RESET)
                                } else {
                                    print!("{}f{}", UYELLOW, RESET)
                                }
                            },
                        }
                    },
                    Placement::Building(m) => {
                        match m {
                            Manmade::Hovel => print!("h"),
                        }
                    },
                    Placement::Foundation(m, process) => {
                        match m {
                            Manmade::Hovel => print!("{}h{}", RED, RESET)
                        }
                    },
                }
            }
        }
    }
    
    pub fn show_adv(&self, king_pos: &Pos) {
        // first line
        print!("┌───");
        for _col in 0..self.n_col - 1 {
            print!("┬───");
        }
        print!("┐\n");
        // others line
        for row in 0..self.n_row {
            //// 1line
            // leftest block
            if row % 2 == 1 {
                print!("  ");
            }
            print!("│");
            // other block
            for col in 0..self.n_col {
                let pos = Pos::new(row, col);
                // TILE BLOCK
                self.show_tile(&pos, king_pos);
                print!("│");
            }
            print!("\n");
            //// 2line
            if row < self.n_row - 1 {
                if row % 2 == 0 {
                    print!("└");
                    for _col in 0..self.n_col {
                        print!("─┬─┴");
                    }
                    print!("─┐\n");
                } else {
                    print!("┌");
                    for _col in 0..self.n_col {
                        print!("─┴─┬");
                    }
                    print!("─┘\n");
                }
            }
        }
        // last line
        if self.n_row % 2 == 0 {
            print!("  ");
        }
        print!("└───");
        for _col in 0..self.n_col - 1 {
            print!("┴───");
        }
        print!("┘");
        print!("\n");
    }
}
