use super::map_find::{Dir, Pos};
use super::tile::Tile;
use super::Map;

impl Map {
    fn tile(&self, pos: &Pos) -> &Tile {
        &self.tiles[pos.into_usize(self.n_col)]
    }

    fn tile_mut(&mut self, pos: &Pos) -> &mut Tile {
        &mut self.tiles[pos.into_usize(self.n_col)]
    }

    fn to_tiles(&self, positions: &[Pos]) -> Vec<&Tile> {
        positions.iter().map(|p| self.tile(&p)).collect()
    }

    fn adj_tiles(&self, pos: &Pos) -> Vec<&Tile> {
        self.to_tiles(&self.find_adjs(pos))
    }
}
