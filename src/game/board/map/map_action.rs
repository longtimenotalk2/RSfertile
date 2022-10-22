use super::super::tile::entity::{Building, Landform, Terrian};
use super::map_find::{Dir, Pos};
use super::Map;

impl Map {
    pub fn found(&mut self, pos: &Pos, building: Building) {
        self.tile_mut(pos).found(building);
    }

    pub fn build(&mut self, pos: &Pos, mount: i64) -> bool{
        self.tile_mut(pos).build(mount)
    }
}
