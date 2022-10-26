use crate::game::board::map::tile::entity::{Terrian, Placement, Resource};


const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[m";

fn refuse(txt: &str) {
    println!("{}Refuse : {}{}", RED, txt, RESET);
}

#[derive(Debug)]
pub enum CtrlErr {
    WrongTerrian(Terrian),
    WrongPlacement(Placement),
    LackResource(Resource),
    Consumed,
    OOBoundary,
    Undo,
    Input,
}



impl CtrlErr {
    pub fn str(&self, action : &str) -> String {
        match self {
            CtrlErr::WrongTerrian(t) => format!("Can not {} in terrian {}", action, t.str()),
            CtrlErr::WrongPlacement(p) => format!("Can not {} in {}", action, p.str()),
            CtrlErr::LackResource(r) => format!("Can not {} with no {}", action, r.str()),
            CtrlErr::Consumed => format!("Can not {}, corresponding tile is consumed", action),
            CtrlErr::OOBoundary => format!("Can not {} to target, out of boundary", action),
            CtrlErr::Undo => format!("Can not undo, initial state"),
            CtrlErr::Input => format!("Invalid input"),
        }
    }
}