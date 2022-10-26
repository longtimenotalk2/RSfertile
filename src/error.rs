use game::board::map::tile::entity::{Terrian, Placement, Resource};


const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[m";

fn refuse(txt: &str) {
    println!("{}Refuse : {}{}", RED, txt, RESET);
}

enum CtrlErr {
    WrongTerrian(Terrian),
    WrongPlacement(Placement),
    LackResource(Resource),
}



impl CtrlErr {
    pub fn str(&self, action : &str) -> String {
        match self {
            CtrlErr::WrongTerrian(t) => format!("Can not {} in terrian {}"),
            CtrlErr::WrongPlacement(p) => ,
            CtrlErr::LackResource(r) => ,
        }
    }
}