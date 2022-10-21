pub mod tile;
mod board_basic;
pub mod board_find;
mod board_show;
mod board_new;
mod board_turn;
mod board_hovel;
mod board_king;

use tile::Tile;
use board_king::King;
use board_find::Pos;

#[derive(Clone)]
pub struct Board {
    n_row: i64,
    n_col: i64,
    tiles: Vec<Tile>,
    hovels_pos : Vec<Pos>,
    turn: i64,
    cp: i64,
    king : King,
}



