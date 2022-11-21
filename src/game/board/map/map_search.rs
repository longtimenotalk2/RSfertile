use super::Map;
use super::map_find::Pos;
use super::map_scale::Scale;
use super::tile::entity::Resource;
use std::collections::HashMap;

fn find_min(hm : &HashMap<Pos, f64>) -> Option<(&Pos, f64)> {
    let mut result : Option<(&Pos, f64)> = None;
    for (pos, v) in hm.iter() {
        if result.is_none() || *v < result.unwrap().1 {
            result = Some((pos, *v))
        }
    }
    result
}

#[derive(Clone)]
pub struct TransNote {
    start : Pos,
    end : Pos,
    item : Resource,
    cost : f64,
}

impl TransNote {
    fn new(start : &Pos, end : &Pos, item : &Resource, cost : f64) -> Self {
        Self {
            start : start.clone(),
            end : end.clone(),
            item : item.clone(),
            cost,
        }
    }

    pub fn str(&self) -> String {
        format!("{} : Trans {} from {} to {}", self.cost, self.item.str(), self.start.str(), self.end.str())
    }

    fn get_cost(&self) -> f64 {
        self.cost
    }
}

#[derive(Clone)]
pub struct WorkNote {
    pos : Pos,
    cost : f64,
    action : String,
}

impl WorkNote {
    fn new(pos : &Pos, cost : f64, action : &str) -> Self {
        Self {
            pos : pos.clone(),
            cost,
            action : action.to_string(),
        }
    }

    pub fn str(&self) -> String {
        format!("{} : {} at {}", self.cost, self.action, self.pos.str())
    }
}

pub enum Note {
    Trans(TransNote),
    Work(WorkNote),
}

impl Note {
    pub fn str(&self) -> String {
        match self {
            Note::Trans(n) => n.str(),
            Note::Work(n) => n.str(),
        }
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
            if let Ok(_) = self.can_saw(&pos) {
                list.push(pos);
            }
        }
        list
    }
    
    fn search_food(&self, target : &Pos) -> Option<(f64, Map, Vec<TransNote>)> {
        let scale = self.get_scale(target);
        let mut hm_pos_mv = HashMap::new();
        for pos in self.food_pos_list().iter() {
            if let Some(mv) = scale.get(pos) {
                hm_pos_mv.insert(pos.clone(), mv);
            }
        }

        if let Some((pos, mvcost)) = find_min(&hm_pos_mv) {
            let mut map_new = self.clone();
            map_new.pick(pos).unwrap();
            let transnote = TransNote::new(pos, target, &Resource::Food, mvcost);
            Some((mvcost, map_new, vec![transnote]))
        }else{
            None
        }
    }

    fn search_wood(&self, target : &Pos) -> Option<(f64, Map, Vec<TransNote>)> {
        let scale = self.get_scale(target);
        let mut hm = HashMap::new();
        for pos in self.saw_pos_list().iter() {
            if let Some(mv_w) = scale.get(pos) {
                if let Some((mv_f, map_f, _)) = self.search_food(pos) {
                    hm.insert(pos.clone(), mv_w + mv_f);
                }
            }
        }

        if let Some((pos, mvcost)) = find_min(&hm) {
            let (_, mut map_new, mut trans_note) = self.search_food(pos).unwrap();
            map_new.saw(&pos).unwrap();
            let cost_behind = trans_note[0].get_cost();
            trans_note.push(TransNote::new(pos, target, &Resource::Wood, mvcost-cost_behind));
            Some((mvcost, map_new, trans_note))
        }else{
            None
        }
    }

    fn search_by_order(&self, target : &Pos, item_list : &[Resource]) -> Option<(f64, Map, Vec<TransNote>)> {
        let mut mvcost = 0.;
        let mut map_now : Option<Map> = None;
        let mut trans_note = vec![];
        
        for item in item_list {
            match item {
                Resource::Food => {
                    if let Some((mv, map, list)) = self.search_food(target) {
                        mvcost += mv;
                        map_now = Some(map);
                        trans_note.extend_from_slice(&list);
                    }else{
                        return None;
                    }
                },
                Resource::Wood => {
                    if let Some((mv, map, list)) = self.search_wood(target) {
                        mvcost += mv;
                        map_now = Some(map);
                        trans_note.extend_from_slice(&list);
                    }else{
                        return None;
                    }
                },
                _ => return None,
            }
        }
        if let Some(map) = map_now{
            Some((mvcost, map, trans_note))
        }else{
            None
        }
    }

    pub fn search_build(&self, target : &Pos) -> Option<(f64, Map, Vec<TransNote>)> {
        if let Some((mvcost1, map1, trans_note1)) = self.search_by_order(target, &[Resource::Wood, Resource::Food]) {
            if let Some((mvcost2, map2, trans_note2)) = self.search_by_order(target, &[Resource::Food, Resource::Wood]) {
                if mvcost1 < mvcost2 {
                    Some((mvcost1, map1, trans_note1))
                }else{
                    Some((mvcost2, map2, trans_note2))
                }
            }else{
                Some((mvcost1, map1, trans_note1))
            }
        }else{
            self.search_by_order(target, &[Resource::Food, Resource::Wood])
        }
    }
}