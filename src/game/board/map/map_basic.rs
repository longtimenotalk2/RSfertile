use super::map_find::{Dir, Pos};
use super::tile::Tile;
use super::Map;

impl Map {
    pub(super) fn tile(&self, pos: &Pos) -> &Tile {
        &self.tiles[pos.into_usize(self.n_col)]
    }

    pub(super) fn tile_mut(&mut self, pos: &Pos) -> &mut Tile {
        &mut self.tiles[pos.into_usize(self.n_col)]
    }

    pub(super) fn to_tiles(&self, positions: &[Pos]) -> Vec<&Tile> {
        positions.iter().map(|p| self.tile(&p)).collect()
    }

    pub(super) fn all_tiles_with_pos(&self) -> Vec<(Pos, &Tile)> {
        let mut result = vec!();
        for r in 0..self.n_row {
            for c in 0..self.n_col {
                let pos = Pos::new(r, c);
                let tile = self.tile(&pos);
                result.push((pos, tile));
            }
        }
        result
    }

    pub(super) fn adj_tiles(&self, pos: &Pos) -> Vec<&Tile> {
        self.to_tiles(&self.find_adjs(pos))
    }

    pub fn get_n_row(&self) -> i64 {
        self.n_row
    }

    pub fn get_n_col(&self) -> i64 {
        self.n_col
    }
}
