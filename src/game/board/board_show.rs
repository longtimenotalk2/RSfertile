use super::{Board};
use super::tile::entity::{Terrian, Landform, Building};
use crate::constant::*;
use super::board_find::Pos;

// https://fuhao.xiao84.com/86855.html
// https://crates.io/crates/colorful 

const RED : &str = "\u{1b}[31m";
const GREEN : &str = "\u{1b}[32m";
const UGREEN : &str = "\u{1b}[4;32m";
const YELLOW : &str = "\u{1b}[33m";
const UYELLOW : &str = "\u{1b}[4;33m";
const BLUE : &str = "\u{1b}[34m";
const RESET : &str = "\u{1b}[m";

impl Board {
    pub fn show_adv(&self) {
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
                let tile = self.tile(&pos);
// Left: show terrian
match tile.get_terrian() {
    Terrian::Plain => print!(" "),
    Terrian::Sea => print!("{}Sea{}", BLUE, RESET),
    Terrian::Hill => print!("H"),
};
match tile.get_terrian() {
    Terrian::Sea => (),
    _ => {
    // Middle: King
                        
    if pos.eq(self.king.get_pos()) {
        match tile.get_building() {
            Building::Foundation(f) => print!("{}", f.1),
            _ => print!("@"),
        }
    }else{
        match tile.get_building() {
            Building::Foundation(f) => print!("{}{}{}", RED, f.1, RESET),
            _ => print!(" "),
        }
    }
                
    // Right: show building
    match tile.get_building() {
        Building::Void => {
            match tile.get_landform(){
                Landform::Tree => if tile.get_supply() {
                    print!("{}T{}", GREEN, RESET)
                }else{
                    print!("{}T{}", UGREEN, RESET)
                },
                _ => print!(" "),
            }
        },
        Building::Farm => if tile.get_supply() {
            print!("{}f{}", YELLOW, RESET)
        }else{
            print!("{}f{}", UYELLOW, RESET)
        },
        Building::Hovel => print!("h"),
        Building::Foundation(f) => {
            match f.0 {
Building::Foundation(_) => panic!("Foundation can not contain foundation"),
Building::Hovel => print!("{}h{}", RED, RESET), 
_ => print!(" "),
}
        }
    };
                    }
                }
                
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

        // hovels
        for (i, pos) in self.hovels_pos.iter().enumerate() {
            match self.tile(&pos).get_building() {
                Building::Hovel => (
                    println!("Hovel_{} ({}, {}) : power = {}", i, pos.get().0, pos.get().1, self.get_power(pos))
                ),
                _ => panic!("Hovel position mismatch!"),
            }
        }
        // Turn and power
        println!("Turn : {} ({}/{}), Power Sum : {}", self.turn, self.cp as f64 / COEFFICIENT as f64, MAX_CP/COEFFICIENT, self.get_all_power());
        // Inventory
        println!("Inventory : food = {}, wood = {}", self.king.get_food(), self.king.get_wood())
        // Test
        
        //
    }
}