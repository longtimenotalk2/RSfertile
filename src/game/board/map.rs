pub mod map_find;
pub mod map_basic;
pub mod map_show;
pub mod map_hovel;
pub mod map_action;

use super::tile::Tile;

#[derive(Clone)]
pub struct Map {
    n_row: i64,
    n_col: i64,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(n_row : i64, n_col : i64) -> Self {
         Self {
            n_row,
            n_col,
            tiles: vec![Tile::new(); (n_row * n_col).try_into().unwrap()],
        }
    }
}
