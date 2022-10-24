pub mod entity;
use crate::constant::*;
use entity::{Manmade, Natural, Placement, Terrian, Resource};

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

    pub(super) fn set_terrian(&mut self, terrian: Terrian) {
        self.terrian = terrian;
    }

    pub(super) fn set_placement(&mut self, placement: Placement) {
        self.placement = placement;
    }

    pub fn is_hovel(&self) -> bool {
        self.placement.is_hovel()
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
        } else {
            Err("Can not consume a no supply tile")
        }
    }



    pub fn can_step(&self) -> Result<(), &str> {
        self.terrian.can_step()
    }

    pub fn mvcost(&self) -> Result<f64, &str> {
        match self.can_step() {
            Err(s) => Err(s),
            Ok(_) => Ok(self.terrian.mvcost().expect("") + self.placement.mvcost().expect("")),
        }
    }

    pub fn can_found(&self) -> Result<(), &str> {
        match self.terrian.can_found() {
            Err(e) => Err(e),
            Ok(_) => self.placement.can_found(),
        }
    }

    pub fn found(&mut self, manmade: Manmade) -> Result<(), &str> {
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

    pub fn can_pick(&self) -> Result<Resource, &str> {
        match self.placement.produce() {
            None => Err("No resource can pick in this tile."),
            Some(r) => {
                if self.supply {
                    Ok(r)
                }else{
                    Err("This tile has been picked this turn.")
                }
            }
        }
    }

    pub fn pick(&mut self) -> Result<Resource, &str> {
        match self.can_pick() {
            Err(s) => Err(s),
            Ok(_) => {
                match self.placement.produce() {
                    None => Err("No resource can pick in this tile."),
                    Some(r) => {
                        match self.consume() {
                            Err(s1) => Err(s1),
                            Ok(_) => Ok(r),
                        }
                    },
                }
            }
        }
    }
}
