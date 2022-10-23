pub mod entity;
use entity::{Terrian, Placement, Natural, Manmade};
use crate::constant::*;


#[derive(Clone)]
pub struct Tile {
    terrian: Terrian,
    placement: Placement,
    supply: bool,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            terrian: Terrian::Plain,
            placement: Placement::Void,
            supply: true,
        }
    }

    pub fn get_terrian(&self) -> &Terrian {
        &self.terrian
    }

    pub fn get_placement(&self) -> &Placement {
        &self.placement
    }

    pub fn get_process(&self) -> Option<i64> {
        self.placement.get_process()
    }

    pub fn is_hovel(&self) -> bool {
        self.placement.is_hovel()
    }

    pub fn mvcost(&self) -> f64 {
        self.terrian.mvcost() + self.placement.mvcost()
    }

    pub fn can_step(&self) -> Result<(), &str> {
        self.terrian.can_step()
    }

    pub fn can_found(&self) -> Result<(), &str> {
        match self.terrian.can_found() {
            Err(e) => Err(e),
            Ok(_) => self.placement.can_found(),
        }
    }

    pub fn found(&mut self, manmade : Manmade) -> Result<(), &str> {
        match self.can_found() {
            Err(e) => Err(e),
            Ok(_) => self.placement.found(manmade),
        }
    }

    pub fn can_sow(&self) -> Result<(), &str> {
        match self.terrian.can_sow() {
            Err(e) => Err(e),
            Ok(_) => self.placement.can_sow(),
        }
    }

    pub fn sow(&mut self) -> Result<(), &str> {
         match self.can_sow() {
            Err(e) => Err(e),
            Ok(_) => self.placement.sow(),
        }
    }

    pub fn can_build(&self) -> Result<(), &str> {
        self.placement.can_build()
    }

    pub fn build(&mut self) -> Result<bool, &str> {
        self.placement.build()
    }

    pub fn can_produce_food(&self) -> bool {
        self.placement.can_produce_food()
    }

    pub fn can_pick_food(&self) -> bool {
        self.placement.can_produce_food() && self.supply 
    }

    pub fn can_pick_wood(&self) -> bool {
        self.placement.can_produce_wood() && self.supply 
    }

    pub fn can_pick(&self) -> bool {
        self.can_pick_food() || self.can_pick_wood()
    }

    pub fn get_supply(&self) -> bool {
        self.supply
    }

    pub fn refresh(&mut self) {
        self.supply = true;
    }

    pub fn consume(&mut self) -> Result<(), &str> {
        if self.supply {
            self.supply = false;
            Ok(())
        }else{
            Err("Can not consume a no supply tile")
        }
    }

    pub(super) fn set_terrian(&mut self, terrian: Terrian) {
        self.terrian = terrian;
    }

    pub(super) fn set_placement(&mut self, placement: Placement) {
        self.placement = placement;
    }
}
