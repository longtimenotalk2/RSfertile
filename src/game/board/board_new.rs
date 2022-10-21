use super::Board;

use super::tile::Tile;
use super::board_king::King;
use super::tile::entity::{Terrian, Landform, Building};
use super::board_find::Pos;

impl Board {
    fn new(n_row: i64, n_col: i64) -> Self {
        Self {
            n_row,
            n_col,
            tiles: vec![Tile::new(); (n_row * n_col).try_into().unwrap()],
            hovels_pos : vec![],
            turn : 1,
            cp : 0,
            king : King::new(Pos::new(1, 1))
        }
    }

    pub fn new_std() -> Self {
        let mut b = Board::new(4, 6);
        b.set_terrian(&Pos::new(2, 2), Terrian::Sea);
        b.set_terrian(&Pos::new(0, 3), Terrian::Hill);
        b.set_landform(&Pos::new(0, 2), Landform::Tree);
        b.set_landform(&Pos::new(1, 5), Landform::Tree);
        b.set_landform(&Pos::new(2, 5), Landform::Tree);
        b.set_landform(&Pos::new(3, 5), Landform::Tree);
        b.found(&Pos::new(1, 1), Building::Hovel);
        b.build(&Pos::new(1, 1), 5);
       
        b
    }
}
