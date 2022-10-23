use super::map::map_find::Pos;
use super::map::tile::entity::{Building, Landform, Terrian};
use super::Board;
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

impl Board {
    pub fn show_adv(&self) {
        self.map.show_adv(self.king.get_pos());
        // Turn and power
        println!(
            "Turn : {} ({}/{}), Power Sum : {}",
            self.turn,
            self.cp as f64 / COEFFICIENT as f64,
            MAX_CP / COEFFICIENT,
            self.get_all_power()
        );
        // Inventory
        println!(
            "Inventory : food = {}, wood = {}",
            self.king.get_food(),
            self.king.get_wood()
        )
    }
}
