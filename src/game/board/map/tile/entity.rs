use super::Tile;
use crate::constant::*;

#[derive(Clone)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

impl Terrian {
    pub(super) fn mvcost(&self) -> Result<f64, &'static str> {
        match self {
            Terrian::Plain => Ok(MVCOST_PLAIN),
            Terrian::Hill => Ok(MVCOST_HILL),
            Terrian::Sea => Err("Can not get Sea's mvcost"),
        }
    }

    pub(super) fn can_step(&self) -> Result<(), &'static str> {
        match self {
            Terrian::Sea => Err("Can not step on the Sea tile."),
            _ => Ok(()),
        }
    }

    pub(super) fn can_found(&self) -> Result<(), &'static str> {
        match self {
            Terrian::Sea => Err("Can not found on the Sea tile."),
            _ => Ok(()),
        }
    }

    pub(super) fn can_sow(&self) -> Result<(), &'static str> {
        match self {
            Terrian::Sea => Err("Can not sow on the Sea tile."),
            _ => Ok(()),
        }
    }
}

#[derive(Clone)]
pub enum Natural {
    Tree,
    Farm,
}

impl Natural {
    pub(super) fn mvcost(&self) -> Result<f64, &'static str> {
        match self {
            Natural::Tree => Ok(MVCOST_TREE),
            _ => Ok(0.),
        }
    }
}

#[derive(Clone)]
pub enum Resource {
    Food,
    Wood,
}

#[derive(Clone)]
pub enum Manmade {
    Hovel,
}

impl Manmade {
    pub(super) fn max_process(&self) -> i64 {
        match self {
            Manmade::Hovel => PROCESS_HOVEL
        }
    }
}

#[derive(Clone)]
pub enum Placement {
    Void,
    Landform(Natural),
    Building(Manmade),
    Foundation(Manmade, i64),
}

impl Placement {
    pub(super) fn is_hovel(&self) -> bool {
        match self {
            Placement::Building(m) => {
                match m {
                    Manmade::Hovel => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    fn remained_process(&self) -> i64 {
        match self {
            Placement::Foundation(m, p) => {
                if *p < 0 {
                    panic!("Should not has minus process")
                }
                let rp = m.max_process() - p;
                if rp < 0 {
                    panic!("Should not get minus remained process")
                }
                rp
            },
            _ => panic!("Should not cal remained process on Non Foundation")
        }
    }

    pub fn get_process(&self) -> Option<i64> {
        match self {
            Placement::Foundation(m, p) => Some(*p),
            _ => None
        }
    }
    
    pub(super) fn mvcost(&self) -> Result<f64, &'static str> {
        match self {
            Placement::Landform(n) => n.mvcost(),
            _ => Ok(0.),
        }
    }

    pub(super) fn can_found(&self) -> Result<(), &'static str> {
        match self {
            Placement::Void => Ok(()),
            Placement::Landform(natural) => match natural {
                Natural::Tree => Err("Can not found on a tile with Tree."),
                Natural::Farm => Ok(()),
            },
            Placement::Building(_) => Err("Can not found on a Building."),
            Placement::Foundation(..) => Err("Can not found on a Foundation."),
        }
    }

    pub(super) fn found(&mut self, manmade : Manmade) -> Result<(), &'static str> {
        match self.can_found() {
            Err(s) => Err(s),
            Ok(()) => {
                *self = Placement::Foundation(manmade, 0);
                Ok(())
            }
        }
    }

    pub(super) fn can_sow(&self) -> Result<(), &'static str> {
        match self {
            Placement::Void => Ok(()),
            Placement::Landform(_) => Err("Can sow sow on a tile with Natural"),
            Placement::Building(_) => Err("Can not sow on a Building."),
            Placement::Foundation(..) => Err("Can not sow on a Foundation."),
        }
    }

    pub fn sow(&mut self) -> Result<(), &'static str> {
        match self.can_sow() {
            Err(s) => Err(s),
            Ok(()) => {
                *self = Placement::Landform(Natural::Farm);
                Ok(())
            }
        }
    }

    pub(super) fn can_build(&self) -> Result<(), &'static str> {
        match self {
            Placement::Foundation(..) => Ok(()),
            _ => Err("Can only build on Foundation"),
        }
    }

    pub(super) fn build(&mut self) -> Result<bool, &'static str> {
        match self.can_build() {
            Err(s) => Err(s),
            Ok(_) => {
                match self.clone() {
                    Placement::Foundation(m, p) => {
                        *self = Placement::Foundation(m.clone(), p+1);
                        if self.remained_process() == 0 {
                            *self = Placement::Building(m);
                            Ok(true)
                        }else{
                            Ok(false)
                        }
                    },
                    _ => panic!("Should not build on Non Foundation")
                }
            }
        }
    }

    pub(super) fn produce(&self) -> Option<Resource> {
        match self {
            Placement::Landform(n)=> {
                match n {
                    Natural::Farm => Some(Resource::Food),
                    Natural::Tree => Some(Resource::Wood),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    pub(super) fn can_produce_food(&self) -> bool {
        match self.produce() {
            None => false,
            Some(r) => match r {
                Resource::Food => true,
                _ => false,
            }
        }
    }

    pub(super) fn set_natural(&mut self, natural : Natural) {
        *self = Placement::Landform(natural);
    }

    
}
