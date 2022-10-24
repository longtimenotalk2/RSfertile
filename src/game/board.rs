pub mod map;
mod board_show;
mod board_new;
mod board_turn;
mod board_king;

use map::tile::Tile;
use map::Map;
use board_king::King;
use map::map_find::Pos;

#[derive(Clone)]
pub struct Board {
    map : Map,
    turn: i64,
    cp: i64,
    king : King,
}



