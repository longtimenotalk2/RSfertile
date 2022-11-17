use super::Map;
use super::map_find::Pos;
use super::map_scale::Scale;
use super::tile::entity::Resource;
use std::collections::HashMap;

fn find_min(hm : &HashMap<Pos, f64>) -> Option<(Pos, f64)> {
    let mut vmin = -1.0;
    let mut rpos = Pos::new(0, 0);
    let mut c = 0;
    for (pos, v) in hm.iter() {
        if vmin == -1.0 || *v < vmin {
            c += 1;
            vmin = *v;
            rpos = pos.clone();
        }
    }
    if c == 0 {
        None
    }else{
        Some((rpos, vmin))
    }
    
}

impl Map {
    fn food_pos_list(&self) -> Vec<Pos> {
        let mut list = vec!();
        for (pos, tile) in self.all_tiles_with_pos() {
            if let Ok(Resource::Food) = tile.can_pick() {
                list.push(pos);
            }
        }
        list
    }

    fn saw_pos_list(&self) -> Vec<Pos> {
        let mut list = vec!();
        for (pos, tile) in self.all_tiles_with_pos() {
            if let Ok(()) = tile.can_saw() {
                list.push(pos);
            }
        }
        list
    }
    
    fn search_food(&self, target : &Pos) -> Option<(f64, Map)> {
        let scale = self.get_scale(target);
        let mut hm_pos_mv = HashMap::new();
        for pos in self.food_pos_list() .iter() {
            if let Some(mv) = scale.get(pos) {
                hm_pos_mv.insert(pos.clone(), mv);
            }
                
        }

        if let Some((pos, mvcost)) = find_min(&hm_pos_mv) {
            let mut map_new = self.clone();
            map_new.pick(&pos).unwrap();
            Some((mvcost, map_new))
        }else{
            None
        }
    }

    fn search_wood(&self, target : &Pos) -> Option<(f64, Map)> {
        let scale = self.get_scale(target);
        let mut hm = HashMap::new();
        for pos in self.saw_pos_list() .iter() {
            if let Some(mv_w) = scale.get(pos) {
                if let Some((mv_f, map_f)) = self.search_food(pos) {
                    hm.insert(pos.clone(), mv_w + mv_f);
                }
            }
        }

        if let Some((pos, mvcost)) = find_min(&hm) {
            let (_, mut map_new) = self.search_food(&pos).unwrap();
            map_new.saw(&pos).unwrap();
            Some((mvcost, map_new))
        }else{
            None
        }
    }

    pub fn search_build(&self, target : &Pos) -> Option<(f64, Map)> {
        if let Some((mv1, map1)) = self.search_wood(target) {
            if let Some((mv2, map2)) = map1.search_food(target) {
                Some((mv1+mv2, map2))
            }else{
                None
            }
        }else{
            None
        }
    }

}