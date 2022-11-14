use super::Map;
use super::map_find::{Pos, Dir};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Scale {
    n_row : i64,
    n_col : i64,
    target : Pos,
    mvcost_map : Vec<Option<f64>>,
}

impl Scale {
    pub fn new(target : &Pos, map : &Map) -> Self {
        let mut this = Scale {
            n_row : map.get_n_row(),
            n_col : map.get_n_col(),
            target : target.clone(),
            mvcost_map : vec!(None; (map.get_n_row() * map.get_n_col()).try_into().unwrap()),
        };

        this.set(target, 0.);
        let mut consider_pos = vec![target.clone()];
        while consider_pos.len() > 0 {
            let mut next_pos : Vec<Pos>= vec![];
            for pos in &consider_pos {
                let mvcost_start = this.get(pos).unwrap();
                for (p, mvcost) in  map.hash_target_dir2mvcost(pos).iter() {
                    let mvcost_sum = mvcost_start + *mvcost;
                    if let Some(mcnow) = this.get(&p) {
                        if mvcost_sum < mcnow {
                            this.set(&p, mvcost_sum);
                            next_pos.push(p.clone())
                        }
                    }else{
                        this.set(&p, mvcost_sum);
                        next_pos.push(p.clone())
                    }
                }
            }
            consider_pos = next_pos;
        }
        this
    }

    pub fn get(&self, pos : &Pos) -> Option<f64> {
        self.mvcost_map[pos.into_usize(self.n_col)]
    }

    fn set(&mut self, pos : &Pos, mvcost : f64) {
        self.mvcost_map[pos.into_usize(self.n_col)] = Some(mvcost);
    }
}

impl Map {
    fn hash_target_dir2mvcost(&self, target : &Pos) -> HashMap<Pos, f64> {
        let mut hm = HashMap::new();
        for dir in [Dir::R, Dir::DR, Dir::DL, Dir::L, Dir::UL, Dir::UR] {
            if let Some(pos) = self.find_dir(target, &dir) {
                if let Ok(mvcost) = self.mvcost_dir(&pos, &dir.anti()) {
                    hm.insert(pos, mvcost);
                }
            }
        }
        hm
    }

    pub fn get_scale(&self, target : &Pos) -> Scale {
        Scale::new(target, self)
    }
    
}