use super::map::Map;
use super::Board;

use super::board_king::King;
use super::map::map_find::Pos;
use super::map::tile::entity::{Natural, Terrian};
use super::map::tile::Tile;
use super::board_plan::Program;
use super::board_power::ManPower;

impl Board {

    pub fn new_std() -> Self {
        Self {
            map: Map::new_std(),
            turn: 1,
            cp: 0,
            king: King::new(Pos::new(1, 1)),
            program: Program::new(),
            manpower: ManPower::new(),
        }
    }
}
