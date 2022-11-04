use super::Map;
use super::map_find::Pos;
use super::map_scale::Scale;

impl Map {
    fn mask_scale_with_food(&self, scale : &mut Scale) {
        todo!()
    }
    
    pub fn search_food(&self, target : &Pos) -> Option<(f64, Map)> {
        let mut map = self.clone();
        let scale = map.check_insert_scale(target);
        
        
        todo!()
    }
}