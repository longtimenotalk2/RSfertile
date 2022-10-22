

use crate::constant::*;
use super::Tile;

#[derive(Clone)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

impl Terrian {
    pub fn mvcost(&self) -> f64 {
        match self {
            Terrian::Plain => MVCOST_PLAIN,
            Terrian::Hill => MVCOST_HILL,
            Terrian::Sea => panic!("Can not get Sea's mvcost"),
        }
    }

    pub fn can_step(&self) -> Result<(), &str> {
        match self {
            Terrian::Sea => Err("Can not step on the Sea tile."),
            _ => Ok(()),
        }
    }

    pub fn can_found(&self) -> Result<(), &str> {
        match self {
            Terrian::Sea => Err("Can not found on the Sea tile."),
            _ => Ok(()),
        }
    }

    pub fn can_sow(&self) -> Result<(), &str> {
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
    pub fn mvcost(&self) -> f64 {
        match self {
            Natural::Tree => MVCOST_TREE,
            _ => 0.,
        }
    }
}

#[derive(Clone)]
pub enum Manmade {
    Hovel,
}

#[derive(Clone)]
pub enum Placement {
    Void,
    Landform(Natural),
    Building(Manmade),
    Foundation(Manmade, i64),
}

impl Placement {
    pub fn mvcost(&self) -> f64 {
        match self {
            Placement::Landform(n) => n.mvcost(),
            _ => 0.
        }
    }

    pub fn can_found(&self) -> Result<(), &str> {
        match self {
            Placement::Void => Ok(()),
            Placement::Landform(natural) => match natural {
                Natural::Tree => Err("Can not found on a tile with Tree."),
                Natural::Farm => Ok(()),
            }
            Placement::Building(_) => Err("Can not found on a Building."),
            Placement::Foundation(..) => Err("Can not found on a Foundation.")
        }
    }

    pub fn can_sow(&self) -> Result<(), &str> {
        match self {
            Placement::Void => Ok(()),
            Placement::Landform(_) => Err("Can sow sow on a tile with Natural"),
            Placement::Building(_) => Err("Can not sow on a Building."),
            Placement::Foundation(..) => Err("Can not sow on a Foundation.")
        }
    }
}




        


