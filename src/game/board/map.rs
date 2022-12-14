pub mod map_action;
pub mod map_basic;
pub mod map_find;
pub mod map_hovel;
pub mod map_new;
pub mod map_show;
pub mod tile;

use super::map::map_find::Pos;
use super::map::tile::Tile;

#[derive(Clone)]
pub struct Map {
    n_row: i64,
    n_col: i64,
    tiles: Vec<Tile>,
    hovels_pos: Vec<Pos>,
}
