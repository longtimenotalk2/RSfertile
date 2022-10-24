pub mod tile;
pub mod map_find;
pub mod map_basic;
pub mod map_show;
pub mod map_hovel;
pub mod map_new;
pub mod map_action;

use super::map::tile::Tile;
use super::map::map_find::Pos;

#[derive(Clone)]
pub struct Map {
    n_row: i64,
    n_col: i64,
    tiles: Vec<Tile>,
    hovels_pos : Vec<Pos>,
}


