use super::map::map_find::Pos;
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
            self.cp as f64 / C_F,
            MAX_CP / C_I,
            self.map.get_all_power()
        );
        // Inventory
        println!(
            "Inventory : food = {}, wood = {}",
            self.king.get_food(),
            self.king.get_wood()
        )
    }

    pub fn show_distance(&self, target_pos: &Pos) {
        self.map.show_distance(target_pos);
    }

    pub fn show_scale_test(&mut self) {
        let pos = Pos::new(2, 2);
        self.map.insert_scale(&pos);
        let scale = self.map.get_scale(&pos).unwrap();
        self.map.show_scale(scale);
    }
}
