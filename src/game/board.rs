mod board_king;
mod board_new;
mod board_show;
mod board_turn;
pub mod map;

use board_king::King;
use map::map_find::Pos;
use map::tile::Tile;
use map::Map;

#[derive(Clone)]
pub struct Board {
    map: Map,
    turn: i64,
    cp: i64,
    king: King,
}
