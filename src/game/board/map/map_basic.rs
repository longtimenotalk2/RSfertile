use super::Map;
use super::tile::Tile;
use super::tile::entity::{Building, Terrian, Landform};
use super::map_find::{Pos, Dir};


impl Map {
    pub fn tile(&self, pos : &Pos) -> &Tile {
        &self.tiles[pos.into_usize(self.n_col)]
    }

    pub fn tile_mut(&mut self, pos : &Pos) -> &mut Tile {
        &mut self.tiles[pos.into_usize(self.n_col)]
    }

    pub fn to_tiles (&self, positions : &[Pos]) -> Vec<&Tile> {
        positions.iter().map(|p| self.tile(&p)).collect()
    }
        


}