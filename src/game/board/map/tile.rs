pub mod entity;
use entity::{Terrian, Placement};
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

    pub fn can_sow(&self) -> Result<(), &str> {
        match self.terrian.can_sow() {
            Err(e) => Err(e),
            Ok(_) => self.placement.can_sow(),
        }
    }

    pub fn can_build(&self) -> bool {
        match self.get_building() {
            Building::Foundation(_) => true,
            _ => false,
        }
    }

    pub fn remained_mount(&self) -> i64 {
        self.get_building().remained_mount()
    }

    pub fn can_pick_food(&self) -> bool {
        if self.can_step() && self.supply {
            match self.get_building() {
                Building::Farm => true,
                _ => false,
            }
        }else{
            false
        }
    }

    pub fn can_pick_wood(&self) -> bool {
        if self.can_step() && self.supply {
            match self.get_landform() {
                Landform::Tree => true,
                _ => false,
            }
        }else{
            false
        }
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

    pub fn consume(&mut self) {
        if self.supply {
            self.supply = false;
        }else{
            panic!("can not consume a no supply tile")
        }
    }

    pub fn set_terrian(&mut self, terrian: Terrian) {
        self.terrian = terrian;
    }

    pub fn set_landform(&mut self, landform: Landform) {
        self.landform = landform;
    }

    fn set_building(&mut self, building: Building) {
        self.building = building;
    }

    pub fn sow(&mut self) {
        if self.can_sow(){
            self.set_building(Building::Farm);
        }else{
            panic!("Can not sow in this tile");
        }
    }

    pub fn found(&mut self, building : Building) {
       if self.can_found() {
           self.set_building(
               match building {
                Building::Foundation(_) => panic!("Can not found a foundation"),
               Building::Farm => panic!("Can not found a farm"),
               Building::Void => panic!("Can not found void"),
               other => Building::Foundation(Box::new((other, 0)))
               }
            )
       }else{
           panic!("Can not found in this tile")
       }
    }

    pub fn build(&mut self, mount : i64) -> bool {
        if self.can_build() && mount <= self.remained_mount() {
            self.building.build(mount)
        }else{
            panic!("Can not build on this tile or mount over remained")
        }
    }
}
