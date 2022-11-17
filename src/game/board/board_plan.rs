use super::map::map_find::Pos;
use super::Board;
use crate::error::CtrlErr;

#[derive(Clone)]
pub struct Program {
    plans: Vec<Pos>,
}

impl Program {
    pub fn new() -> Self {
        Self { plans: vec![] }
    }

    fn insert(&mut self, pos: &Pos) {
        self.plans.push(pos.clone())
    }

    fn delete(&mut self, pos: &Pos) {
        let index = self.plans.iter().position(|x| x == pos).unwrap();
        self.plans.remove(index);
    }

    fn get_plans(&self) -> &Vec<Pos> {
        &self.plans
    }

    fn first_plan(&self) -> Option<&Pos> {
        self.plans.get(0)
    }
}

impl Board {
    pub(super) fn new_plan(&mut self, pos: &Pos) {
        self.program.insert(pos);
    }

    pub(super) fn finish_plan(&mut self, pos: &Pos) {
        self.program.delete(pos);
    }

    pub fn get_plans(&self) -> &Vec<Pos> {
        self.program.get_plans()
    }

    fn try_plan(&self, pos : &Pos) -> Result<i64, CtrlErr> {
        if let Some((mvcost, map_new)) = self.map.search_build(pos) {
            let pwcost = mvcost as i64 + 2;
            self.manpower.enough(pwcost).map(|i| pwcost)
        }else{
            Err(CtrlErr::NoTarget)
        }
    }

    fn do_plan(&mut self, pos : &Pos) -> Result<bool, CtrlErr> {
        let pwcost = self.try_plan(pos)?;
        let (mvcost, map_new) = self.map.search_build(pos).unwrap();
        self.manpower.employ(pwcost).unwrap();
        self.map = map_new;
        let result = self.map.build(&self.king.get_pos());
        if let Ok(true) = result {
            self.finish_plan(&self.king.get_pos().clone());
        }
        result
    }

    pub(super) fn plan_main(&mut self) {
        
    }
}
