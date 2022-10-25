pub mod entity;
use crate::constant::*;
use entity::{Manmade, Natural, Placement, Resource, Terrian};

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

    pub fn consume(&mut self) -> Result<(), &'static str> {
        if self.supply {
            self.supply = false;
            Ok(())
        } else {
            Err("Can not consume a no supply tile")
        }
    }

    pub fn can_step(&self) -> Result<(), &'static str> {
        self.terrian.can_step()
    }

    pub fn mvcost(&self) -> Result<f64, &'static str> {
        self.can_step()?;
        Ok(self.terrian.mvcost().unwrap()+self.placement.mvcost().unwrap())
    }

    pub fn can_found(&self) -> Result<(), &'static str> {
        self.terrian.can_found().and(
            self.placement.can_found()
        )
    }

    pub fn found(&mut self, manmade: Manmade) -> Result<(), &'static str> {
        self.can_found()?;
        self.placement.found(manmade)
    }

    pub fn can_sow(&self) -> Result<(), &'static str> {
        self.terrian.can_sow().and(
            self.placement.can_sow()
        )
    }

    pub fn sow(&mut self) -> Result<(), &'static str> {
        self.can_sow()?;
        self.placement.sow()
    }

    pub fn can_build(&self) -> Result<(), &'static str> {
        self.placement.can_build()
    }

    pub fn build(&mut self) -> Result<bool, &'static str> {
        self.placement.build()
    }

    pub fn can_produce_food(&self) -> bool {
        self.placement.can_produce_food()
    }

    pub fn can_pick(&self) -> Result<Resource, &'static str> {
        if let Some(r) = self.placement.produce() {
            if self.supply {
                Ok(r)
            }else{
                Err("This tile has been picked this turn.")
            }
        }else{
            Err("No resource can pick in this tile.")
        }
    }

    pub fn pick(&mut self) -> Result<Resource, &'static str> {
        let r = self.can_pick()?;
        self.consume()?;
        Ok(r)
    }
}
