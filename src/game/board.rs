mod board_king;
mod board_new;
mod board_show;
mod board_turn;
mod board_power;
mod board_plan;
pub mod map;

use board_king::King;
use map::map_find::Pos;
use map::tile::Tile;
use map::Map;
use board_plan::Program;
use board_power::ManPower;

#[derive(Clone)]
pub struct Board {
    map: Map,
    turn: i64,
    cp: i64,
    king: King,
    program : Program,
    manpower : ManPower,
}
