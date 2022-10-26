use crate::game::board::map::tile::entity::{Terrian, Placement, Resource};


const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[m";

fn refuse(txt: &str) {
    println!("{}Refuse : {}{}", RED, txt, RESET);
}

pub enum CtrlErr {
    WrongTerrian(Terrian),
    WrongPlacement(Placement),
    LackResource(Resource),
}



impl CtrlErr {
    pub fn str(&self, action : &str) -> String {
        match self {
            CtrlErr::WrongTerrian(t) => format!("Can not {} in terrian {}", action, t.str()),
            CtrlErr::WrongPlacement(p) => format!("Can not {} in {}", action, p.str()),
            CtrlErr::LackResource(r) => format!("Can not {} with no {}", action, r.str()),
        }
    }
}