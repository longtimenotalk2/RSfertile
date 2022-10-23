use super::Board;
use super::map::Map;

use super::map::tile::Tile;
use super::board_king::King;
use super::map::tile::entity::{Terrian, Natural};
use super::map::map_find::Pos;

impl Board {
    fn new(n_row: i64, n_col: i64) -> Self {
        Self {
            map : Map::new(n_row, n_col),
            turn : 1,
            cp : 0,
            king : King::new(Pos::new(1, 1))
        }
    }

    pub fn new_std() -> Self {
        Self {
            map : Map::new_std(),
            turn : 1,
            cp : 0,
            king : King::new(Pos::new(1, 1))
        }
    }
}
