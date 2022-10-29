use super::map::map_find::Pos;
use super::Board;

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
        dbg!(&pos);
        dbg!(&self.plans);
        let index = self.plans.iter().position(|x| x == pos).unwrap();
        self.plans.remove(index);
    }

    fn get_plans(&self) -> &Vec<Pos> {
        &self.plans
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
}
