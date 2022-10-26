pub mod entity;
use crate::constant::*;
use crate::error::CtrlErr;
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

    pub fn is_sawmill(&self) -> bool {
        self.placement.is_sawmill()
    }

    pub fn get_supply(&self) -> bool {
        self.supply
    }

    pub fn refresh(&mut self) {
        self.supply = true;
    }

    pub fn consume(&mut self) -> Result<(), CtrlErr> {
        if self.supply {
            self.supply = false;
            Ok(())
        } else {
            Err(CtrlErr::Consumed)
        }
    }

    pub fn can_step(&self) -> Result<(), CtrlErr> {
        self.terrian.can_step()
    }

    pub fn mvcost(&self) -> Result<f64, CtrlErr> {
        self.can_step()?;
        Ok(self.terrian.mvcost().unwrap()+self.placement.mvcost().unwrap())
    }

    pub fn can_found(&self) -> Result<(), CtrlErr> {
        self.terrian.can_found().and(
            self.placement.can_found()
        )
    }

    pub fn found(&mut self, manmade: Manmade) -> Result<(), CtrlErr> {
        self.can_found()?;
        self.placement.found(manmade)
    }

    pub fn can_sow(&self) -> Result<(), CtrlErr> {
        self.terrian.can_sow().and(
            self.placement.can_sow()
        )
    }

    pub fn sow(&mut self) -> Result<(), CtrlErr> {
        self.can_sow()?;
        self.placement.sow()
    }

    pub fn can_build(&self) -> Result<(), CtrlErr> {
        self.placement.can_build()
    }

    pub fn build(&mut self) -> Result<bool, CtrlErr> {
        self.placement.build()
    }

    pub fn can_produce_food(&self) -> bool {
        self.placement.can_produce_food()
    }

    pub fn can_pick(&self) -> Result<Resource, CtrlErr> {
        if let Some(r) = self.placement.produce() {
            if self.supply {
                Ok(r)
            }else{
                Err(CtrlErr::Consumed)
            }
        }else{
            Err(CtrlErr::WrongPlacement(self.placement.clone()))
        }
    }

    pub fn pick(&mut self) -> Result<Resource, CtrlErr> {
        let r = self.can_pick()?;
        self.consume()?;
        Ok(r)
    }

    pub fn can_saw(&self) -> Result<(), CtrlErr> {
        if self.is_sawmill() {
            Ok(())
        }else{
            Err(CtrlErr::WrongPlacement(self.placement.clone()))
        }
    }

    pub fn can_give_sow(&self) -> Result<(), CtrlErr> {
        if self.is_hovel() {
            Ok(())
        }else{
            Err(CtrlErr::WrongPlacement(self.placement.clone()))
        }
    }
}
