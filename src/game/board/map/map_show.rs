use super::map_find::Pos;
use super::tile::entity::{Manmade, Natural, Placement, Terrian};
use super::Map;
use super::map_scale::Scale;
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

pub enum ShowStyle<'a> {
    Default(&'a Pos),
    Distance(&'a Pos),
    MVCost(&'a Scale),
}

impl Map {
    fn print_default(&self, pos: &Pos, king_pos: &Pos) {
        let tile = self.tile(pos);
        if pos.eq(king_pos) {
            match tile.get_process() {
                Some(process) => print!("{}", process),
                None => print!("@"),
            }
        } else {
            match tile.get_process() {
                Some(process) => print!("{}{}{}", RED, process, RESET),
                None => print!(" "),
            }
        }
    }

    fn print_distance(&self, pos: &Pos, target_pos: &Pos) {
        let d = self.distance(pos, target_pos);
        print!("{}", d);
    }

    fn print_mvcost(&self, pos : &Pos, scale : &Scale) {
        match scale.get(pos) {
            Some(mc) => {
                if mc > 9. {
                    print!("+");
                }else{
                    print!("{}", mc);
                }
            },
            None => print!("{}{}{}", RED,"x", RESET)
        }
    }
    
    fn show_middle(&self, pos: &Pos, style: &ShowStyle) {
        match style {
            ShowStyle::Default(kp) => self.print_default(pos, kp),
            ShowStyle::Distance(tp) => self.print_distance(pos, tp),
            ShowStyle::MVCost(scale) => self.print_mvcost(pos, scale),
        }
    }
    
    fn show_tile(&self, pos: &Pos, style: &ShowStyle) {
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
                self.show_middle(pos, &style);
                //Right
                match tile.get_placement() {
                    Placement::Void => print!(" "),
                    Placement::Landform(n) => match n {
                        Natural::Tree => {
                            if tile.get_supply() {
                                print!("{}T{}", GREEN, RESET)
                            } else {
                                print!("{}T{}", UGREEN, RESET)
                            }
                        }
                        Natural::Farm => {
                            if tile.get_supply() {
                                print!("{}f{}", YELLOW, RESET)
                            } else {
                                print!("{}f{}", UYELLOW, RESET)
                            }
                        }
                    },
                    Placement::Building(m) => match m {
                        Manmade::Hovel => print!("h"),
                        Manmade::Sawmill => print!("s"),
                        _ => print!(" "),
                    },
                    Placement::Foundation(m, process) => match m {
                        Manmade::Hovel => print!("{}h{}", RED, RESET),
                        Manmade::Sawmill => print!("{}s{}", RED, RESET),
                        _ => print!(" "),
                    },
                }
            }
        }
    }

    fn show_frame(&self, style: &ShowStyle) {
        // col num
        for c in 0..self.n_col {
            print!("   {}", c);
        }
        print!("\n");
        
        // first line
        print!(" "); // row num
        print!("┌───");
        for _col in 0..self.n_col - 1 {
            print!("┬───");
        }
        print!("┐\n");
        // others line
        for row in 0..self.n_row {
            //// 1line
            print!("{}", row); // row num
            // leftest block
            if row % 2 == 1 {
                print!("  ");
            }
            print!("│");
            // other block
            for col in 0..self.n_col {
                let pos = Pos::new(row, col);
                // TILE BLOCK
                self.show_tile(&pos, style);
                print!("│");
            }
            print!("\n");
            //// 2line
            print!(" "); // row num
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

    fn show_hovels(&self) {
        for (i, pos) in self.hovels_pos.iter().enumerate() {
            println!(
                "Hovel_{} ({}, {}) : power = {}",
                i,
                pos.get().0,
                pos.get().1,
                self.get_power(pos)
            );
        }
    }

    pub fn show_adv(&self, king_pos: &Pos) {
        self.show_frame(&ShowStyle::Default(king_pos));
        self.show_hovels();
    }

    pub fn show_distance(&self, target_pos: &Pos) {
        self.show_frame(&ShowStyle:: Distance(target_pos));
    }

    pub fn show_scale(&self, scale : &Scale) {
        self.show_frame(&ShowStyle::MVCost(scale))
    }
}
